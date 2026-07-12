Feature: Filtros em cascata do index
  Background:
    Given the backend API is running with an isolated database
    And o contexto de filtros do index foi carregado
    And estou na pagina index

  Scenario: Selecionar uma organizacao atualiza somente o contexto local
    Given existe a organizacao "ICPC"
    When seleciono "ICPC" no filtro de organizacao
    Then devo continuar na pagina "/"
    And a selecao nao deve enviar request ao backend
    And as opcoes do filtro de competicoes devem ser:
      | ICPC Latin America Championship |
      | ICPC South America Regional     |
      | ICPC World Championship         |
    When clico em Apply Filters
    Then devo estar na pagina "/?organizer=1"
    And devo ver 2 competicoes no resumo dos filtros
    And devo ver 2 eventos no resumo dos filtros
    And devo ver 11 times no resumo dos filtros

  Scenario: Selecionar uma competicao atualiza instituicoes sem aplicar filtros
    Given existe a organizacao "ICPC"
    And existe a competicao "ICPC Latin America Championship"
    When seleciono "ICPC" no filtro de organizacao
    And seleciono "ICPC Latin America Championship" no filtro de competicao
    Then devo continuar na pagina "/"
    And a selecao nao deve enviar request ao backend
    And as opcoes do filtro de instituicoes devem ser:
      | Pontificia Universidad Catolica de Chile |
      | Universidad de Buenos Aires              |
      | Universidad de los Andes                 |
      | Universidade Estadual de Campinas        |
      | Universidade Federal do Rio de Janeiro   |
      | Universidade de Sao Paulo                |
    When clico em Apply Filters
    Then devo estar na pagina "/?organizer=1&competition=2"
    And devo ver 1 competicoes no resumo dos filtros
    And devo ver 1 eventos no resumo dos filtros
    And devo ver 6 times no resumo dos filtros
