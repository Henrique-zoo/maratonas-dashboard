//! # `cucumber::step_definitions::domain_response_steps`
//!
//! ## Responsabilidade
//! Traduz expectativas de domínio em verificações sobre as respostas JSON reais da API.
//!
//! ## Lógica de Implementação
//! Valida o status HTTP e projeta campos de opções, estatísticas, estruturas,
//! históricos e portfólios em tabelas Gherkin. Comparações preservam a ordem da
//! resposta e validam cabeçalhos explícitos. Os steps de opções por nome usam
//! tabelas de coluna única sem cabeçalho. Helpers exigem campos e tipos presentes;
//! ausências e diferenças geram panic com contexto para o relatório do Cucumber.
//!
//! ## Funções
//! - `response_with_status`, `successful_response`: contrato HTTP e desserialização.
//! - `field`, `value_at` e helpers tipados: extração de campos obrigatórios.
//! - `table_rows`, `assert_rows`: validação de cabeçalhos e comparação ordenada.
//! - Steps `*_should_*`: expectativas de domínio, resultados vazios e erros públicos.
//!
//! ## Tipos
//! Reutiliza `ApiWorld`, `StatusCode`, `Step` e `serde_json::Value`.

use axum::http::StatusCode;
use cucumber::{gherkin::Step, then};
use serde_json::Value;

use crate::ApiWorld;

/// Valida o status da última resposta e desserializa seu corpo JSON.
///
/// # Parâmetros
/// - `world`: contexto com a resposta da última consulta.
/// - `expected_status`: código HTTP exigido pelo step.
///
/// # Retorno
/// Payload JSON validado quanto ao status, ainda sem projeção de domínio.
///
/// # Erros
/// Dispara panic em resposta ausente, status divergente ou JSON inválido,
/// incluindo o corpo no diagnóstico.
fn response_with_status(world: &ApiWorld, expected_status: StatusCode) -> Value {
    let response = world
        .last_response
        .as_ref()
        .expect("a response should have been captured before asserting it");

    assert_eq!(
        response.status, expected_status,
        "unexpected response status; body: {}",
        response.body
    );

    serde_json::from_str(&response.body).unwrap_or_else(|error| {
        panic!(
            "response body should be valid JSON: {error}\nresponse body: {}",
            response.body
        )
    })
}

/// Obtém o JSON da última resposta, exigindo HTTP 200.
fn successful_response(world: &ApiWorld) -> Value {
    response_with_status(world, StatusCode::OK)
}

/// Obtém os elementos de um array JSON; outro tipo dispara panic com o valor recebido.
fn array(value: &Value) -> &[Value] {
    value
        .as_array()
        .unwrap_or_else(|| panic!("expected a JSON array, but received {value}"))
}

/// Obtém um campo obrigatório pelo nome; sua ausência dispara panic com o objeto recebido.
fn field<'a>(value: &'a Value, name: &str) -> &'a Value {
    value
        .get(name)
        .unwrap_or_else(|| panic!("expected field {name:?} in {value}"))
}

/// Lê um campo obrigatório textual e devolve uma cópia; campo ausente ou de outro tipo falha.
fn text_field(value: &Value, name: &str) -> String {
    field(value, name)
        .as_str()
        .unwrap_or_else(|| panic!("field {name:?} should be text in {value}"))
        .to_owned()
}

/// Converte um campo textual, numérico ou booleano em texto para uma tabela.
///
/// # Erros
/// Dispara panic para campo ausente, null, array ou objeto.
fn scalar_field(value: &Value, name: &str) -> String {
    match field(value, name) {
        Value::String(value) => value.clone(),
        Value::Number(value) => value.to_string(),
        Value::Bool(value) => value.to_string(),
        unexpected => panic!("field {name:?} should be scalar, but was {unexpected}"),
    }
}

/// Lê um campo obrigatório como `i64`; ausência ou tipo incompatível dispara panic.
fn integer_field(value: &Value, name: &str) -> i64 {
    field(value, name)
        .as_i64()
        .unwrap_or_else(|| panic!("field {name:?} should be an integer in {value}"))
}

/// Resolve um JSON Pointer padrão no payload e dispara panic quando o caminho não existe.
fn value_at<'a>(value: &'a Value, pointer: &str) -> &'a Value {
    value
        .pointer(pointer)
        .unwrap_or_else(|| panic!("expected JSON pointer {pointer:?} in {value}"))
}

/// Resolve um JSON Pointer e exige que seu valor seja um inteiro `i64`.
fn integer_at(value: &Value, pointer: &str) -> i64 {
    value_at(value, pointer)
        .as_i64()
        .unwrap_or_else(|| panic!("value at {pointer:?} should be an integer in {value}"))
}

/// Resolve um JSON Pointer e devolve uma cópia do texto encontrado.
fn text_at(value: &Value, pointer: &str) -> String {
    value_at(value, pointer)
        .as_str()
        .unwrap_or_else(|| panic!("value at {pointer:?} should be text in {value}"))
        .to_owned()
}

/// Lê as linhas de dados após validar o cabeçalho da tabela Gherkin.
///
/// # Parâmetros
/// - `step`: passo que contém a tabela de domínio.
/// - `expected_headers`: nomes e ordem exatos das colunas esperadas.
///
/// # Retorno
/// Linhas de dados preservadas na ordem recebida, sem a primeira linha de cabeçalho.
///
/// # Erros
/// Dispara panic em tabela ausente, cabeçalho divergente ou quantidade incorreta
/// de células em uma linha.
fn table_rows(step: &Step, expected_headers: &[&str]) -> Vec<Vec<String>> {
    let table = step.table().expect("step should provide a data table");
    let (headers, rows) = table
        .rows
        .split_first()
        .expect("data table should include a header row");
    let expected_headers = expected_headers
        .iter()
        .map(|header| (*header).to_owned())
        .collect::<Vec<_>>();

    assert_eq!(headers, &expected_headers, "unexpected data-table columns");

    for row in rows {
        assert_eq!(
            row.len(),
            headers.len(),
            "each data-table row should have one value per column"
        );
    }

    rows.to_vec()
}

/// Compara a projeção de domínio com a tabela Gherkin, incluindo sua ordem.
///
/// # Parâmetros
/// - `step`: tabela com cabeçalho e valores esperados.
/// - `headers`: contrato de nomes e ordem das colunas.
/// - `actual`: linhas extraídas da resposta, sem cabeçalho.
///
/// # Erros
/// Dispara panic em tabela inválida ou divergência de linhas, valores ou ordem.
fn assert_rows(step: &Step, headers: &[&str], actual: Vec<Vec<String>>) {
    assert_eq!(actual, table_rows(step, headers));
}

/// Exige HTTP 200 e um array vazio na raiz do payload.
fn assert_empty_array(world: &ApiWorld) {
    let payload = successful_response(world);
    assert!(
        array(&payload).is_empty(),
        "expected no results, but received {payload}"
    );
}

/// Exige HTTP 200 e listas vazias em `location_types` e `events` da visão anual.
fn assert_empty_annual_view(world: &ApiWorld) {
    let payload = successful_response(world);
    assert!(
        array(field(&payload, "location_types")).is_empty(),
        "expected no location types in {payload}"
    );
    assert!(
        array(field(&payload, "events")).is_empty(),
        "expected no events in {payload}"
    );
}

/// Obtém a lista obrigatória `years`; não reordena nem converte seus valores.
fn years(value: &Value) -> &[Value] {
    array(field(value, "years"))
}

/// Serializa o primeiro elemento de `years`; uma lista vazia dispara panic.
fn first_year(value: &Value) -> String {
    years(value)
        .first()
        .unwrap_or_else(|| panic!("expected at least one available year in {value}"))
        .to_string()
}

/// Serializa o último elemento de `years`; uma lista vazia dispara panic.
fn last_year(value: &Value) -> String {
    years(value)
        .last()
        .unwrap_or_else(|| panic!("expected at least one available year in {value}"))
        .to_string()
}

/// Concatena os nomes dos itens do array indicado com `; `, preservando a ordem da API.
fn joined_names(value: &Value, field_name: &str) -> String {
    array(field(value, field_name))
        .iter()
        .map(|item| text_field(item, "name"))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Projeta totais de instituições, equipes, participantes e participantes femininas em uma linha.
fn assert_statistics(world: &ApiWorld, step: &Step) {
    let payload = successful_response(world);
    let actual = vec![vec![
        scalar_field(&payload, "total_institutions"),
        scalar_field(&payload, "total_teams"),
        scalar_field(&payload, "total_participants"),
        scalar_field(&payload, "female_participants"),
    ]];

    assert_rows(
        step,
        &[
            "institutions",
            "teams",
            "participants",
            "female participants",
        ],
        actual,
    );
}

/// Compara os totais da competição com a tabela de estatísticas do cenário.
#[then("the competition statistics should be:")]
fn competition_statistics_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    assert_statistics(world, step);
}

/// Compara os totais do evento com a tabela de estatísticas do cenário.
#[then("the event statistics should be:")]
fn event_statistics_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    assert_statistics(world, step);
}

/// Compara países, IDs e totais de instituições, equipes e participantes na ordem da resposta.
#[then("the country statistics should be:")]
fn country_statistics_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(&payload)
        .iter()
        .map(|item| {
            vec![
                scalar_field(item, "id"),
                text_field(item, "name"),
                scalar_field(item, "total_institutions"),
                scalar_field(item, "total_teams"),
                scalar_field(item, "total_participants"),
                scalar_field(item, "female_participants"),
            ]
        })
        .collect();

    assert_rows(
        step,
        &[
            "id",
            "country",
            "institutions",
            "teams",
            "participants",
            "female participants",
        ],
        actual,
    );
}

/// Compara por país os totais de equipes e participantes femininas do evento.
#[then("the event country statistics should be:")]
fn event_country_statistics_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(&payload)
        .iter()
        .map(|item| {
            vec![
                text_field(item, "name"),
                scalar_field(item, "total_teams"),
                scalar_field(item, "female_participants"),
            ]
        })
        .collect();

    assert_rows(step, &["country", "teams", "female participants"], actual);
}

/// Compara eventos, competições e anos disponíveis, mantendo a sequência de anos retornada.
#[then("the available event histories should be:")]
fn available_event_histories_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(&payload)
        .iter()
        .map(|item| {
            vec![
                scalar_field(item, "id"),
                text_field(item, "name"),
                scalar_field(item, "competition_id"),
                text_field(item, "competition_name"),
                years(item)
                    .iter()
                    .map(Value::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
            ]
        })
        .collect();

    assert_rows(
        step,
        &[
            "event id",
            "event",
            "competition id",
            "competition",
            "years",
        ],
        actual,
    );
}

/// Compara por ano a melhor posição, a equipe correspondente e a média de posições.
#[then("the performance history should be:")]
fn performance_history_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(&payload)
        .iter()
        .map(|item| {
            vec![
                scalar_field(item, "year"),
                scalar_field(item, "best_performance_rank"),
                scalar_field(item, "best_performance_team_id"),
                text_field(item, "best_performance_team_name"),
                scalar_field(item, "average_performance_rank"),
            ]
        })
        .collect();

    assert_rows(
        step,
        &[
            "year",
            "best rank",
            "best team id",
            "best team",
            "average rank",
        ],
        actual,
    );
}

/// Exige um array vazio para histórico de evento ou desempenho; ambos usam o mesmo contrato.
#[then(regex = r"^no (event|performance) history should be returned$")]
fn no_history_should_be_returned(world: &mut ApiWorld, _kind: String) {
    assert_empty_array(world);
}

/// Exige um array vazio para uma consulta geográfica sem resultados.
#[then("no location statistics should be returned")]
fn no_location_statistics_should_be_returned(world: &mut ApiWorld) {
    assert_empty_array(world);
}

/// Compara IDs e nomes dos organizadores na ordem retornada.
#[then("the organizer options should be, in order:")]
fn organizer_options_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(&payload)
        .iter()
        .map(|item| vec![scalar_field(item, "id"), text_field(item, "name")])
        .collect();
    assert_rows(step, &["id", "organizer"], actual);
}

/// Compara os nomes das opções com uma tabela de coluna única sem cabeçalho.
fn assert_option_names(world: &ApiWorld, step: &Step) {
    let payload = successful_response(world);
    let actual = array(&payload)
        .iter()
        .map(|item| text_field(item, "name"))
        .collect::<Vec<_>>();
    super::assert_table_values(step, actual);
}

/// Compara a ordem dos nomes das opções de competição.
#[then("the competition options should be, in order:")]
fn competition_options_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    assert_option_names(world, step);
}

/// Compara a ordem dos nomes das opções de instituição.
#[then("the institution options should be, in order:")]
fn institution_options_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    assert_option_names(world, step);
}

/// Compara a ordem dos nomes das opções de equipe.
#[then("the team options should be, in order:")]
fn team_options_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    assert_option_names(world, step);
}

/// Compara IDs e nomes das competições com a tabela de detalhes.
#[then("the competition details should be, in order:")]
fn competition_details_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(&payload)
        .iter()
        .map(|item| vec![scalar_field(item, "id"), text_field(item, "name")])
        .collect();
    assert_rows(step, &["id", "competition"], actual);
}

/// Compara IDs e nomes das equipes e seus identificadores, nomes e siglas institucionais.
#[then("the team details should be, in order:")]
fn team_details_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(&payload)
        .iter()
        .map(|item| {
            vec![
                scalar_field(item, "id"),
                text_field(item, "name"),
                scalar_field(item, "institution_id"),
                text_field(item, "institution_name"),
                text_field(item, "institution_short_name"),
            ]
        })
        .collect();
    assert_rows(
        step,
        &["id", "team", "institution id", "institution", "short name"],
        actual,
    );
}

/// Exige opções vazias para competição, instituição ou equipe, conforme a frase do cenário.
#[then(regex = r"^no (competition|institution|team) options should be returned$")]
fn no_options_should_be_returned(world: &mut ApiWorld, _kind: String) {
    assert_empty_array(world);
}

/// Compara evento, competição, quantidade de anos e primeiro e último anos retornados.
#[then("the event history summary should be:")]
fn event_history_summary_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(&payload)
        .iter()
        .map(|item| {
            vec![
                text_field(item, "name"),
                text_field(item, "competition_name"),
                years(item).len().to_string(),
                first_year(item),
                last_year(item),
            ]
        })
        .collect();
    assert_rows(
        step,
        &[
            "event",
            "competition",
            "year count",
            "first year",
            "last year",
        ],
        actual,
    );
}

/// Exige um único organizador e compara competições, snapshots e anos disponíveis.
#[then("the organizer overview should contain:")]
fn organizer_overview_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let organizers = array(&payload);
    assert_eq!(organizers.len(), 1, "expected one organizer in {payload}");
    let organizer = &organizers[0];
    let actual = array(field(organizer, "competitions"))
        .iter()
        .map(|competition| {
            vec![
                text_field(organizer, "name"),
                text_field(competition, "name"),
                scalar_field(competition, "snapshot_year"),
                years(competition).len().to_string(),
                first_year(competition),
                last_year(competition),
            ]
        })
        .collect();
    assert_rows(
        step,
        &[
            "organizer",
            "competition",
            "snapshot year",
            "year count",
            "first year",
            "last year",
        ],
        actual,
    );
}

/// Localiza a competição por nome na primeira visão de organizador e compara totais de seus eventos.
#[then(regex = r#"^"([^"]+)" should summarize these events:$"#)]
fn competition_should_summarize_events(
    world: &mut ApiWorld,
    competition_name: String,
    #[step] step: &Step,
) {
    let payload = successful_response(world);
    let organizer = array(&payload)
        .first()
        .expect("expected an organizer overview");
    let competition = array(field(organizer, "competitions"))
        .iter()
        .find(|competition| text_field(competition, "name") == competition_name)
        .unwrap_or_else(|| panic!("competition {competition_name:?} was not found in {payload}"));
    let actual = array(field(competition, "events"))
        .iter()
        .map(|event| {
            vec![
                text_field(event, "name"),
                scalar_field(event, "total_participants"),
                scalar_field(event, "female_participants"),
            ]
        })
        .collect();
    assert_rows(
        step,
        &["event", "participants", "female participants"],
        actual,
    );
}

/// Exige um array vazio quando não há visão de organizador para os filtros.
#[then("no organizer overview should be returned")]
fn no_organizer_overview_should_be_returned(world: &mut ApiWorld) {
    assert_empty_array(world);
}

/// Compara nome, data e totais de participantes dos eventos da visão anual.
#[then("the annual event overview should be:")]
fn annual_event_overview_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(field(&payload, "events"))
        .iter()
        .map(|event| {
            vec![
                text_field(event, "name"),
                text_field(event, "date"),
                scalar_field(event, "total_participants"),
                scalar_field(event, "female_participants"),
            ]
        })
        .collect();
    assert_rows(
        step,
        &["event", "date", "participants", "female participants"],
        actual,
    );
}

/// Exige listas vazias de tipos de localização e eventos na visão anual do organizador.
#[then("the annual overview should contain no location types or events")]
fn annual_overview_should_be_empty(world: &mut ApiWorld) {
    assert_empty_annual_view(world);
}

/// Exige uma única competição e compara nome, snapshot e resumo dos anos disponíveis.
#[then("the competition view should describe:")]
fn competition_view_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let competitions = array(&payload);
    assert_eq!(
        competitions.len(),
        1,
        "expected one competition in {payload}"
    );
    let competition = &competitions[0];
    let actual = vec![vec![
        text_field(competition, "name"),
        scalar_field(competition, "snapshot_year"),
        years(competition).len().to_string(),
        first_year(competition),
        last_year(competition),
    ]];
    assert_rows(
        step,
        &[
            "competition",
            "snapshot year",
            "year count",
            "first year",
            "last year",
        ],
        actual,
    );
}

/// Normaliza o acesso à competição em respostas atuais e anuais.
///
/// # Retorno
/// Primeiro elemento quando o payload é um array; o próprio payload nos demais casos.
///
/// # Erros
/// Dispara panic quando a resposta em array está vazia.
fn competition_from_view(payload: &Value) -> &Value {
    if payload.is_array() {
        array(payload)
            .first()
            .unwrap_or_else(|| panic!("expected a competition view in {payload}"))
    } else {
        payload
    }
}

/// Compara os eventos da competição e os nomes de suas equipes, preservando as duas ordens.
#[then("the competition events should contain these teams, in order:")]
fn competition_event_teams_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let competition = competition_from_view(&payload);
    let actual = array(field(competition, "events"))
        .iter()
        .map(|event| vec![text_field(event, "name"), joined_names(event, "teams")])
        .collect();
    assert_rows(step, &["event", "teams"], actual);
}

/// Exige um array vazio quando a visão atual da competição está ausente.
#[then("no competition view should be returned")]
fn no_competition_view_should_be_returned(world: &mut ApiWorld) {
    assert_empty_array(world);
}

/// Exige listas vazias de tipos de localização e eventos na visão anual da competição.
#[then("the annual competition view should contain no location types or events")]
fn annual_competition_view_should_be_empty(world: &mut ApiWorld) {
    assert_empty_annual_view(world);
}

/// Projeta o portfólio de uma instituição ou equipe em linhas comparáveis.
///
/// # Retorno
/// Uma linha por competição com entidade, competição, snapshot, quantidade de
/// anos e nomes dos eventos. Preserva a ordem da API.
///
/// # Erros
/// Dispara panic se o payload não contiver exatamente uma entidade ou se os
/// campos obrigatórios tiverem formato incompatível.
fn portfolio_rows(payload: &Value) -> Vec<Vec<String>> {
    let entities = array(payload);
    assert_eq!(entities.len(), 1, "expected one portfolio in {payload}");
    let entity = &entities[0];

    array(field(entity, "competitions"))
        .iter()
        .map(|competition| {
            vec![
                text_field(entity, "name"),
                text_field(competition, "name"),
                scalar_field(competition, "snapshot_year"),
                years(competition).len().to_string(),
                joined_names(competition, "events"),
            ]
        })
        .collect()
}

/// Compara o portfólio completo da instituição com a tabela de competições e eventos.
#[then("the institution portfolio should contain:")]
fn institution_portfolio_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    assert_rows(
        step,
        &[
            "institution",
            "competition",
            "snapshot year",
            "year count",
            "events",
        ],
        portfolio_rows(&payload),
    );
}

/// Exige um array vazio para uma consulta de portfólio institucional sem resultados.
#[then("no institution portfolio should be returned")]
fn no_institution_portfolio_should_be_returned(world: &mut ApiWorld) {
    assert_empty_array(world);
}

/// Compara o portfólio completo da equipe com a tabela de competições e eventos.
#[then("the team portfolio should contain:")]
fn team_portfolio_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    assert_rows(
        step,
        &[
            "team",
            "competition",
            "snapshot year",
            "year count",
            "events",
        ],
        portfolio_rows(&payload),
    );
}

/// Exige um array vazio para uma consulta de portfólio de equipe sem resultados.
#[then("no team portfolio should be returned")]
fn no_team_portfolio_should_be_returned(world: &mut ApiWorld) {
    assert_empty_array(world);
}

/// Compara os totais de membros e participantes femininas no desempenho da equipe.
#[then(
    regex = r"^the team performance should report (\d+) members and (\d+) female participants?$"
)]
fn team_performance_totals_should_equal(
    world: &mut ApiWorld,
    expected_members: i64,
    expected_female_participants: i64,
) {
    let payload = successful_response(world);
    assert_eq!(integer_field(&payload, "total_members"), expected_members);
    assert_eq!(
        integer_field(&payload, "female_participants"),
        expected_female_participants
    );
}

/// Compara eventos e posições da equipe na ordem retornada.
#[then("the event ranks should be:")]
fn event_ranks_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(field(&payload, "events"))
        .iter()
        .map(|event| {
            vec![
                text_field(event, "name"),
                scalar_field(event, "team_event_rank"),
            ]
        })
        .collect();
    assert_rows(step, &["event", "rank"], actual);
}

/// Exige totais de membros e participantes femininas zerados e uma lista vazia de eventos.
#[then("the team performance should contain no members or events")]
fn empty_team_performance_should_be_returned(world: &mut ApiWorld) {
    let payload = successful_response(world);
    assert_eq!(integer_field(&payload, "total_members"), 0);
    assert_eq!(integer_field(&payload, "female_participants"), 0);
    assert!(array(field(&payload, "events")).is_empty());
}

/// Compara identidade do evento e da competição, ano selecionado e resumo dos anos disponíveis.
#[then("the event view should describe:")]
fn event_view_should_be_described(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = vec![vec![
        scalar_field(&payload, "id"),
        text_field(&payload, "name"),
        integer_at(&payload, "/competition/id").to_string(),
        text_at(&payload, "/competition/name"),
        scalar_field(&payload, "year"),
        years(&payload).len().to_string(),
        first_year(&payload),
        last_year(&payload),
    ]];
    assert_rows(
        step,
        &[
            "event id",
            "event",
            "competition id",
            "competition",
            "selected year",
            "year count",
            "first year",
            "last year",
        ],
        actual,
    );
}

/// Compara o ano selecionado e a quantidade de anos disponíveis na estrutura do evento.
#[then(regex = r"^the event view should select (\d+) from (\d+) available years$")]
fn event_view_year_should_equal(
    world: &mut ApiWorld,
    expected_year: i64,
    expected_year_count: usize,
) {
    let payload = successful_response(world);
    assert_eq!(integer_field(&payload, "year"), expected_year);
    assert_eq!(years(&payload).len(), expected_year_count);
}

/// Compara os IDs e as datas das ocorrências na ordem recebida.
#[then("the event occurrences should be:")]
fn event_occurrences_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let payload = successful_response(world);
    let actual = array(field(&payload, "instances"))
        .iter()
        .map(|instance| vec![scalar_field(instance, "id"), text_field(instance, "date")])
        .collect();
    assert_rows(step, &["id", "date"], actual);
}

/// Exige preservação do evento e ano sem participantes, com uma ocorrência e sem tipos de localização.
#[then(
    regex = r#"^the view should preserve the unplayed event "([^"]+)" for (\d+) as occurrence (\d+) without location types$"#
)]
fn unplayed_event_should_be_preserved(
    world: &mut ApiWorld,
    expected_name: String,
    expected_year: i64,
    expected_occurrence_id: i64,
) {
    let payload = successful_response(world);
    assert_eq!(text_field(&payload, "name"), expected_name);
    assert_eq!(integer_field(&payload, "year"), expected_year);
    assert!(array(field(&payload, "location_types")).is_empty());
    let instances = array(field(&payload, "instances"));
    assert_eq!(instances.len(), 1);
    assert_eq!(integer_field(&instances[0], "id"), expected_occurrence_id);
}

/// Exige HTTP 404 e um corpo JSON válido para evento ou recorte ausente.
#[then("the event view should not be found")]
fn event_view_should_not_be_found(world: &mut ApiWorld) {
    response_with_status(world, StatusCode::NOT_FOUND);
}

/// Exige o status informado e um único campo público textual `error` no payload.
#[then(expr = "the response should expose only a public JSON error with status {int}")]
fn response_should_expose_public_error(world: &mut ApiWorld, expected_status: u16) {
    let status = StatusCode::from_u16(expected_status).expect("status should be valid HTTP");
    let payload = response_with_status(world, status);
    let object = payload
        .as_object()
        .unwrap_or_else(|| panic!("error response should be an object, but was {payload}"));
    assert_eq!(
        object.len(),
        1,
        "error response should expose only its public message: {payload}"
    );
    assert!(
        object.get("error").and_then(Value::as_str).is_some(),
        "error response should expose a string field named 'error': {payload}"
    );
}

/// Verifica se a mensagem `error` contém o trecho esperado, ignorando diferenças de maiúsculas.
#[then(regex = r#"^the public error message should mention "([^"]+)"$"#)]
fn public_error_message_should_mention(world: &mut ApiWorld, expected: String) {
    let response = world
        .last_response
        .as_ref()
        .expect("an error response should have been captured");
    let payload: Value = serde_json::from_str(&response.body)
        .unwrap_or_else(|error| panic!("error response should be valid JSON: {error}"));
    let message = field(&payload, "error")
        .as_str()
        .unwrap_or_else(|| panic!("public error should be text in {payload}"));
    assert!(
        message.to_lowercase().contains(&expected.to_lowercase()),
        "public error {message:?} should mention {expected:?}"
    );
}
