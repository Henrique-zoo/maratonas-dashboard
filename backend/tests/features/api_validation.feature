Feature: API HTTP contract validation
  Required parameters and malformed values should be rejected predictably,
  while missing events and missing years should remain distinguishable.

  Background:
    Given the backend API is running with an isolated database
    And the deterministic BDD fixture is loaded

  Scenario Outline: Reject missing selections required by general structures
    When I request GET "<path>"
    Then the response should be a JSON error with status 400
    And the response body should contain "<entity>"

    Examples:
      | path                     | entity      |
      | /organizers/structures   | organizer   |
      | /competitions/structures | competition |
      | /institutions/structures | institution |
      | /teams/structures        | team        |

  Scenario Outline: Require a year for annual structures
    When I request GET "<path>"
    Then the response should be a JSON error with status 400
    And the response body should contain "year"

    Examples:
      | path                                |
      | /organizers/competitions/10/structure |
      | /competitions/10/structure            |
      | /teams/1/competitions/10/structure     |

  Scenario Outline: Reject missing required analytics parameters
    When I request GET "<path>"
    Then the response should be a JSON error with status 400
    And the response body should contain "<parameter>"

    Examples:
      | path                                                               | parameter     |
      | /competitions/10/stats                                             | year          |
      | /events/100/stats                                                  | year          |
      | /competitions/10/location-stats?year=2025                          | location type |
      | /competitions/10/location-stats?location_type=Country              | year          |
      | /events/100/location-stats?year=2025                               | location type |
      | /events/100/location-stats?location_type=Country                   | year          |
      | /institutions/1/events/100/performance?end_year=2025               | start year    |
      | /institutions/1/events/100/performance?start_year=2023             | end year      |

  Scenario Outline: Reject malformed identifier lists
    When I request GET "<path>"
    Then the response should be a JSON error with status 400

    Examples:
      | path                                                        |
      | /organizers/structures?organizer_ids=1,invalid              |
      | /competitions/options?organizer_ids=1,invalid               |
      | /competitions/structures?competition_ids=10,invalid         |
      | /institutions/options?competition_ids=10,invalid            |
      | /institutions/structures?institution_ids=1,invalid          |
      | /teams/options?institution_ids=1,invalid                    |
      | /teams/structures?team_ids=1,invalid                        |

  Scenario Outline: Reject malformed values outside lists
    When I request GET "<path>"
    Then the response should be a JSON error with status 400

    Examples:
      | path                                                               |
      | /events/invalid/structure                                          |
      | /events/100/stats?year=invalid                                     |
      | /events/100/location-stats?location_type=Planet&year=2025          |
      | /institutions/invalid/events/100/performance?start_year=2023&end_year=2025 |

  Scenario: Distinguish a nonexistent event from a year with no instance
    When I request GET "/events/999/structure"
    Then the response should be a JSON error with status 404
    And the response body should contain "Event 999"
    When I request GET "/events/100/structure?year=2022"
    Then the response should be a JSON error with status 404
    And the response body should contain "Event 100"
    And the response body should contain "2022"
