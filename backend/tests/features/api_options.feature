Feature: Hierarchical selection and navigation options
  The options routes expose compact catalogs and help locate entities at
  successive hierarchy levels without defining the scope of the detailed
  result displayed after selection.

  Background:
    Given the backend API is running with an isolated database
    And the deterministic BDD fixture is loaded

  Scenario: List organizers alphabetically
    When I request GET "/organizers/options"
    Then the response status should be 200
    And the response JSON should equal:
      """
      [
        {"id": 1, "name": "Algorithm League"},
        {"id": 3, "name": "Inactive Organizer"},
        {"id": 2, "name": "Women Code Foundation"}
      ]
      """

  Scenario: List the complete competition catalog
    When I request GET "/competitions/options"
    Then the response status should be 200
    And the values of field "name" in the JSON array at "$" should be:
      | Global Algorithm Cup |
      | Regional Code League |
      | Unplayed Cup         |
      | Women Challenge      |

  Scenario: Restrict competitions to the selected organizers
    When I request GET "/competitions/options?organizer_ids=1"
    Then the response status should be 200
    And the response JSON should equal:
      """
      [
        {"id": 10, "name": "Global Algorithm Cup"},
        {"id": 20, "name": "Regional Code League"}
      ]
      """

  Scenario: Return no competitions for a nonexistent organizer
    When I request GET "/competitions/options?organizer_ids=999"
    Then the response status should be 200
    And the response JSON should equal:
      """
      []
      """

  Scenario: Locate institutions that participated in a competition
    When I request GET "/institutions/options?competition_ids=10"
    Then the response status should be 200
    And the values of field "name" in the JSON array at "$" should be:
      | Alpha University |
      | Beta Institute   |
      | Gamma University |

  Scenario: Include registered entities with no participations in complete catalogs
    When I request GET "/institutions/options"
    Then the response status should be 200
    And the values of field "name" in the JSON array at "$" should be:
      | Ada College       |
      | Alpha University  |
      | Beta Institute    |
      | Gamma University  |
      | Idle University   |
    When I request GET "/teams/options"
    Then the response status should be 200
    And the values of field "name" in the JSON array at "$" should be:
      | Ada Lovelace |
      | Alpha Bytes  |
      | Alpha Coders |
      | Beta Stack   |
      | Gamma Graph  |
      | Idle Team    |

  Scenario: Return no institutions for a competition with no participations
    When I request GET "/institutions/options?competition_ids=40"
    Then the response status should be 200
    And the response JSON should equal:
      """
      []
      """

  Scenario: Locate teams by competition and institution
    When I request GET "/teams/options?competition_ids=10&institution_ids=1"
    Then the response status should be 200
    And the response JSON should equal:
      """
      [
        {
          "id": 2,
          "name": "Alpha Bytes",
          "institution_id": 1,
          "institution_name": "Alpha University",
          "institution_short_name": "AU"
        },
        {
          "id": 1,
          "name": "Alpha Coders",
          "institution_id": 1,
          "institution_name": "Alpha University",
          "institution_short_name": "AU"
        }
      ]
      """

  Scenario: Apply union within lists and intersection across dimensions
    When I request GET "/teams/options?competition_ids=10,30&institution_ids=1,4"
    Then the response status should be 200
    And the values of field "name" in the JSON array at "$" should be:
      | Ada Lovelace |
      | Alpha Bytes  |
      | Alpha Coders |

  Scenario: Filter teams only by institution
    When I request GET "/teams/options?institution_ids=5"
    Then the response status should be 200
    And the response JSON should equal:
      """
      [
        {
          "id": 6,
          "name": "Idle Team",
          "institution_id": 5,
          "institution_name": "Idle University",
          "institution_short_name": "IU"
        }
      ]
      """

  Scenario: Return no teams when hierarchical filters are incompatible
    When I request GET "/teams/options?competition_ids=20&institution_ids=2"
    Then the response status should be 200
    And the response JSON should equal:
      """
      []
      """

  Scenario: List the events and years in which an institution participated
    When I request GET "/institutions/1/events/options"
    Then the response status should be 200
    And the JSON array at "$" should have length 3
    And the values of field "name" in the JSON array at "$" should be:
      | Algorithm Final    |
      | Algorithm Regional |
      | Regional Final     |
    And the JSON value at "/0/competition_name" should be "Global Algorithm Cup"
    And the JSON array at "/0/years" should have length 3
    And the JSON value at "/0/years/0" should be 2023
    And the JSON value at "/0/years/2" should be 2025
    And the JSON array at "/1/years" should have length 2
    And the JSON value at "/1/years/0" should be 2024
    And the JSON value at "/1/years/1" should be 2025
    And the JSON value at "/2/competition_name" should be "Regional Code League"
    And the JSON array at "/2/years" should have length 2
    And the JSON value at "/2/years/0" should be 2024
    And the JSON value at "/2/years/1" should be 2026

  Scenario: Return an empty event list for an institution with no participations
    When I request GET "/institutions/5/events/options"
    Then the response status should be 200
    And the response JSON should equal:
      """
      []
      """
