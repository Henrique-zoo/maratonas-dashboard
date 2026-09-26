//! # `cucumber` — alvo de integração BDD
//!
//! ## Responsabilidade
//! Executa os cenários Gherkin contra o router real e um PostgreSQL descartável,
//! centralizando estado, preparação do banco, isolamento e encerramento da suíte.
//!
//! ## Lógica de Implementação
//! Um container PostgreSQL 16 atende toda a execução. Migrations e fixture são
//! aplicadas na base-modelo; cenários consultivos compartilham uma cópia configurada
//! para leitura e cenários com `@isolated_database` recebem clones graváveis.
//! Os hooks associam e liberam os recursos de cada `ApiWorld`. Requisições usam
//! `oneshot` em memória, enquanto os steps de constraints acessam SQL diretamente.
//! Até quatro cenários executam simultaneamente; clonagens do template são serializadas.
//!
//! ## Submódulos
//! - `step_definitions`: traduções dos passos Gherkin para ações e verificações.
//!
//! ## Funções
//! - `main`: prepara a suíte, executa os hooks e verifica as estatísticas após a limpeza.
//! - `connect_pool`: abre pools no PostgreSQL administrado pela suíte.
//! - `has_isolated_database_tag`: resolve a tag nos níveis feature, regra e cenário.
//!
//! ## Tipos
//! - `ApiWorld`: estado de requisições, banco e seleções de um cenário.
//! - `DatabaseServer` e `SuiteDatabase`: container e recursos compartilhados.
//! - `TestDatabase`, `TestResponse` e `TestDatabaseMutation`: resultados e recursos locais.
//! - `HomeFilterContext`: simulação dos filtros a partir dos dados reais da API.
//!
//! ## Execução
//! Execute `cargo test --locked --test cucumber` no diretório `backend`, com
//! Docker disponível. O runner usa `tests/features` e não lê `DATABASE_URL`.
//! Veja `backend/README.md`, na seção de estratégia de testes, para o ciclo de vida
//! e as convenções de extensão da suíte.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use backend::{AppState, routes};
use cucumber::{World as _, writer::Stats as _};
use futures::{FutureExt as _, lock::Mutex};
use serde::{Deserialize, de::DeserializeOwned};
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};
use tower::ServiceExt as _;

mod step_definitions;

/// Base-modelo preparada uma vez com migrations e fixture para clonagem.
const POSTGRES_TEMPLATE_DB: &str = "md_stack_cucumber_template";
/// Cópia da base-modelo usada pelos cenários consultivos.
const POSTGRES_SHARED_DB: &str = "md_stack_cucumber_shared";
/// Usuário criado exclusivamente no PostgreSQL descartável da suíte.
const POSTGRES_USER: &str = "md_stack";
/// Senha de teste usada apenas no container descartável.
const POSTGRES_PASSWORD: &str = "md_stack";
/// Porta interna do PostgreSQL; a porta publicada no host é dinâmica.
const POSTGRES_PORT: u16 = 5432;
/// Nome da tag Gherkin que solicita uma base independente e gravável.
const ISOLATED_DATABASE_TAG: &str = "isolated_database";
/// Prefixo dos nomes gerados para as bases isoladas e reconhecidos na limpeza.
const SCENARIO_DATABASE_PREFIX: &str = "md_stack_cucumber_scenario_";

/// Contexto criado pelo Cucumber para cada cenário.
///
/// O `Default` começa sem banco ou router; o hook `before` os associa conforme
/// a tag de isolamento. Respostas, mutações e seleções pertencem ao cenário,
/// mesmo quando os handles de leitura são compartilhados entre Worlds.
#[derive(Default, cucumber::World)]
pub(crate) struct ApiWorld {
    /// Banco associado pelo hook `before` e liberado pelo hook `after`.
    database: Option<TestDatabase>,
    /// Router real com o estado associado ao pool deste cenário.
    app: Option<Router>,
    /// Resultados das operações concorrentes, na ordem definida pelo step.
    pub(crate) concurrent_database_mutations: Vec<TestDatabaseMutation>,
    /// Resultado da última tentativa de mutação sequencial.
    pub(crate) last_database_mutation: Option<TestDatabaseMutation>,
    /// Última resposta HTTP capturada pelos helpers de requisição.
    pub(crate) last_response: Option<TestResponse>,
    /// Total de requisições feitas por este World, incluindo a carga inicial dos filtros.
    pub(crate) request_count: usize,
    /// Contexto local que simula a seleção dos filtros da página inicial.
    pub(crate) home: HomeFilterContext,
}

/// Associação do cenário a um pool compartilhado ou a uma base isolada com nome próprio.
struct TestDatabase {
    /// Pool usado pelo router e pelos steps SQL do cenário.
    pool: PgPool,
    /// Nome da base isolada; `None` identifica o pool compartilhado.
    name: Option<String>,
}

impl TestDatabase {
    /// Identifica o modo de uso do banco para a saída de diagnóstico do World.
    ///
    /// # Retorno
    /// `isolated` quando existe um nome de banco próprio; `shared-read-only` no modo compartilhado.
    fn kind(&self) -> &'static str {
        if self.name.is_some() {
            "isolated"
        } else {
            "shared-read-only"
        }
    }
}

/// Dono do container PostgreSQL e dos recursos compartilhados durante a execução da suíte.
struct DatabaseServer {
    /// Handle que mantém o PostgreSQL descartável vivo durante a suíte.
    container: ContainerAsync<GenericImage>,
    /// Pools, router e sincronização compartilhados pelos hooks.
    suite: Arc<SuiteDatabase>,
}

/// Recursos reutilizados pelos hooks de preparação e limpeza.
///
/// O contador fornece nomes únicos e o mutex serializa clonagens da base-modelo.
/// Clonar o `Arc` permite compartilhar esses recursos entre cenários concorrentes.
struct SuiteDatabase {
    /// Endereço do PostgreSQL fornecido pelo Testcontainers.
    host: String,
    /// Porta do PostgreSQL publicada dinamicamente no host.
    port: u16,
    /// Pool conectado à base `postgres` para criar e remover bancos.
    admin_pool: PgPool,
    /// Pool de leitura reutilizado pelos cenários sem a tag de isolamento.
    shared_pool: PgPool,
    /// Router associado ao pool compartilhado.
    shared_app: Router,
    /// Contador atômico usado para gerar nomes de bases isoladas.
    next_database: AtomicU64,
    /// Mutex que serializa clonagens concorrentes da base-modelo.
    template_ddl: Mutex<()>,
}

/// Status e corpo UTF-8 da última resposta produzida pelo router real.
#[derive(Debug)]
pub(crate) struct TestResponse {
    /// Código HTTP retornado pelo router.
    pub(crate) status: StatusCode,
    /// Corpo completo da resposta convertido para UTF-8.
    pub(crate) body: String,
}

/// Resultado resumido de uma mutação para validar constraints e concorrência.
///
/// Preserva SQLSTATE e constraint quando SQLx os disponibiliza, permitindo que
/// uma rejeição esperada seja validada por um step `Then`.
#[derive(Debug)]
pub(crate) struct TestDatabaseMutation {
    /// Indica se a operação ou transação retornou sucesso.
    pub(crate) succeeded: bool,
    /// SQLSTATE informado pelo PostgreSQL, quando disponível.
    pub(crate) code: Option<String>,
    /// Nome da constraint informada pelo PostgreSQL, quando disponível.
    pub(crate) constraint: Option<String>,
}

impl TestDatabaseMutation {
    /// Registra o resultado de uma mutação sem transformar uma rejeição esperada em panic.
    ///
    /// # Parâmetros
    /// - `result`: resultado SQLx da operação ou do commit da transação.
    ///
    /// # Retorno
    /// Indicação de sucesso e, quando fornecidos pelo PostgreSQL, SQLSTATE e nome
    /// da constraint. Erros de conexão também indicam falha, mas podem não conter
    /// código ou constraint. O valor de sucesso é descartado.
    pub(crate) fn from_result<T>(result: Result<T, sqlx::Error>) -> Self {
        match result {
            Ok(_) => Self {
                succeeded: true,
                code: None,
                constraint: None,
            },
            Err(error) => {
                let database_error = error.as_database_error();

                Self {
                    succeeded: false,
                    code: database_error
                        .and_then(|error| error.code())
                        .map(|code| code.into_owned()),
                    constraint: database_error
                        .and_then(|error| error.constraint())
                        .map(str::to_owned),
                }
            }
        }
    }
}

/// Projeção mínima de ID e nome desserializada dos endpoints de opções.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct EntityOption {
    /// Identificador da entidade.
    pub(crate) id: i32,
    /// Nome exibido na opção.
    pub(crate) name: String,
}

/// Referência a uma competição presente no portfólio de uma instituição ou equipe.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct StructureCompetition {
    /// Identificador da competição no portfólio.
    pub(crate) id: i32,
}

/// Projeção de uma competição e seus eventos usada na cascata de filtros.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CompetitionStructure {
    /// Identificador da competição.
    pub(crate) id: i32,
    /// Eventos retornados na estrutura da competição.
    pub(crate) events: Vec<CompetitionEvent>,
}

/// Projeção das equipes de um evento usada para calcular opções e resumos.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CompetitionEvent {
    /// Equipes retornadas para o evento.
    pub(crate) teams: Vec<CompetitionTeam>,
}

/// Identidade mínima da equipe e nomes institucionais usados na correspondência de filtros.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CompetitionTeam {
    /// Identificador usado para deduplicar a equipe.
    pub(crate) id: i32,
    /// Nome completo da instituição da equipe.
    pub(crate) institution_name: String,
    /// Sigla institucional, quando fornecida pela API.
    pub(crate) institution_short_name: Option<String>,
}

/// Portfólio institucional mínimo usado para localizar competições e equipes.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct InstitutionStructure {
    /// Identificador da instituição.
    pub(crate) id: i32,
    /// Nome completo usado na correspondência com equipes.
    pub(crate) name: String,
    /// Sigla alternativa usada na correspondência com equipes.
    pub(crate) short_name: Option<String>,
    /// Competições presentes no portfólio institucional.
    pub(crate) competitions: Vec<StructureCompetition>,
}

/// Portfólio mínimo de competições de uma equipe, independente do caminho usado para selecioná-la.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct TeamStructure {
    /// Identificador da equipe.
    pub(crate) id: i32,
    /// Competições presentes no portfólio da equipe.
    pub(crate) competitions: Vec<StructureCompetition>,
}

/// Contagens de competições, eventos e equipes calculadas sobre as estruturas carregadas.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct FilterSummary {
    /// Número de competições incluídas no recorte.
    pub(crate) competitions: usize,
    /// Número de eventos incluídos no recorte.
    pub(crate) events: usize,
    /// Número de equipes distintas por ID.
    pub(crate) teams: usize,
}

/// Modelo em memória da seleção de filtros usado pelos steps de navegação.
///
/// Os dados vêm de requisições reais à API, mas seleção, endereço e resumo são
/// simulados em Rust. Esta estrutura não executa JavaScript nem valida o browser.
#[derive(Clone, Debug)]
pub(crate) struct HomeFilterContext {
    /// Endereço simulado, atualizado somente ao aplicar os filtros.
    pub(crate) current_path: String,
    /// Contador capturado antes da última seleção para detectar consultas adicionais.
    pub(crate) request_count_before_last_interaction: usize,
    /// Catálogo de organizadores carregado pela API.
    organizer_options: Vec<EntityOption>,
    /// Catálogo completo de competições carregado pela API.
    competition_options: Vec<EntityOption>,
    /// Catálogo completo de instituições carregado pela API.
    institution_options: Vec<EntityOption>,
    /// Catálogo completo de equipes carregado pela API.
    team_options: Vec<EntityOption>,
    /// Opções de competição consultadas para cada organizador.
    competition_options_by_organizer: BTreeMap<i32, Vec<EntityOption>>,
    /// Estruturas usadas para resolver equipes e contar eventos.
    competition_structures: Vec<CompetitionStructure>,
    /// Portfólios institucionais usados na cascata e no resumo.
    institution_structures: Vec<InstitutionStructure>,
    /// Portfólios de equipes usados no resumo entre competições.
    team_structures: Vec<TeamStructure>,
    /// Organizador escolhido na primeira etapa da cascata.
    selected_organizer: Option<EntityOption>,
    /// Competição escolhida entre as opções do organizador.
    selected_competition: Option<EntityOption>,
    /// Instituição escolhida entre as participantes da competição.
    selected_institution: Option<EntityOption>,
    /// Equipe escolhida entre as opções da instituição e competição.
    selected_team: Option<EntityOption>,
}

impl Default for HomeFilterContext {
    /// Inicializa o contexto da página raiz com coleções vazias e nenhum filtro selecionado.
    fn default() -> Self {
        Self {
            current_path: "/".to_owned(),
            request_count_before_last_interaction: 0,
            organizer_options: Vec::new(),
            competition_options: Vec::new(),
            institution_options: Vec::new(),
            team_options: Vec::new(),
            competition_options_by_organizer: BTreeMap::new(),
            competition_structures: Vec::new(),
            institution_structures: Vec::new(),
            team_structures: Vec::new(),
            selected_organizer: None,
            selected_competition: None,
            selected_institution: None,
            selected_team: None,
        }
    }
}

impl fmt::Debug for ApiWorld {
    /// Exibe o modo de banco, as respostas e o contexto do cenário sem imprimir pools ou container.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiWorld")
            .field(
                "database_kind",
                &self.database.as_ref().map(TestDatabase::kind),
            )
            .field("app_ready", &self.app.is_some())
            .field(
                "concurrent_database_mutations",
                &self.concurrent_database_mutations,
            )
            .field("last_database_mutation", &self.last_database_mutation)
            .field("last_response", &self.last_response)
            .field("request_count", &self.request_count)
            .field("home", &self.home)
            .finish()
    }
}

impl ApiWorld {
    /// Obtém o pool associado ao cenário pelo hook `before`.
    ///
    /// # Retorno
    /// Referência ao pool compartilhado de leitura ou ao pool isolado do cenário.
    ///
    /// # Erros
    /// Dispara panic se o hook ainda não associou um banco ao World.
    pub(crate) fn database_pool(&self) -> &PgPool {
        &self
            .database
            .as_ref()
            .expect("BDD database should be initialized")
            .pool
    }

    /// Associa o banco e o router reais ao World antes da execução dos steps.
    ///
    /// # Parâmetros
    /// - `suite`: recursos compartilhados e acesso administrativo ao PostgreSQL de testes.
    /// - `isolated`: indica se o cenário precisa de uma cópia gravável da base-modelo.
    ///
    /// # Lógica de Implementação
    /// Cenários de leitura clonam os handles do pool e do router compartilhados.
    /// No modo isolado, gera um nome único, serializa `CREATE DATABASE ... TEMPLATE`
    /// com o mutex da suíte e monta outro router com um pool de até cinco conexões.
    ///
    /// # Erros
    /// Dispara panic em associação duplicada, falha de clonagem ou conexão.
    /// Uma base criada antes de uma falha de conexão pode ser recolhida na limpeza
    /// de órfãos realizada ao final da suíte.
    async fn attach_database(&mut self, suite: &SuiteDatabase, isolated: bool) {
        assert!(
            self.database.is_none() && self.app.is_none(),
            "a Cucumber world should receive its database only once",
        );

        if !isolated {
            self.database = Some(TestDatabase {
                pool: suite.shared_pool.clone(),
                name: None,
            });
            self.app = Some(suite.shared_app.clone());
            return;
        }

        let database_number = suite.next_database.fetch_add(1, Ordering::Relaxed);
        let database_name = format!("{SCENARIO_DATABASE_PREFIX}{database_number}");

        {
            let _template_guard = suite.template_ddl.lock().await;
            sqlx::query(&format!(
                "CREATE DATABASE \"{database_name}\" TEMPLATE \"{POSTGRES_TEMPLATE_DB}\""
            ))
            .execute(&suite.admin_pool)
            .await
            .unwrap_or_else(|error| {
                panic!("Cucumber should clone isolated database {database_name}: {error}")
            });
        }

        let pool = connect_pool(&suite.host, suite.port, &database_name, 5).await;
        self.app = Some(routes::create_router().with_state(AppState::new(pool.clone())));
        self.database = Some(TestDatabase {
            pool,
            name: Some(database_name),
        });
    }

    /// Libera os recursos associados ao World no hook `after`.
    ///
    /// # Parâmetros
    /// - `suite`: acesso administrativo usado para remover bancos isolados.
    ///
    /// # Lógica de Implementação
    /// Descarta o router antes de fechar o pool isolado e remover seu banco.
    /// No modo compartilhado, descarta apenas o handle local do pool; o pool da
    /// suíte permanece aberto para outros cenários. Tolera um World sem banco.
    ///
    /// # Erros
    /// Dispara panic se a remoção do banco isolado falhar.
    async fn detach_database(&mut self, suite: &SuiteDatabase) {
        self.app.take();

        let Some(database) = self.database.take() else {
            return;
        };

        let Some(database_name) = database.name else {
            drop(database.pool);
            return;
        };

        database.pool.close().await;
        suite
            .drop_database(&database_name)
            .await
            .unwrap_or_else(|error| {
                panic!("Cucumber should remove isolated database {database_name}: {error}")
            });
    }

    /// Executa GET no router real e guarda status e corpo para os próximos steps.
    ///
    /// # Parâmetros
    /// - `path`: URI da requisição, incluindo query string quando necessária.
    ///
    /// # Lógica de Implementação
    /// Usa `oneshot` em memória, sem iniciar um servidor HTTP, incrementa o contador
    /// de requisições e substitui `last_response` pela resposta mais recente.
    ///
    /// # Erros
    /// Dispara panic se o router não estiver pronto, a URI for inválida ou o corpo
    /// não puder ser lido e convertido para UTF-8.
    pub(crate) async fn get(&mut self, path: &str) {
        let app = self
            .app
            .as_ref()
            .expect("backend API should be initialized before requests")
            .clone();
        self.request_count += 1;

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(path)
                    .body(Body::empty())
                    .expect("request should be valid"),
            )
            .await
            .expect("request should be handled by backend router");
        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");

        self.last_response = Some(TestResponse {
            status,
            body: String::from_utf8(body.to_vec()).expect("response body should be UTF-8"),
        });
    }

    /// Consulta o router e desserializa uma resposta HTTP 200.
    ///
    /// # Parâmetros
    /// - `path`: URI enviada a [`ApiWorld::get`].
    ///
    /// # Retorno
    /// Corpo convertido para o tipo `T`, preservando também a resposta no World.
    ///
    /// # Erros
    /// Dispara panic nas falhas de `get`, em status diferente de 200 ou em JSON
    /// incompatível com o tipo solicitado.
    pub(crate) async fn get_json<T>(&mut self, path: &str) -> T
    where
        T: DeserializeOwned,
    {
        self.get(path).await;

        let response = self
            .last_response
            .as_ref()
            .expect("a response should have been captured");

        assert_eq!(response.status, StatusCode::OK);

        serde_json::from_str(&response.body).expect("response should match the expected JSON shape")
    }

    /// Carrega opções e estruturas reais da API para simular a cascata de filtros.
    ///
    /// Consulta organizadores, competições, instituições e equipes, as opções de
    /// competição por organizador e os portfólios completos. Substitui `home` por
    /// um contexto sem seleções, mantendo o contador das requisições executadas.
    ///
    /// # Erros
    /// Propaga por panic as falhas de consulta e desserialização de `get_json`.
    pub(crate) async fn load_home_filter_context(&mut self) {
        let organizer_options: Vec<EntityOption> = self.get_json("/organizers/options").await;
        let competition_options: Vec<EntityOption> = self.get_json("/competitions/options").await;
        let institution_options: Vec<EntityOption> = self.get_json("/institutions/options").await;
        let team_options: Vec<EntityOption> = self.get_json("/teams/options").await;
        let mut competition_options_by_organizer = BTreeMap::new();

        for organizer in &organizer_options {
            competition_options_by_organizer.insert(
                organizer.id,
                self.get_json(&format!(
                    "/competitions/options?organizer_ids={}",
                    organizer.id
                ))
                .await,
            );
        }

        let competition_structures: Vec<CompetitionStructure> = self
            .get_json(&format!(
                "/competitions/structures?competition_ids={}",
                option_ids_csv(&competition_options)
            ))
            .await;
        let institution_structures: Vec<InstitutionStructure> = self
            .get_json(&format!(
                "/institutions/structures?institution_ids={}",
                option_ids_csv(&institution_options)
            ))
            .await;
        let team_structures: Vec<TeamStructure> = self
            .get_json(&format!(
                "/teams/structures?team_ids={}",
                option_ids_csv(&team_options)
            ))
            .await;

        self.home = HomeFilterContext::new(
            organizer_options,
            competition_options,
            institution_options,
            team_options,
            competition_options_by_organizer,
            competition_structures,
            institution_structures,
            team_structures,
        );
    }
}

/// Monta os parâmetros de conexão para o PostgreSQL descartável.
///
/// # Parâmetros
/// - `host`: endereço informado pelo Testcontainers.
/// - `port`: porta publicada dinamicamente no host.
/// - `database`: nome da base compartilhada, administrativa, modelo ou isolada.
///
/// # Retorno
/// Opções SQLx com as credenciais fixas exclusivas do container de testes.
/// A configuração não lê `DATABASE_URL` da aplicação.
fn connect_options(host: &str, port: u16, database: &str) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(host)
        .port(port)
        .username(POSTGRES_USER)
        .password(POSTGRES_PASSWORD)
        .database(database)
}

/// Abre um pool no PostgreSQL de testes com o limite de conexões indicado.
///
/// # Parâmetros
/// - `host`, `port`, `database`: destino usado por `connect_options`.
/// - `max_connections`: limite de conexões simultâneas deste pool.
///
/// # Retorno
/// Pool conectado à base solicitada.
///
/// # Erros
/// Dispara panic se a conexão inicial falhar.
async fn connect_pool(host: &str, port: u16, database: &str, max_connections: u32) -> PgPool {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_with(connect_options(host, port, database))
        .await
        .unwrap_or_else(|error| panic!("Cucumber should connect to database {database}: {error}"))
}

impl DatabaseServer {
    /// Inicializa o container, a base-modelo e os recursos de leitura da suíte.
    ///
    /// # Lógica de Implementação
    /// Sobe `postgres:16-alpine` com porta dinâmica, aplica as migrations embutidas
    /// e a fixture `tests/fixtures/api_bdd.sql`. Fecha o pool do modelo, marca a base
    /// como template e cria a cópia compartilhada com `default_transaction_read_only`.
    /// Bloqueia novas conexões ao modelo para permitir sua clonagem nos hooks.
    ///
    /// # Retorno
    /// Dono do container e dos pools administrativo e compartilhado, com o router
    /// de leitura já associado ao estado real da aplicação.
    ///
    /// # Erros
    /// Dispara panic em falhas de Docker, conexão, migrations, fixture ou preparação
    /// das bases. O encerramento explícito é executado por `shutdown` após a suíte.
    async fn start() -> Self {
        let container = GenericImage::new("postgres", "16-alpine")
            .with_exposed_port(POSTGRES_PORT.tcp())
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_DB", POSTGRES_TEMPLATE_DB)
            .with_env_var("POSTGRES_USER", POSTGRES_USER)
            .with_env_var("POSTGRES_PASSWORD", POSTGRES_PASSWORD)
            .start()
            .await
            .expect("PostgreSQL testcontainer should start");

        let host = container
            .get_host()
            .await
            .expect("PostgreSQL testcontainer should expose host")
            .to_string();
        let port = container
            .get_host_port_ipv4(POSTGRES_PORT)
            .await
            .expect("PostgreSQL testcontainer should expose port");
        let template_pool = connect_pool(&host, port, POSTGRES_TEMPLATE_DB, 5).await;

        sqlx::migrate!("./migrations")
            .run(&template_pool)
            .await
            .expect("Cucumber template migrations should run");

        sqlx::raw_sql(include_str!("fixtures/api_bdd.sql"))
            .execute(&template_pool)
            .await
            .expect("the deterministic Cucumber template fixture should load");

        template_pool.close().await;

        let admin_pool = connect_pool(&host, port, "postgres", 8).await;
        sqlx::query(&format!(
            "ALTER DATABASE \"{POSTGRES_TEMPLATE_DB}\" IS_TEMPLATE true"
        ))
        .execute(&admin_pool)
        .await
        .expect("Cucumber database should be marked as a template");
        sqlx::query(&format!(
            "CREATE DATABASE \"{POSTGRES_SHARED_DB}\" TEMPLATE \"{POSTGRES_TEMPLATE_DB}\""
        ))
        .execute(&admin_pool)
        .await
        .expect("Cucumber should create the shared read-only database");
        sqlx::query(&format!(
            "ALTER DATABASE \"{POSTGRES_SHARED_DB}\" SET default_transaction_read_only = on"
        ))
        .execute(&admin_pool)
        .await
        .expect("Cucumber shared database should default to read-only transactions");
        sqlx::query(&format!(
            "ALTER DATABASE \"{POSTGRES_TEMPLATE_DB}\" ALLOW_CONNECTIONS false"
        ))
        .execute(&admin_pool)
        .await
        .expect("Cucumber template database should reject ordinary connections");

        let shared_pool = connect_pool(&host, port, POSTGRES_SHARED_DB, 16).await;
        let shared_app = routes::create_router().with_state(AppState::new(shared_pool.clone()));
        let suite = Arc::new(SuiteDatabase {
            host,
            port,
            admin_pool,
            shared_pool,
            shared_app,
            next_database: AtomicU64::new(1),
            template_ddl: Mutex::new(()),
        });

        Self { container, suite }
    }

    /// Encerra os recursos da suíte após a coleta das estatísticas do Cucumber.
    ///
    /// Tenta remover bases isoladas remanescentes, fecha os pools compartilhado e
    /// administrativo, solicita a parada do container com timeout de cinco segundos
    /// e o remove. Uma falha na parada é registrada em stderr antes da remoção.
    ///
    /// # Erros
    /// Dispara panic se a listagem de órfãos ou a remoção do container falhar.
    /// Falhas individuais ao remover bases órfãs são registradas em stderr.
    async fn shutdown(self) {
        self.suite.remove_orphaned_databases().await;
        self.suite.shared_pool.close().await;
        self.suite.admin_pool.close().await;

        if let Err(error) = self.container.stop_with_timeout(Some(5)).await {
            eprintln!("failed to stop PostgreSQL testcontainer cleanly: {error}");
        }

        self.container
            .rm()
            .await
            .expect("PostgreSQL testcontainer should be removed");
    }
}

impl SuiteDatabase {
    /// Remove uma base isolada, encerrando conexões ainda abertas com `WITH (FORCE)`.
    ///
    /// # Parâmetros
    /// - `database_name`: nome interno gerado pela suíte ou encontrado pela busca de órfãos.
    ///
    /// # Retorno
    /// `Ok(())` quando a base foi removida ou já não existia.
    ///
    /// # Erros
    /// Propaga o erro SQLx da remoção. O nome interpolado deve vir exclusivamente
    /// dos identificadores controlados pela infraestrutura de teste.
    async fn drop_database(&self, database_name: &str) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(
            "DROP DATABASE IF EXISTS \"{database_name}\" WITH (FORCE)"
        ))
        .execute(&self.admin_pool)
        .await?;
        Ok(())
    }

    /// Recolhe bases de cenários que permaneceram no container após os hooks.
    ///
    /// Seleciona somente nomes com o prefixo da suíte seguido de dígitos. Tenta
    /// remover cada base e registra falhas individuais em stderr para continuar
    /// a limpeza das demais.
    ///
    /// # Erros
    /// Dispara panic se a consulta ao catálogo de bancos falhar.
    async fn remove_orphaned_databases(&self) {
        let database_names = sqlx::query_scalar::<_, String>(
            "SELECT datname
             FROM pg_database
             WHERE datname ~ '^md_stack_cucumber_scenario_[0-9]+$'",
        )
        .fetch_all(&self.admin_pool)
        .await
        .expect("Cucumber should list orphaned scenario databases");

        for database_name in database_names {
            if let Err(error) = self.drop_database(&database_name).await {
                eprintln!("failed to remove orphaned database {database_name}: {error}");
            }
        }
    }
}

/// Verifica se o isolamento foi solicitado na feature, regra ou cenário.
///
/// # Parâmetros
/// - `feature`: funcionalidade que contém o cenário.
/// - `rule`: regra Gherkin opcional que agrupa o cenário.
/// - `scenario`: cenário em preparação.
///
/// # Retorno
/// `true` se algum nível contiver `isolated_database`; o parser fornece a tag
/// sem o prefixo `@`.
fn has_isolated_database_tag(
    feature: &cucumber::gherkin::Feature,
    rule: Option<&cucumber::gherkin::Rule>,
    scenario: &cucumber::gherkin::Scenario,
) -> bool {
    feature
        .tags
        .iter()
        .chain(rule.into_iter().flat_map(|rule| rule.tags.iter()))
        .chain(scenario.tags.iter())
        .any(|tag| tag == ISOLATED_DATABASE_TAG)
}

impl HomeFilterContext {
    /// Constrói o contexto dos filtros com dados carregados da API.
    ///
    /// # Parâmetros
    /// - `organizer_options`, `competition_options`, `institution_options`, `team_options`:
    ///   catálogos usados para resolver nomes e IDs selecionados.
    /// - `competition_options_by_organizer`: competições disponíveis por organizador.
    /// - `competition_structures`, `institution_structures`, `team_structures`:
    ///   portfólios usados para calcular opções dependentes e o resumo local.
    ///
    /// # Retorno
    /// Contexto na página raiz, sem seleções e com as coleções recebidas em memória.
    fn new(
        organizer_options: Vec<EntityOption>,
        competition_options: Vec<EntityOption>,
        institution_options: Vec<EntityOption>,
        team_options: Vec<EntityOption>,
        competition_options_by_organizer: BTreeMap<i32, Vec<EntityOption>>,
        competition_structures: Vec<CompetitionStructure>,
        institution_structures: Vec<InstitutionStructure>,
        team_structures: Vec<TeamStructure>,
    ) -> Self {
        Self {
            current_path: "/".to_owned(),
            request_count_before_last_interaction: 0,
            organizer_options,
            competition_options,
            institution_options,
            team_options,
            competition_options_by_organizer,
            competition_structures,
            institution_structures,
            team_structures,
            selected_organizer: None,
            selected_competition: None,
            selected_institution: None,
            selected_team: None,
        }
    }

    /// Retorna ao endereço raiz e limpa todas as seleções, preservando os dados carregados.
    pub(crate) fn open_home(&mut self) {
        self.current_path = "/".to_owned();
        self.selected_organizer = None;
        self.selected_competition = None;
        self.selected_institution = None;
        self.selected_team = None;
    }

    /// Verifica se o catálogo carregado contém uma competição com o nome exato informado.
    pub(crate) fn competition_exists(&self, name: &str) -> bool {
        self.competition_options
            .iter()
            .any(|option| option.name == name)
    }

    /// Verifica se o catálogo carregado contém uma instituição com o nome exato informado.
    pub(crate) fn institution_exists(&self, name: &str) -> bool {
        self.institution_options
            .iter()
            .any(|option| option.name == name)
    }

    /// Verifica se o catálogo carregado contém uma equipe com o nome exato informado.
    pub(crate) fn team_exists(&self, name: &str) -> bool {
        self.team_options.iter().any(|option| option.name == name)
    }

    /// Seleciona um organizador e limpa competição, instituição e equipe.
    ///
    /// # Erros
    /// Dispara panic se o nome não existir no catálogo de organizadores.
    pub(crate) fn select_organizer(&mut self, name: &str) {
        self.selected_organizer = Some(self.find_option(&self.organizer_options, name));
        self.selected_competition = None;
        self.selected_institution = None;
        self.selected_team = None;
    }

    /// Seleciona uma competição disponível para o organizador e limpa os filtros seguintes.
    ///
    /// # Erros
    /// Dispara panic se a competição não pertencer às opções atualmente disponíveis.
    pub(crate) fn select_competition(&mut self, name: &str) {
        let selected = self.find_option(&self.competition_options_for_selected_organizer(), name);
        self.selected_competition = Some(selected);
        self.selected_institution = None;
        self.selected_team = None;
    }

    /// Seleciona uma instituição participante da competição e limpa a equipe selecionada.
    ///
    /// # Erros
    /// Dispara panic se a instituição não pertencer às opções atualmente disponíveis.
    pub(crate) fn select_institution(&mut self, name: &str) {
        let selected = self.find_option(&self.institution_options_for_selected_competition(), name);
        self.selected_institution = Some(selected);
        self.selected_team = None;
    }

    /// Seleciona uma equipe disponível para a instituição e competição atuais.
    ///
    /// # Erros
    /// Dispara panic se a equipe não pertencer às opções atualmente disponíveis.
    pub(crate) fn select_team(&mut self, name: &str) {
        let selected = self.find_option(&self.team_options_for_selected_filters(), name);
        self.selected_team = Some(selected);
    }

    /// Monta o endereço local com os IDs selecionados, na ordem da cascata.
    ///
    /// Atualiza apenas `current_path`; não faz requisições nem navegação de browser.
    /// Sem filtros, mantém `/`.
    pub(crate) fn apply_filters(&mut self) {
        let mut params = Vec::new();

        if let Some(organizer) = &self.selected_organizer {
            params.push(format!("organizer={}", organizer.id));
        }

        if let Some(competition) = &self.selected_competition {
            params.push(format!("competition={}", competition.id));
        }

        if let Some(institution) = &self.selected_institution {
            params.push(format!("institution={}", institution.id));
        }

        if let Some(team) = &self.selected_team {
            params.push(format!("team={}", team.id));
        }

        self.current_path = if params.is_empty() {
            "/".to_owned()
        } else {
            format!("/?{}", params.join("&"))
        };
    }

    /// Retorna os nomes das competições disponíveis para o organizador, em ordem alfabética.
    pub(crate) fn competition_option_names(&self) -> Vec<String> {
        option_names(&self.competition_options_for_selected_organizer())
    }

    /// Retorna os nomes das instituições da competição selecionada, em ordem alfabética.
    pub(crate) fn institution_option_names(&self) -> Vec<String> {
        option_names(&self.institution_options_for_selected_competition())
    }

    /// Retorna os nomes das equipes da competição e instituição selecionadas, em ordem alfabética.
    pub(crate) fn team_option_names(&self) -> Vec<String> {
        option_names(&self.team_options_for_selected_filters())
    }

    /// Calcula o resumo das estruturas carregadas de acordo com os filtros locais.
    ///
    /// # Lógica de Implementação
    /// Sem uma entidade selecionada, organizador e competição restringem o recorte.
    /// Ao selecionar instituição ou equipe, a hierarquia serve para localizar a
    /// entidade e o cálculo considera seu portfólio completo de competições.
    /// Conta eventos do recorte e deduplica equipes pelo ID.
    ///
    /// # Retorno
    /// Totais de competições, eventos e equipes presentes no contexto atual.
    /// Esta projeção em memória apoia os cenários de filtros; não executa o frontend.
    pub(crate) fn summary(&self) -> FilterSummary {
        let selected_organizer = self.selected_organizer.as_ref().map(|option| option.id);
        let selected_competition = self.selected_competition.as_ref().map(|option| option.id);
        let selected_institution = self.selected_institution.as_ref().map(|option| option.id);
        let selected_team = self.selected_team.as_ref().map(|option| option.id);
        let hierarchy_only_locates_entity =
            selected_institution.is_some() || selected_team.is_some();
        let competition_ids_for_organizer =
            selected_organizer.map(|_| id_set(&self.competition_options_for_selected_organizer()));
        let competition_ids_for_institution = selected_institution.map(|_| {
            self.selected_institution_structure()
                .map(|institution| {
                    institution
                        .competitions
                        .iter()
                        .map(|competition| competition.id)
                        .collect::<BTreeSet<_>>()
                })
                .unwrap_or_default()
        });
        let competition_ids_for_team = selected_team.map(|_| {
            self.selected_team_structure()
                .map(|team| {
                    team.competitions
                        .iter()
                        .map(|competition| competition.id)
                        .collect::<BTreeSet<_>>()
                })
                .unwrap_or_default()
        });
        let has_active_filters = selected_organizer.is_some()
            || selected_competition.is_some()
            || selected_institution.is_some()
            || selected_team.is_some();
        let selected_institution_structure = self.selected_institution_structure();
        let mut summary = FilterSummary::default();
        let mut team_ids = BTreeSet::new();

        for competition in &self.competition_structures {
            if !hierarchy_only_locates_entity
                && competition_ids_for_organizer
                    .as_ref()
                    .is_some_and(|ids| !ids.contains(&competition.id))
            {
                continue;
            }

            if !hierarchy_only_locates_entity
                && selected_competition.is_some_and(|id| competition.id != id)
            {
                continue;
            }

            if competition_ids_for_institution
                .as_ref()
                .is_some_and(|ids| !ids.contains(&competition.id))
            {
                continue;
            }

            if competition_ids_for_team
                .as_ref()
                .is_some_and(|ids| !ids.contains(&competition.id))
            {
                continue;
            }

            let mut competition_event_count = 0;

            for event in &competition.events {
                let event_team_ids = event
                    .teams
                    .iter()
                    .filter(|team| {
                        selected_institution_structure
                            .is_none_or(|institution| team_matches_institution(team, institution))
                            && selected_team.is_none_or(|id| team.id == id)
                    })
                    .map(|team| team.id)
                    .collect::<Vec<_>>();

                if (selected_institution.is_some() || selected_team.is_some())
                    && event_team_ids.is_empty()
                {
                    continue;
                }

                competition_event_count += 1;
                team_ids.extend(event_team_ids);
            }

            if has_active_filters && competition_event_count == 0 {
                continue;
            }

            summary.competitions += 1;
            summary.events += competition_event_count;
        }

        summary.teams = team_ids.len();
        summary
    }

    /// Resolve o primeiro item com nome exato no conjunto de opções informado.
    ///
    /// # Retorno
    /// Cópia da opção encontrada.
    ///
    /// # Erros
    /// Dispara panic quando o nome não existe no conjunto informado.
    fn find_option(&self, options: &[EntityOption], name: &str) -> EntityOption {
        options
            .iter()
            .find(|option| option.name == name)
            .cloned()
            .unwrap_or_else(|| panic!("expected option named {name}; options were {options:?}"))
    }

    /// Lista as competições do organizador em ordem alfabética; sem organizador, retorna vazio.
    fn competition_options_for_selected_organizer(&self) -> Vec<EntityOption> {
        let Some(organizer) = &self.selected_organizer else {
            return Vec::new();
        };
        let ids = self
            .competition_options_by_organizer
            .get(&organizer.id)
            .cloned()
            .unwrap_or_default();

        sort_options(ids)
    }

    /// Filtra e ordena instituições com participação na competição; sem competição, retorna vazio.
    fn institution_options_for_selected_competition(&self) -> Vec<EntityOption> {
        let Some(competition) = &self.selected_competition else {
            return Vec::new();
        };
        let ids = self
            .institution_structures
            .iter()
            .filter(|institution| {
                institution
                    .competitions
                    .iter()
                    .any(|item| item.id == competition.id)
            })
            .map(|institution| institution.id)
            .collect::<BTreeSet<_>>();

        self.options_by_ids(&self.institution_options, &ids)
    }

    /// Filtra equipes da competição pela instituição selecionada e ordena por nome.
    ///
    /// Retorna vazio enquanto a competição ou a estrutura da instituição estiverem
    /// ausentes. A correspondência institucional usa `team_matches_institution`.
    fn team_options_for_selected_filters(&self) -> Vec<EntityOption> {
        let (Some(competition), Some(institution)) = (
            &self.selected_competition,
            self.selected_institution_structure(),
        ) else {
            return Vec::new();
        };
        let ids = self
            .competition_structures
            .iter()
            .find(|structure| structure.id == competition.id)
            .map(|structure| {
                structure
                    .events
                    .iter()
                    .flat_map(|event| &event.teams)
                    .filter(|team| team_matches_institution(team, institution))
                    .map(|team| team.id)
                    .collect::<BTreeSet<_>>()
            })
            .unwrap_or_default();

        self.options_by_ids(&self.team_options, &ids)
    }

    /// Localiza por ID o portfólio da instituição selecionada, retornando `None` se ausente.
    fn selected_institution_structure(&self) -> Option<&InstitutionStructure> {
        let selected = self.selected_institution.as_ref()?;
        self.institution_structures
            .iter()
            .find(|institution| institution.id == selected.id)
    }

    /// Localiza por ID o portfólio da equipe selecionada, retornando `None` se ausente.
    fn selected_team_structure(&self) -> Option<&TeamStructure> {
        let selected = self.selected_team.as_ref()?;
        self.team_structures
            .iter()
            .find(|team| team.id == selected.id)
    }

    /// Copia as opções cujos IDs pertencem ao conjunto fornecido e as ordena pelo nome.
    fn options_by_ids(&self, options: &[EntityOption], ids: &BTreeSet<i32>) -> Vec<EntityOption> {
        let mut filtered = options
            .iter()
            .filter(|option| ids.contains(&option.id))
            .cloned()
            .collect::<Vec<_>>();
        filtered.sort_by(|left, right| left.name.cmp(&right.name));
        filtered
    }
}

/// Serializa os IDs na ordem recebida, separados por vírgulas, para filtros da API.
fn option_ids_csv(options: &[EntityOption]) -> String {
    options
        .iter()
        .map(|option| option.id.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

/// Extrai os nomes preservando a ordem das opções recebidas.
fn option_names(options: &[EntityOption]) -> Vec<String> {
    options.iter().map(|option| option.name.clone()).collect()
}

/// Ordena opções pelo nome e devolve o vetor recebido.
fn sort_options(mut options: Vec<EntityOption>) -> Vec<EntityOption> {
    options.sort_by(|left, right| left.name.cmp(&right.name));
    options
}

/// Reúne os IDs das opções em um conjunto ordenado, eliminando repetições.
fn id_set(options: &[EntityOption]) -> BTreeSet<i32> {
    options.iter().map(|option| option.id).collect()
}

/// Compara a instituição da equipe pelo nome completo ou pela sigla disponível.
///
/// As projeções usadas neste contexto não carregam o ID institucional da equipe;
/// a correspondência depende dos nomes e siglas presentes na fixture.
fn team_matches_institution(team: &CompetitionTeam, institution: &InstitutionStructure) -> bool {
    team.institution_name == institution.name
        || institution
            .short_name
            .as_ref()
            .is_some_and(|short_name| team.institution_short_name.as_ref() == Some(short_name))
}

/// Executa a suíte BDD e converte as estatísticas de falha em resultado do processo.
///
/// # Lógica de Implementação
/// Prepara um servidor PostgreSQL para toda a suíte, registra hooks de associação
/// e limpeza dos Worlds e permite até quatro cenários concorrentes. Lê as
/// features em `tests/features`, relativo ao diretório de execução do target.
/// Coleta falhas de steps, steps ignorados, parsing e hooks, encerra o servidor
/// e só então verifica os contadores.
///
/// # Erros
/// Dispara panic se algum contador de falha for diferente de zero, produzindo
/// saída de processo malsucedida. Falhas na preparação ou no encerramento também
/// podem disparar panic; a limpeza explícita pressupõe que o runner retornou.
#[tokio::main]
async fn main() {
    let database_server = DatabaseServer::start().await;
    let before_suite = Arc::clone(&database_server.suite);
    let after_suite = Arc::clone(&database_server.suite);

    let writer = ApiWorld::cucumber()
        .max_concurrent_scenarios(4)
        .before(move |feature, rule, scenario, world| {
            let suite = Arc::clone(&before_suite);
            let isolated = has_isolated_database_tag(feature, rule, scenario);
            async move { world.attach_database(&suite, isolated).await }.boxed_local()
        })
        .after(move |_, _, _, _, world| {
            let suite = Arc::clone(&after_suite);
            async move {
                if let Some(world) = world {
                    world.detach_database(&suite).await;
                }
            }
            .boxed_local()
        })
        .run("tests/features")
        .await;

    let failures = (
        writer.failed_steps(),
        writer.skipped_steps(),
        writer.parsing_errors(),
        writer.hook_errors(),
    );
    database_server.shutdown().await;

    let (failed_steps, skipped_steps, parsing_errors, hook_errors) = failures;
    assert!(
        failed_steps == 0 && skipped_steps == 0 && parsing_errors == 0 && hook_errors == 0,
        "Cucumber failed: {failed_steps} failed step(s), {skipped_steps} skipped step(s), \
         {parsing_errors} parsing error(s), {hook_errors} hook error(s)",
    );
}
