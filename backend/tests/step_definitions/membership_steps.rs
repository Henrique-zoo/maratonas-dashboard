//! # `cucumber::step_definitions::membership_steps`
//!
//! ## Responsabilidade
//! Exercita identidade de equipes, elencos, participações e constraints reais do PostgreSQL.
//!
//! ## Lógica de Implementação
//! Opera diretamente no pool do World, usando cenários com `@isolated_database`.
//! Preparações exigem sucesso; tentativas potencialmente inválidas capturam
//! `TestDatabaseMutation` para verificar SQLSTATE e constraint posteriormente.
//! Operações concorrentes usam transações distintas, barreira e `tokio::join!`;
//! os `Then`s verificam resultados e estado persistido. Estas operações SQL não
//! representam endpoints de escrita da API.
//!
//! ## Funções
//! - `create_team`, `insert_team_event`, `insert_membership`: preparação e mutações SQL.
//! - `insert_*_in_transaction`: mutações sincronizadas para disputas concorrentes.
//! - `try_*`, `concurrently_*`: captura dos resultados no World.
//! - `assert_last_database_constraint` e steps `*_should_*`: verificações do contrato do banco.
//!
//! ## Tipos
//! Reutiliza `ApiWorld`, `TestDatabaseMutation`, pools SQLx e `Barrier`.

use std::sync::Arc;

use cucumber::{given, then, when};
use sqlx::postgres::PgQueryResult;
use tokio::sync::Barrier;

use crate::{ApiWorld, TestDatabaseMutation};

/// Converte uma lista CSV de IDs, removendo espaços ao redor de cada valor.
///
/// # Retorno
/// IDs na ordem recebida, sem deduplicação.
///
/// # Erros
/// Dispara panic se algum elemento não puder ser convertido para `i32`.
fn parse_member_ids(member_ids: &str) -> Vec<i32> {
    member_ids
        .split(',')
        .map(|member_id| {
            member_id
                .trim()
                .parse::<i32>()
                .unwrap_or_else(|error| panic!("invalid member id {member_id:?}: {error}"))
        })
        .collect()
}

/// Cria uma equipe e seu elenco canônico em uma única transação.
///
/// # Parâmetros
/// - `pool`: pool gravável da base isolada do cenário.
/// - `team_id`, `name`, `institution_id`: identidade e vínculo institucional.
/// - `contestant_ids`: membros inseridos em `team_contestant`.
///
/// # Retorno
/// `Ok(())` após o commit da equipe e de todo o elenco.
///
/// # Erros
/// Propaga falhas SQLx da abertura, inserções ou commit, incluindo constraints
/// de identidade avaliadas ao finalizar a transação.
async fn create_team(
    pool: &sqlx::PgPool,
    team_id: i32,
    name: &str,
    institution_id: i32,
    contestant_ids: &[i32],
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query("INSERT INTO team (id, name, institution_id) VALUES ($1, $2, $3)")
        .bind(team_id)
        .bind(name)
        .bind(institution_id)
        .execute(&mut *transaction)
        .await?;

    for contestant_id in contestant_ids {
        sqlx::query("INSERT INTO team_contestant (team_id, member_id) VALUES ($1, $2)")
            .bind(team_id)
            .bind(contestant_id)
            .execute(&mut *transaction)
            .await?;
    }

    transaction.commit().await
}

/// Insere uma participação com campus nulo e posição 999 para testar regras do banco.
///
/// # Parâmetros
/// - `pool`: pool da base isolada.
/// - `team_id`: equipe participante.
/// - `event_instance_id`: ocorrência do evento.
///
/// # Retorno
/// Resultado SQLx da inserção.
///
/// # Erros
/// Propaga erros de conexão, execução e constraints.
async fn insert_team_event(
    pool: &sqlx::PgPool,
    team_id: i32,
    event_instance_id: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "INSERT INTO team_event (team_id, event_instance_id, campus_location_id, rank)
         VALUES ($1, $2, NULL, 999)",
    )
    .bind(team_id)
    .bind(event_instance_id)
    .execute(pool)
    .await
}

/// Insere uma participação em transação após sincronizar operações concorrentes.
///
/// # Parâmetros
/// - `pool`: handle do pool isolado, com capacidade para conexões simultâneas.
/// - `barrier`: barreira compartilhada com a outra operação do teste.
/// - `team_id`, `event_instance_id`: equipe e ocorrência de destino.
///
/// # Retorno
/// `Ok(())` após o commit.
///
/// # Erros
/// Propaga erros SQLx. A barreira é aguardada após a abertura da transação e
/// antes do INSERT, aproximando o início das duas mutações.
async fn insert_team_event_in_transaction(
    pool: sqlx::PgPool,
    barrier: Arc<Barrier>,
    team_id: i32,
    event_instance_id: i32,
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    barrier.wait().await;

    sqlx::query(
        "INSERT INTO team_event (team_id, event_instance_id, campus_location_id, rank)
         VALUES ($1, $2, NULL, 999)",
    )
    .bind(team_id)
    .bind(event_instance_id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await
}

/// Acrescenta um competidor ao elenco em transação sincronizada por barreira.
///
/// # Parâmetros
/// - `pool`: handle do pool isolado.
/// - `barrier`: sincronização com a inscrição concorrente da equipe.
/// - `team_id`, `member_id`: equipe e membro acrescentado ao elenco canônico.
///
/// # Retorno
/// `Ok(())` após o commit.
///
/// # Erros
/// Propaga falhas SQLx, inclusive a rejeição do elenco caso a participação já o
/// tenha tornado imutável.
async fn insert_canonical_contestant_in_transaction(
    pool: sqlx::PgPool,
    barrier: Arc<Barrier>,
    team_id: i32,
    member_id: i32,
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    barrier.wait().await;

    sqlx::query("INSERT INTO team_contestant (team_id, member_id) VALUES ($1, $2)")
        .bind(team_id)
        .bind(member_id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await
}

/// Insere o papel de um membro em uma participação na base isolada do cenário.
///
/// # Parâmetros
/// - `world`: contexto com o pool preparado pelo hook.
/// - `member_id`, `team_event_id`: membro e participação relacionados.
/// - `role`: texto convertido para o enum PostgreSQL `role`.
///
/// # Retorno
/// Resultado SQLx da inserção.
///
/// # Erros
/// Propaga falhas SQLx; acessar um World sem banco dispara panic.
async fn insert_membership(
    world: &mut ApiWorld,
    member_id: i32,
    role: &str,
    team_event_id: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "INSERT INTO team_event_member (member_id, team_event_id, role)
         VALUES ($1, $2, $3::TEXT::role)",
    )
    .bind(member_id)
    .bind(team_event_id)
    .bind(role)
    .execute(world.database_pool())
    .await
}

/// Exige que a última tentativa tenha falhado com SQLSTATE e constraint específicos.
///
/// # Parâmetros
/// - `world`: contexto que deve conter uma mutação capturada.
/// - `code`: SQLSTATE esperado.
/// - `constraint`: nome da constraint esperada.
///
/// # Erros
/// Dispara panic se não houver resultado, se a mutação tiver sucesso ou se o
/// diagnóstico do PostgreSQL divergir do esperado.
fn assert_last_database_constraint(world: &ApiWorld, code: &str, constraint: &str) {
    let mutation = world
        .last_database_mutation
        .as_ref()
        .expect("a database mutation should have been attempted");

    assert!(
        !mutation.succeeded,
        "the database mutation unexpectedly succeeded"
    );
    assert_eq!(mutation.code.as_deref(), Some(code));
    assert_eq!(mutation.constraint.as_deref(), Some(constraint));
}

/// Insere um membro com gênero `Other` como pré-condição; falha de preparação dispara panic.
#[given(expr = "member {int} exists")]
async fn member_exists(world: &mut ApiWorld, member_id: i32) {
    sqlx::query("INSERT INTO member (id, gender) VALUES ($1, 'Other'::gender)")
        .bind(member_id)
        .execute(world.database_pool())
        .await
        .expect("test member should be inserted");
}

/// Prepara uma equipe com elenco canônico via transação e exige sucesso da criação.
#[given(
    regex = r#"^team (\d+) named "([^"]+)" from institution (\d+) has official roster ([\d,\s]+)$"#
)]
async fn team_has_canonical_contestants(
    world: &mut ApiWorld,
    team_id: i32,
    name: String,
    institution_id: i32,
    member_ids: String,
) {
    let contestant_ids = parse_member_ids(&member_ids);
    create_team(
        world.database_pool(),
        team_id,
        &name,
        institution_id,
        &contestant_ids,
    )
    .await
    .unwrap_or_else(|error| panic!("team {team_id} should be created: {error}"));
}

/// Tenta criar equipe e elenco, capturando sucesso ou rejeição para o próximo step.
#[when(
    regex = r#"^I try to create team (\d+) named "([^"]+)" from institution (\d+) with official roster ([\d,\s]+)$"#
)]
async fn try_to_create_team(
    world: &mut ApiWorld,
    team_id: i32,
    name: String,
    institution_id: i32,
    member_ids: String,
) {
    let contestant_ids = parse_member_ids(&member_ids);
    let result = create_team(
        world.database_pool(),
        team_id,
        &name,
        institution_id,
        &contestant_ids,
    )
    .await;

    world.last_database_mutation = Some(TestDatabaseMutation::from_result(result));
}

/// Inscreve a equipe na ocorrência indicada e exige que a preparação tenha sucesso.
#[when(expr = "team {int} participates in event occurrence {int}")]
async fn register_team_in_event_instance(
    world: &mut ApiWorld,
    team_id: i32,
    event_instance_id: i32,
) {
    insert_team_event(world.database_pool(), team_id, event_instance_id)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "team {team_id} should be registered in event instance {event_instance_id}: {error}"
            )
        });
}

/// Tenta inscrever uma equipe e guarda o resultado sem falhar imediatamente por rejeição.
#[when(expr = "I try to register team {int} in event occurrence {int}")]
async fn try_to_register_team_in_event_instance(
    world: &mut ApiWorld,
    team_id: i32,
    event_instance_id: i32,
) {
    let result = insert_team_event(world.database_pool(), team_id, event_instance_id).await;
    world.last_database_mutation = Some(TestDatabaseMutation::from_result(result));
}

/// Tenta incluir um membro no elenco canônico e captura o diagnóstico da mutação.
#[when(expr = "I try to add member {int} to team {int}'s official roster")]
async fn try_to_add_canonical_contestant(world: &mut ApiWorld, member_id: i32, team_id: i32) {
    let result = sqlx::query(
        "INSERT INTO team_contestant (team_id, member_id)
         VALUES ($1, $2)",
    )
    .bind(team_id)
    .bind(member_id)
    .execute(world.database_pool())
    .await;

    world.last_database_mutation = Some(TestDatabaseMutation::from_result(result));
}

/// Tenta alterar nome e instituição de uma equipe e captura o resultado SQLx.
#[when(regex = r#"^I try to rename team (\d+) to \"([^\"]+)\" and move it to institution (\d+)$"#)]
async fn try_to_change_team_attributes(
    world: &mut ApiWorld,
    team_id: i32,
    name: String,
    institution_id: i32,
) {
    let result = sqlx::query(
        "UPDATE team
         SET name = $2, institution_id = $3
         WHERE id = $1",
    )
    .bind(team_id)
    .bind(name)
    .bind(institution_id)
    .execute(world.database_pool())
    .await;

    world.last_database_mutation = Some(TestDatabaseMutation::from_result(result));
}

/// Tenta remover um `Contestant` da participação e guarda o resultado da constraint.
#[when(expr = "I try to remove contestant {int} from participation {int}")]
async fn try_to_remove_event_contestant(world: &mut ApiWorld, member_id: i32, team_event_id: i32) {
    let result = sqlx::query(
        "DELETE FROM team_event_member
         WHERE member_id = $1
           AND team_event_id = $2
           AND role = 'Contestant'::role",
    )
    .bind(member_id)
    .bind(team_event_id)
    .execute(world.database_pool())
    .await;

    world.last_database_mutation = Some(TestDatabaseMutation::from_result(result));
}

/// Insere o membro no papel indicado e exige sucesso da operação.
#[when(regex = r"^member (\d+) serves as (Contestant|Coach|Reserve) in participation (\d+)$")]
async fn register_member(world: &mut ApiWorld, member_id: i32, role: String, team_event_id: i32) {
    insert_membership(world, member_id, &role, team_event_id)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "member {member_id} should be registered as {role} in team event {team_event_id}: {error}"
            )
        });
}

/// Tenta alterar o papel de um membro na participação e captura a aceitação ou rejeição.
#[when(
    regex = r"^I try to change member (\d+) in participation (\d+) to (Contestant|Coach|Reserve)$"
)]
async fn try_to_change_member_role(
    world: &mut ApiWorld,
    member_id: i32,
    team_event_id: i32,
    role: String,
) {
    let result = sqlx::query(
        "UPDATE team_event_member
         SET role = $3::TEXT::role
         WHERE member_id = $1 AND team_event_id = $2",
    )
    .bind(member_id)
    .bind(team_event_id)
    .bind(role)
    .execute(world.database_pool())
    .await;

    world.last_database_mutation = Some(TestDatabaseMutation::from_result(result));
}

/// Executa duas inscrições com transações sincronizadas e preserva seus resultados.
///
/// Uma barreira de duas partes aproxima o início das mutações; `tokio::join!`
/// aguarda ambas. O vetor do World segue a ordem das equipes recebidas, não a
/// ordem de conclusão. Rejeições SQL esperadas ficam disponíveis para os `Then`s.
#[when(
    regex = r"^teams (\d+) and (\d+) are registered simultaneously in event occurrences (\d+) and (\d+)$"
)]
async fn concurrently_register_teams(
    world: &mut ApiWorld,
    first_team_id: i32,
    second_team_id: i32,
    first_event_instance_id: i32,
    second_event_instance_id: i32,
) {
    let barrier = Arc::new(Barrier::new(2));
    let first = insert_team_event_in_transaction(
        world.database_pool().clone(),
        Arc::clone(&barrier),
        first_team_id,
        first_event_instance_id,
    );
    let second = insert_team_event_in_transaction(
        world.database_pool().clone(),
        barrier,
        second_team_id,
        second_event_instance_id,
    );
    let (first_result, second_result) = tokio::join!(first, second);

    world.concurrent_database_mutations = vec![
        TestDatabaseMutation::from_result(first_result),
        TestDatabaseMutation::from_result(second_result),
    ];
}

/// Disputa a inscrição de uma equipe com a inclusão de um membro no elenco.
///
/// Sincroniza as transações por barreira. Guarda primeiro o resultado da inscrição
/// e depois o da alteração do elenco, independentemente da ordem de conclusão.
#[when(
    regex = r"^team (\d+) is registered in event occurrence (\d+) while member (\d+) is simultaneously added to its official roster$"
)]
async fn concurrently_register_team_and_change_roster(
    world: &mut ApiWorld,
    team_id: i32,
    event_instance_id: i32,
    member_id: i32,
) {
    let barrier = Arc::new(Barrier::new(2));
    let registration = insert_team_event_in_transaction(
        world.database_pool().clone(),
        Arc::clone(&barrier),
        team_id,
        event_instance_id,
    );
    let roster_change = insert_canonical_contestant_in_transaction(
        world.database_pool().clone(),
        barrier,
        team_id,
        member_id,
    );
    let (registration_result, roster_change_result) = tokio::join!(registration, roster_change);

    world.concurrent_database_mutations = vec![
        TestDatabaseMutation::from_result(registration_result),
        TestDatabaseMutation::from_result(roster_change_result),
    ];
}

/// Exige SQLSTATE `23505` e a constraint `uq_team_identity` na última mutação.
#[then("the duplicate team identity should be rejected")]
fn duplicate_team_identity_should_be_rejected(world: &mut ApiWorld) {
    assert_last_database_constraint(world, "23505", "uq_team_identity");
}

/// Exige a constraint `ck_team_contestant_roster_immutable` com SQLSTATE `23514`.
#[then("the roster change should be rejected because the team has participated")]
fn canonical_roster_change_should_be_rejected(world: &mut ApiWorld) {
    assert_last_database_constraint(world, "23514", "ck_team_contestant_roster_immutable");
}

/// Exige a constraint `ck_team_identity_immutable` com SQLSTATE `23514`.
#[then("the identity change should be rejected because the team has participated")]
fn participating_team_identity_change_should_be_rejected(world: &mut ApiWorld) {
    assert_last_database_constraint(world, "23514", "ck_team_identity_immutable");
}

/// Exige a constraint `ck_team_event_contestant_roster` com SQLSTATE `23514`.
#[then("the participation roster change should be rejected")]
fn event_roster_change_should_be_rejected(world: &mut ApiWorld) {
    assert_last_database_constraint(world, "23514", "ck_team_event_contestant_roster");
}

/// Exige a unicidade de competidor por competição-ano via `uq_contestant_registration_competition_year`.
#[then(
    "the registration should be rejected because the contestant already represents a team in that competition-year"
)]
fn registration_should_be_rejected(world: &mut ApiWorld) {
    assert_last_database_constraint(
        world,
        "23505",
        "uq_contestant_registration_competition_year",
    );
}

/// Exige a unicidade de competidor por evento-ano via `uq_contestant_event_year`.
#[then(
    "the registration should be rejected because a contestant already entered that event in the year"
)]
fn event_year_registration_should_be_rejected(world: &mut ApiWorld) {
    assert_last_database_constraint(world, "23505", "uq_contestant_event_year");
}

/// Verifica que exatamente uma das mutações concorrentes foi aceita.
#[then("exactly one simultaneous team registration should succeed")]
fn one_concurrent_registration_should_succeed(world: &mut ApiWorld) {
    let succeeded = world
        .concurrent_database_mutations
        .iter()
        .filter(|mutation| mutation.succeeded)
        .count();

    assert_eq!(succeeded, 1, "concurrent mutations were {world:?}");
}

/// Verifica que uma rejeição concorrente corresponde à unicidade por competição-ano.
#[then(
    "the other registration should be rejected because the contestant already represents a team in that competition-year"
)]
fn concurrent_registration_should_be_rejected(world: &mut ApiWorld) {
    let rejected = world
        .concurrent_database_mutations
        .iter()
        .find(|mutation| !mutation.succeeded)
        .expect("one concurrent registration should have been rejected");

    assert_eq!(rejected.code.as_deref(), Some("23505"));
    assert_eq!(
        rejected.constraint.as_deref(),
        Some("uq_contestant_registration_competition_year")
    );
}

/// Exige sucesso no primeiro resultado da disputa, correspondente à inscrição.
#[then("the simultaneous team registration should succeed")]
fn concurrent_team_registration_should_succeed(world: &mut ApiWorld) {
    let registration = world
        .concurrent_database_mutations
        .first()
        .expect("the concurrent team registration should have a result");

    assert!(
        registration.succeeded,
        "concurrent mutations were {world:?}"
    );
}

/// Aceita a inclusão no elenco ou sua rejeição por imutabilidade após a participação.
///
/// Inspeciona o segundo resultado concorrente. A consistência dos elencos
/// persistidos é verificada pelos steps de comparação de elenco.
#[then("the roster change should either precede participation or be rejected as immutable")]
fn concurrent_roster_change_should_be_consistent(world: &mut ApiWorld) {
    let roster_change = world
        .concurrent_database_mutations
        .get(1)
        .expect("the concurrent roster change should have a result");

    if !roster_change.succeeded {
        assert_eq!(roster_change.code.as_deref(), Some("23514"));
        assert_eq!(
            roster_change.constraint.as_deref(),
            Some("ck_team_contestant_roster_immutable")
        );
    }
}

/// Conta equipes pelo mesmo nome e instituição e compara com o total esperado.
#[then(regex = r#"^there should be (\d+) teams named "([^"]+)" from institution (\d+)$"#)]
async fn homonymous_team_count_should_equal(
    world: &mut ApiWorld,
    expected_count: i64,
    name: String,
    institution_id: i32,
) {
    let actual_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM team
         WHERE name = $1 AND institution_id = $2",
    )
    .bind(name)
    .bind(institution_id)
    .fetch_one(world.database_pool())
    .await
    .expect("homonymous teams should be counted");

    assert_eq!(actual_count, expected_count);
}

/// Compara os IDs do elenco canônico com o CSV esperado, ordenando ambos numericamente.
#[then(regex = r"^team (\d+) should have official roster ([\d,\s]+)$")]
async fn team_should_have_canonical_contestants(
    world: &mut ApiWorld,
    team_id: i32,
    member_ids: String,
) {
    let expected = parse_member_ids(&member_ids);
    let actual = sqlx::query_scalar::<_, Vec<i32>>(
        "SELECT ARRAY(
            SELECT member_id
            FROM team_contestant
            WHERE team_id = $1
            ORDER BY member_id
         )",
    )
    .bind(team_id)
    .fetch_one(world.database_pool())
    .await
    .expect("canonical Contestants should be queried");

    let mut expected_sorted = expected;
    expected_sorted.sort_unstable();
    assert_eq!(actual, expected_sorted);
}

/// Compara os `Contestant`s da participação com o elenco canônico da equipe por IDs ordenados.
#[then(expr = "participation {int} should have exactly the official roster of team {int}")]
async fn team_event_should_match_canonical_roster(
    world: &mut ApiWorld,
    team_event_id: i32,
    team_id: i32,
) {
    let rosters_match = sqlx::query_scalar::<_, bool>(
        "SELECT ARRAY(
            SELECT member_id
            FROM team_event_member
            WHERE team_event_id = $1
              AND role = 'Contestant'::role
            ORDER BY member_id
         ) = ARRAY(
            SELECT member_id
            FROM team_contestant
            WHERE team_id = $2
            ORDER BY member_id
         )",
    )
    .bind(team_event_id)
    .bind(team_id)
    .fetch_one(world.database_pool())
    .await
    .expect("event and canonical rosters should be compared");

    assert!(rosters_match);
}

/// Localiza a participação por equipe e ocorrência e compara seus competidores com o elenco canônico.
#[then(expr = "team {int} in event occurrence {int} should have exactly its official roster")]
async fn registered_team_should_match_canonical_roster(
    world: &mut ApiWorld,
    team_id: i32,
    event_instance_id: i32,
) {
    let rosters_match = sqlx::query_scalar::<_, bool>(
        "SELECT ARRAY(
            SELECT tem.member_id
            FROM team_event_member AS tem
            WHERE tem.team_event_id = te.id
              AND tem.role = 'Contestant'::role
            ORDER BY tem.member_id
         ) = ARRAY(
            SELECT tc.member_id
            FROM team_contestant AS tc
            WHERE tc.team_id = te.team_id
            ORDER BY tc.member_id
         )
         FROM team_event AS te
         WHERE te.team_id = $1 AND te.event_instance_id = $2",
    )
    .bind(team_id)
    .bind(event_instance_id)
    .fetch_one(world.database_pool())
    .await
    .expect("registered and canonical rosters should be compared");

    assert!(rosters_match);
}

/// Verifica no banco que o ID de equipe está ausente após a tentativa rejeitada.
#[then(expr = "team {int} should not exist")]
async fn team_should_not_exist(world: &mut ApiWorld, team_id: i32) {
    let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM team WHERE id = $1)")
        .bind(team_id)
        .fetch_one(world.database_pool())
        .await
        .expect("team existence should be queried");

    assert!(!exists);
}

/// Confirma que nome e instituição persistidos correspondem aos valores esperados.
#[then(regex = r#"^team (\d+) should still be named \"([^\"]+)\" at institution (\d+)$"#)]
async fn team_attributes_should_equal(
    world: &mut ApiWorld,
    team_id: i32,
    expected_name: String,
    expected_institution_id: i32,
) {
    let actual = sqlx::query_as::<_, (String, i32)>(
        "SELECT name, institution_id
         FROM team
         WHERE id = $1",
    )
    .bind(team_id)
    .fetch_one(world.database_pool())
    .await
    .expect("team attributes should be queried");

    assert_eq!(actual, (expected_name, expected_institution_id));
}

/// Confirma a ausência do vínculo entre membro e elenco canônico da equipe.
#[then(expr = "member {int} should not belong to team {int}'s official roster")]
async fn member_should_not_be_canonical_contestant(
    world: &mut ApiWorld,
    member_id: i32,
    team_id: i32,
) {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1
            FROM team_contestant
            WHERE team_id = $1 AND member_id = $2
         )",
    )
    .bind(team_id)
    .bind(member_id)
    .fetch_one(world.database_pool())
    .await
    .expect("canonical membership should be queried");

    assert!(!exists);
}

/// Confirma que a equipe não possui participação na ocorrência indicada.
#[then(expr = "team {int} should not participate in event occurrence {int}")]
async fn team_should_not_be_registered(world: &mut ApiWorld, team_id: i32, event_instance_id: i32) {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1
            FROM team_event
            WHERE team_id = $1 AND event_instance_id = $2
         )",
    )
    .bind(team_id)
    .bind(event_instance_id)
    .fetch_one(world.database_pool())
    .await
    .expect("team event registration should be queried");

    assert!(!exists);
}

/// Consulta e compara o papel persistido do membro na participação.
#[then(
    regex = r"^member (\d+) should have role (Contestant|Coach|Reserve) in participation (\d+)$"
)]
async fn member_should_have_role(
    world: &mut ApiWorld,
    member_id: i32,
    role: String,
    team_event_id: i32,
) {
    let actual_role = sqlx::query_scalar::<_, String>(
        "SELECT role::TEXT
         FROM team_event_member
         WHERE member_id = $1 AND team_event_id = $2",
    )
    .bind(member_id)
    .bind(team_event_id)
    .fetch_one(world.database_pool())
    .await
    .expect("membership role should be queried");

    assert_eq!(actual_role, role);
}

/// Conta participações como `Contestant` por membro, equipe, competição e ano da ocorrência.
#[then(
    regex = r"^member (\d+) should have (\d+) contestant participations? for team (\d+) in competition (\d+) in (\d{4})$"
)]
async fn contestant_participation_count_should_equal(
    world: &mut ApiWorld,
    member_id: i32,
    expected_count: i64,
    team_id: i32,
    competition_id: i32,
    year: i32,
) {
    let actual_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM team_event_member AS tem
         JOIN team_event AS te ON te.id = tem.team_event_id
         JOIN event_instance AS ei ON ei.id = te.event_instance_id
         JOIN event AS e ON e.id = ei.event_id
         WHERE tem.member_id = $1
           AND tem.role = 'Contestant'::role
           AND te.team_id = $2
           AND e.competition_id = $3
           AND EXTRACT(YEAR FROM ei.date)::INT = $4",
    )
    .bind(member_id)
    .bind(team_id)
    .bind(competition_id)
    .bind(year)
    .fetch_one(world.database_pool())
    .await
    .expect("contestant participations should be counted");

    assert_eq!(actual_count, expected_count);
}

/// Conta equipes distintas representadas como `Contestant` pelo membro na competição e ano.
#[then(regex = r"^member (\d+) should represent (\d+) teams? in competition (\d+) in (\d{4})$")]
async fn represented_team_count_should_equal(
    world: &mut ApiWorld,
    member_id: i32,
    expected_count: i64,
    competition_id: i32,
    year: i32,
) {
    let actual_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(DISTINCT te.team_id)
         FROM team_event_member AS tem
         JOIN team_event AS te ON te.id = tem.team_event_id
         JOIN event_instance AS ei ON ei.id = te.event_instance_id
         JOIN event AS e ON e.id = ei.event_id
         WHERE tem.member_id = $1
           AND tem.role = 'Contestant'::role
           AND e.competition_id = $2
           AND EXTRACT(YEAR FROM ei.date)::INT = $3",
    )
    .bind(member_id)
    .bind(competition_id)
    .bind(year)
    .fetch_one(world.database_pool())
    .await
    .expect("represented teams should be counted");

    assert_eq!(actual_count, expected_count);
}
