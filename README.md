# Backend

Backend HTTP da plataforma MD Stack. Este serviço expõe uma API analítica sobre competições de programação, instituições, times, organizadores e eventos, usando Rust, Axum e PostgreSQL.

Este README **não é uma documentação de endpoints**. O objetivo aqui é ajudar futuros desenvolvedores a entenderem:

- o papel do projeto dentro da stack
- a arquitetura interna
- como as camadas conversam entre si
- como ler o projeto usando a lente de `MVC + Service Layer`
- por que algumas decisões foram tomadas
- como evoluir o código sem precisar “adivinhar o padrão”

## Visão Geral

O projeto é, conceitualmente, um backend de leitura e agregação de dados.

Ele não segue o estilo clássico de MVC com um `Model` monolítico no padrão Active Record. Em vez disso, a arquitetura está mais próxima de:

- `Routes + Controllers`
- `Service Layer`
- `Repository Layer`
- `DTOs`
- `SQL explícito com sqlx`

Em outras palavras: a lógica de aplicação fica nos `services`, o acesso ao banco fica nos `repositories`, e os contratos HTTP ficam nos `dtos`.

## Lendo o Projeto como MVC + Service Layer

Para quem vem de Rails, Laravel, Spring MVC, ASP.NET MVC ou qualquer stack parecida, a forma mais útil de entender este backend é: **ele continua sendo compatível com a lógica de MVC, mas com uma Service Layer explícita e com o “Model” dividido em mais de uma peça**.

### O que significa MVC aqui

No MVC clássico:

- `Controller` recebe a requisição
- `Model` representa dados e regras de negócio
- `View` renderiza a resposta

Em uma aplicação HTML tradicional, a `View` costuma ser template. Em uma API JSON, a `View` normalmente não é uma página HTML: ela passa a ser a **representação serializada da resposta**, isto é, os objetos que serão transformados em JSON.

### O que significa Service Layer aqui

A `Service Layer` é uma camada intermediária entre controller e modelo/persistência. Ela existe para concentrar:

- casos de uso
- validação de entrada da aplicação
- orquestração entre múltiplas fontes de dados
- transformação e agregação de resultados

Em arquiteturas com Service Layer, a ideia é evitar dois extremos ruins:

- controllers “gordos”, com regra demais
- models “gordos”, misturando persistência, regra de negócio e formatação de resposta

### Mapa de equivalência deste projeto

A equivalência mais útil é esta:

| Conceito | Neste projeto | Observação |
| --- | --- | --- |
| `Controller` | `controllers/` | Recebe `Path`, `Query`, `State` e delega |
| `View` | `dtos/*/responses` + serialização JSON do Axum | Em API, a “view” é o payload JSON |
| `Model` | `repositories/`, `repositories/types/`, `shared/types` e parte da lógica de domínio usada pelos `services` | O “model” não está concentrado em uma única pasta |
| `Service Layer` | `services/` | Camada explícita de caso de uso |
| Infra de entrada HTTP | `routes/` | Faz o wiring entre path e controller |

### O que é “Model” neste backend

Este é o ponto mais importante para evitar confusão.

O projeto **não tem uma pasta `models/`**, mas isso não significa que ele “não tem model” no sentido arquitetural. O que aconteceu foi uma divisão de responsabilidades que, em outras stacks, ficariam todas juntas dentro do model.

Aqui, o papel de `Model` foi fatiado em quatro partes:

- `repositories/`: como os dados são buscados
- `repositories/types/`: em que formato cru a query devolve os dados
- `shared/types/`: tipos de domínio compartilhados, como enums
- `services/`: parte da regra de aplicação que combina e reorganiza os dados

Então a leitura correta não é “este projeto não tem model”. A leitura correta é:

- o `Model` existe como responsabilidade arquitetural
- mas ele não existe como uma camada única e monolítica

### Onde fica a View em uma API

Como este projeto não renderiza HTML, a `View` aparece de outra forma:

- `dtos/<dominio>/responses` definem a estrutura do payload externo
- o Axum serializa esses DTOs em JSON

Por isso, ao pensar em MVC aqui, vale usar a expressão **View Model** ou **Response DTO** em vez de “template”.

### Resumo curto

Se você quiser guardar um único mapa mental, use este:

```text
Route -> Controller -> Service -> Repository -> Banco
                         |
                         -> Response DTO -> JSON
```

ou, na linguagem de MVC + Service Layer:

```text
HTTP wiring -> Controller -> Service Layer -> Model/Persistence -> View(JSON)
```

## Stack Técnica

- Rust 2024
- Axum para HTTP
- Tokio para runtime assíncrono
- SQLx para acesso a PostgreSQL
- Serde para serialização e desserialização
- Chrono para datas
- Mockall para mocks em testes unitários
- Docker para build e execução do serviço

## Como Rodar

### Rodando pela stack completa

A forma mais simples de subir o projeto é a partir da raiz de `md-stack`:

```bash
docker compose up --build
```

Nesse caso:

- o PostgreSQL sobe via Compose
- o backend recebe `DATABASE_URL` via `.env`
- o frontend/proxy passa a consumir este serviço

### Rodando apenas o backend

Pré-requisitos:

- PostgreSQL disponível
- variável `DATABASE_URL` configurada

Exemplo:

```bash
export DATABASE_URL=postgres://user:password@localhost:5432/maratona_db
cargo run
```

### O que acontece no startup

O fluxo de inicialização está em `src/main.rs`:

1. lê `DATABASE_URL`
2. cria um `PgPool`
3. executa as migrations embutidas com `sqlx::migrate!()`
4. monta `AppState`
5. cria o router Axum
6. sobe o servidor em `0.0.0.0:8000`

Isso significa que o schema do banco é aplicado automaticamente no boot.

## Comandos Úteis

```bash
cargo check
cargo test
cargo run
```

Se quiser validar o backend sem escrever artefatos em `target/` do projeto, é possível apontar o target para outro diretório:

```bash
CARGO_TARGET_DIR=/tmp/backend-target cargo test
```

## Estrutura de Pastas

```text
src/
  controllers/
  dtos/
  repositories/
  routes/
  services/
  shared/
  errors.rs
  lib.rs
  main.rs
  state.rs
migrations/
Dockerfile
README.md
```

### O papel de cada pasta

- `routes/`: registra os endpoints HTTP por domínio
- `controllers/`: extrai `Path`, `Query` e `State`, chama o service e devolve resposta HTTP
- `services/`: valida entrada, orquestra chamadas aos repositórios e transforma dados
- `repositories/`: contratos e implementação de acesso a dados
- `repositories/types/`: tipos de linha retornados pelas queries SQL
- `dtos/`: contratos de entrada e saída da API
- `shared/`: enums, tipos compartilhados e utilitários de serialização
- `migrations/`: schema, tipos SQL, funções auxiliares e seed de dados

## Arquitetura em Camadas

O fluxo principal do projeto é:

```text
HTTP Request
  -> route
  -> controller
  -> service
  -> repository trait
  -> SQL query
  -> repository row types
  -> service aggregation / mapping
  -> response DTO
  -> JSON Response
```

Se você preferir ler o mesmo fluxo em termos de MVC + Service Layer:

```text
HTTP Request
  -> HTTP wiring
  -> Controller
  -> Service Layer
  -> Model/Persistence
  -> View(JSON)
```

### Exemplo mental

Uma requisição para buscar estruturas de competições segue esta lógica:

1. a rota registra o endpoint em `routes/competitions.rs`
2. o controller extrai `competition_ids` da query string
3. o service valida que os IDs existem
4. o service chama `repo.find_structures_by_ids(...)`
5. o repository executa uma query SQL grande e denormalizada
6. a query retorna várias linhas “achatadas”
7. o service reagrupa essas linhas em competições, eventos e times
8. o controller devolve `Json(Vec<CompetitionStructure>)`

## Camada por Camada

### 1. Routes

**Na lente MVC + Service Layer:** esta camada fica um passo antes do `Controller`. Ela é infraestrutura HTTP, não regra de negócio.

As rotas apenas organizam os endpoints por domínio.

Exemplo:

- `routes/competitions.rs`
- `routes/teams.rs`
- `routes/institutions.rs`

Responsabilidade desta camada:

- declarar paths
- ligar path -> controller
- não conter regra de negócio

A agregação final acontece em `routes::create_router()`, que faz o merge das routes criadas em cada submódulo num objeto `axum::Router`. Na `main`, os endpoints declarados aqui são associados ao `TcpListener` pelo `axum::serve()`.

### 2. Controllers

**Na lente MVC + Service Layer:** esta é a camada `Controller` propriamente dita.

Os controllers são deliberadamente finos.

Exemplo simplificado de um controller:

```rust
pub async fn get_structures(
    State(state): State<AppState>,
    Query(filter): Query<StructuresQuery>,
) -> impl IntoResponse {
    services::competitions::get_structures(&state.repo, filter.competition_ids.into_inner())
        .await
        .map(|structures| Json(structures))
}
```

Responsabilidade desta camada:

- extrair parâmetros HTTP
- chamar o service correto
- converter o resultado em `Json(...)`
- deixar o Axum transformar `AppError` em resposta HTTP

O controller **não deveria**:

- montar SQL
- fazer agrupamento de dados
- conter regra analítica significativa

### 3. Services

**Na lente MVC + Service Layer:** esta é a própria `Service Layer`, explícita e central.

Os `services` são o coração da aplicação.

Eles fazem duas coisas principais:

- validação de entrada
- transformação/orquestração dos dados retornados pelos repositórios

Exemplos de responsabilidades típicas:

- exigir `year`, `start_year`, `end_year` ou IDs obrigatórios
- chamar um método de repositório
- transformar rows de banco em DTOs de resposta
- reagrupar dados achatados em estruturas hierárquicas

Em um MVC sem Service Layer, parte dessa lógica poderia acabar em controllers ou models. Aqui ela fica intencionalmente isolada em `services/`.

### 4. Repositories

**Na lente MVC + Service Layer:** esta é a parte de persistência do `Model`.

A camada de repositório é dividida em duas partes:

- um **trait** por domínio, que define o contrato
- uma implementação concreta baseada em `Registry`, que possui o `PgPool`

Exemplo conceitual:

```rust
#[async_trait]
pub trait CompetitionRepository: Send + Sync {
    async fn find_structures_by_ids(&self, ids: Vec<i32>) -> AppResult<Vec<CompetitionStructureRow>>;
}
```

E depois:

```rust
#[async_trait]
impl CompetitionRepository for Registry {
    async fn find_structures_by_ids(&self, ids: Vec<i32>) -> AppResult<Vec<CompetitionStructureRow>> {
        structures::find_structures_by_ids(self, ids).await
    }
}
```

Ou seja:

- o `trait` define “o que o service precisa”
- o `Registry` define “como isso é executado com SQL real”

### 5. Repository Types

**Na lente MVC + Service Layer:** estes tipos ainda fazem parte do lado de `Model`, mas especificamente como **query models** ou **read models internos**.

A pasta `repositories/types/` contém structs como `CompetitionStructureRow`, `TeamStructureRow`, `EventPerformanceRow`, etc.

Esses tipos **não são entidades de domínio**. Eles representam a forma exata da projeção SQL retornada por `query_as`.

Isso é importante porque este backend faz muitas queries analíticas e agregadas. Em vez de mapear tabela por tabela como um ORM faria, as queries retornam recortes específicos do banco já prontos para uso na aplicação.

### 6. DTOs

**Na lente MVC + Service Layer:**

- `requests` ficam do lado da borda HTTP
- `responses` fazem o papel mais próximo de `View` em uma API JSON

Os `dtos` definem o contrato HTTP.

A divisão atual é:

- `dtos/<dominio>/requests`: entrada HTTP
- `dtos/<dominio>/responses`: saída HTTP
- `dtos/common`: tipos compartilhados entre domínios

Exemplos:

- `YearQuery`
- `StructuresQuery`
- `CompetitionStructure`
- `EventPerformance`
- `OptionItem`

Regra prática:

- se o tipo existe para ler a requisição ou serializar a resposta, ele deve estar em `dtos`
- se o tipo existe para receber resultado cru do SQL, ele deve estar em `repositories/types`

### 7. Shared

**Na lente MVC + Service Layer:** esta camada contém tipos transversais de domínio e infraestrutura leve, compartilhados entre várias partes do “Model” e da borda HTTP.

A pasta `shared/` concentra tipos e utilitários reutilizados em todo o projeto.

Dois pontos importantes:

- `shared/types.rs`: enums como `GenderCategory`, `LocationType`, `Scope`
- `shared/serde.rs`: desserializadores customizados, como `CsvOptVec<T>`

`CsvOptVec<T>` permite aceitar filtros como:

```text
?competition_ids=1,2,3
```

em vez de exigir arrays JSON ou múltiplos parâmetros repetidos.

### 8. Errors

**Na lente MVC + Service Layer:** esta é uma peça transversal. Ela atravessa controller, service e repository, mas o mapeamento final para HTTP acontece na borda.

`errors.rs` define:

- `AppError`
- `AppResult<T>`
- a conversão de erro para resposta HTTP com `IntoResponse`

Hoje a aplicação trabalha basicamente com:

- `BadRequest`
- `Database`

Isso mantém a assinatura dos services simples:

```rust
pub type AppResult<T> = Result<T, AppError>;
```

## Conceitos de Rust Importantes para Entender Este Projeto

Esta seção existe para quem vem de linguagens como Ruby, JavaScript, Python ou Java.

### Trait = algo próximo de “interface”

Um `trait` em Rust define um contrato: um conjunto de métodos que um tipo pode implementar.

Neste projeto, os services não dependem do tipo concreto `Registry`. Eles dependem de traits como:

- `CompetitionRepository`
- `TeamRepository`
- `InstitutionRepository`

Isso permite trocar a implementação sem alterar o service.

### `&dyn CompetitionRepository` = trait object

Quando um service recebe algo assim:

```rust
repo: &dyn CompetitionRepository
```

isso significa: “receba uma referência para qualquer valor que implemente `CompetitionRepository`”.

Para quem vem de OO, pense como algo parecido com:

- uma interface em Java/C#
- um objeto que responde a um protocolo específico

#### Por que isso foi escolhido aqui?

Porque isso desacopla a regra de negócio da infraestrutura.

O service não precisa saber:

- se a implementação usa PostgreSQL
- se usa SQLx
- se é um mock de teste

Ele só precisa saber que existe um método como `find_structures_by_ids(...)`.

#### Benefício prático

Nos testes unitários, o projeto usa `mockall` para gerar mocks automaticamente a partir dos traits.

Então, em vez de testar com banco real, dá para fazer:

- mockar o repositório
- devolver rows sintéticas
- validar apenas a transformação do service

### `Send + Sync`

Os traits de repositório usam `Send + Sync` porque o Axum/Tokio roda em ambiente assíncrono e potencialmente multi-thread.

Regra prática:

- `Send`: o valor pode ser movido entre threads
- `Sync`: referências para o valor podem ser compartilhadas entre threads

Na maioria das vezes, pense nisso como uma exigência de segurança de concorrência.

### `async_trait`

Rust ainda trata traits assíncronos com algumas limitações de ergonomia. Para permitir `async fn` dentro de traits, o projeto usa a macro `async_trait`.

Sem ela, os signatures de trait ficariam bem mais verbosas.

### `From` e conversões explícitas

Vários DTOs implementam `From<RowType>` ou `From<TempType>`.

Exemplo mental:

```rust
.map(CompetitionStructure::from)
```

Isso é uma forma explícita e idiomática de dizer:

- “pegue este tipo interno”
- “converta para o tipo que será exposto pela API”

### `fold`, `map` e `collect`: o pipeline funcional dos services

Muitos services seguem um pipeline parecido com este:

```rust
repo.find_structures_by_ids(ids)
    .await?
    .into_iter()
    .fold(IndexMap::new(), |mut acc, row| {
        // agrupa e reorganiza os dados
        acc
    })
    .into_values()
    .map(CompetitionStructure::from)
    .collect()
```

Para quem não está acostumado:

- `into_iter()`: começa a iterar pelos itens
- `fold(...)`: reduz vários itens em uma estrutura acumuladora
- `map(...)`: transforma item por item
- `collect()`: materializa o resultado final em `Vec<_>`

Isso é muito usado aqui porque as queries retornam linhas achatadas, mas a API precisa devolver árvores como:

- competição -> eventos -> times
- instituição -> competições -> eventos -> times
- organizador -> competições -> eventos

### Por que `IndexMap` e não `HashMap`?

`IndexMap` preserva a ordem de inserção.

Isso ajuda em dois pontos:

- JSON sai com ordem estável e previsível
- testes ficam menos frágeis, porque a ordem do resultado não muda aleatoriamente

## Modelo de Dados do Banco

O schema está em `migrations/0001_create_schema.sql`.

Algumas entidades centrais:

- `organizer`: entidade que organiza competições
- `competition`: competição lógica
- `event`: tipo de evento dentro de uma competição
- `event_instance`: ocorrência concreta de um evento em uma data/local
- `institution`: universidade/escola
- `team`: time ligado a uma instituição
- `team_event`: participação de um time em um `event_instance`
- `member`: pessoa
- `team_event_member`: membros ligados a uma participação do time
- `submission`: submissões em problemas
- `location`: árvore hierárquica de localizações

### Detalhe importante: `event` vs `event_instance`

Esse é um ponto importante para quem vai manter o projeto.

- `event` representa o tipo lógico do evento
- `event_instance` representa uma edição concreta em uma data/local

Então “Regional” pode existir como evento lógico, mas ter várias instâncias ao longo dos anos.

### Localizações hierárquicas

O banco define uma função `get_location_tree(start_location_id)`.

Ela é usada em várias queries para:

- montar strings de localização como `Country, State, City`
- calcular recortes agregados por `LocationType`
- descobrir quais níveis geográficos fazem sentido para um conjunto de times/eventos

## Por Que Não Há Uma Camada `models/`?

Em termos de `MVC + Service Layer`, a resposta curta é: **há “model” como responsabilidade arquitetural, mas não como uma pasta única chamada `models/`**.

Este projeto não usa “model” no sentido tradicional de Rails/Active Record, isto é, uma entidade que ao mesmo tempo:

- conhece o banco
- executa queries
- contém regra de negócio
- sabe virar resposta externa

Em vez disso, essas responsabilidades foram repartidas entre:

- `repositories`: acesso a dados
- `repositories/types`: formatos crus das queries
- `shared`: tipos de domínio compartilhados
- `services`: regra de aplicação e transformação
- `dtos`: contrato externo da API

Isso foi uma decisão consciente.

Como este backend é muito orientado a consulta analítica e agregação, um modelo ORM clássico teria menos benefício do que SQL explícito com projeções específicas.

## Estratégia de Testes

Os testes hoje estão concentrados principalmente na camada de service.

Isso conversa diretamente com a arquitetura `MVC + Service Layer`: a camada mais valiosa para testar isoladamente é justamente a que contém os casos de uso e a transformação dos dados.

Em vez de testar transformação com banco real, os tests:

1. mockam o trait de repositório
2. devolvem rows sintéticas
3. validam o comportamento do service

Exemplo de benefícios:

- testes rápidos
- sem dependência de banco
- foco total na regra de aplicação
- falhas mais fáceis de localizar

### Como os mocks funcionam

Os traits usam:

```rust
#[cfg_attr(test, mockall::automock)]
```

Isso faz com que `mockall` gere tipos como:

- `MockCompetitionRepository`
- `MockTeamRepository`
- `MockInstitutionRepository`

Assim, o service pode ser testado isoladamente.

## Convenções de Implementação

### Controllers finos

Se um controller começar a validar regra, agregar dados ou decidir estrutura de domínio, provavelmente a lógica está na camada errada.

### Services como orquestradores

Se uma regra envolve:

- validação de input
- combinação de dados
- transformação entre formatos
- agrupamento de rows em hierarquias

isso deve morar em `services/`.

### SQL explícito nos repositories

Se algo precisa de banco, o lugar certo é `repositories/`.

Não coloque SQL em controller, DTO ou service.

### DTO de resposta não é row de banco

Não exponha diretamente `repositories/types/*` para a API.

Esses tipos existem para refletir a query SQL, não o contrato HTTP.

### Pense em `responses` como a View da API

Se você estiver em dúvida sobre o papel de `dtos/*/responses`, pense neles como a camada de apresentação da API: eles são o formato final que o cliente enxerga.

### `shared` para tipos realmente compartilhados

Enums e utilitários transversais devem ir para `shared/`. Se o tipo existe apenas para um endpoint ou domínio, provavelmente não deve ficar lá.

## Como Adicionar Uma Nova Feature

Fluxo recomendado para adicionar um novo endpoint/caso de uso:

1. definir o contrato HTTP em `dtos/<dominio>/requests` e/ou `responses`
2. adicionar a rota em `routes/<dominio>.rs`
3. criar o controller em `controllers/<dominio>/...`
4. implementar o caso de uso em `services/<dominio>/...`
5. declarar o método necessário no trait de repositório do domínio
6. criar o row type em `repositories/types/...` se necessário
7. implementar a query SQL no módulo correto de repository
8. escrever testes unitários do service com `mockall`

### Regra prática

Se você não sabe onde colocar algo, faça a seguinte pergunta:

- isso é HTTP? -> `controller` ou `dto`
- isso é regra de aplicação? -> `service`
- isso é SQL/banco? -> `repository`
- isso é formato cru de query? -> `repositories/types`
- isso é apresentação da resposta? -> `dtos/*/responses`

## Trade-offs da Arquitetura Atual

### Vantagens

- SQL explícito e fácil de otimizar
- services testáveis sem banco
- controllers simples
- separação clara entre contrato HTTP e projeção SQL
- boa adequação para endpoints analíticos
- leitura arquitetural limpa para quem pensa em `MVC + Service Layer`

### Custos

- mais arquivos e mais “cerimônia” do que frameworks com ORM pesado
- queries podem ficar grandes e exigir disciplina de organização
- exige familiaridade com traits, async e transformação funcional
- o “Model” fica espalhado em mais de uma camada, o que exige onboarding melhor

## Pontos de Atenção para Futuras Evoluções

Algumas melhorias naturais para o futuro:

- adicionar testes de integração para controller + banco
- revisar padronização de mensagens de erro
- adicionar observabilidade estruturada (logs, tracing, métricas)
- revisar endpoints e documentação pública da API separadamente deste README
- considerar separar melhor alguns `Temp*` internos de agregação se a camada de DTO crescer muito
- se o domínio ficar mais rico, avaliar a introdução de entidades de domínio explícitas sem abandonar a Service Layer

## Resumo Mental do Projeto

Se você lembrar de apenas uma coisa, lembre desta:

- `routes` fazem o wiring HTTP
- `controllers` fazem o papel de `Controller`
- `services` são a `Service Layer`
- `repositories` e `repositories/types` representam a parte persistente do `Model`
- `dtos/*/responses` são a `View` da API

Esse é o eixo central da manutenção do backend.
