Feature: API analytics queries
  Statistics and historical series should deterministically represent unique
  competitors without counting coaches, reserves, or the same contestant's
  repeated event registrations more than once per scope.

  Background:
    Given the backend API is running with an isolated database
    And the deterministic BDD fixture is loaded

  Scenario: Aggregate annual statistics for a competition
    When I request GET "/competitions/10/stats?year=2025"
    Then the response status should be 200
    And the response JSON should equal:
      """
      {
        "total_institutions": 3,
        "total_teams": 4,
        "total_participants": 8,
        "female_participants": 4
      }
      """

  Scenario: Aggregate annual statistics for an event
    When I request GET "/events/100/stats?year=2024"
    Then the response status should be 200
    And the response JSON should equal:
      """
      {
        "total_institutions": 3,
        "total_teams": 4,
        "total_participants": 8,
        "female_participants": 4
      }
      """

  Scenario: Group competition statistics by country
    When I request GET "/competitions/10/location-stats?location_type=Country&year=2025"
    Then the response status should be 200
    And the response JSON should equal:
      """
      [
        {
          "id": 3,
          "name": "Argentina",
          "total_institutions": 1,
          "total_teams": 1,
          "total_participants": 2,
          "female_participants": 1
        },
        {
          "id": 2,
          "name": "Brazil",
          "total_institutions": 2,
          "total_teams": 3,
          "total_participants": 6,
          "female_participants": 3
        }
      ]
      """

  Scenario: Group event statistics by country
    When I request GET "/events/100/location-stats?location_type=Country&year=2024"
    Then the response status should be 200
    And the JSON array at "$" should have length 2
    And the values of field "name" in the JSON array at "$" should be:
      | Argentina |
      | Brazil    |
    And the JSON value at "/0/total_teams" should be 1
    And the JSON value at "/0/female_participants" should be 1
    And the JSON value at "/1/total_teams" should be 3
    And the JSON value at "/1/female_participants" should be 3

  Scenario: List historical events available to an institution
    When I request GET "/institutions/1/events/options"
    Then the response status should be 200
    And the response JSON should equal:
      """
      [
        {
          "id": 100,
          "name": "Algorithm Final",
          "competition_id": 10,
          "competition_name": "Global Algorithm Cup",
          "years": [2023, 2024, 2025]
        },
        {
          "id": 101,
          "name": "Algorithm Regional",
          "competition_id": 10,
          "competition_name": "Global Algorithm Cup",
          "years": [2024, 2025]
        },
        {
          "id": 200,
          "name": "Regional Final",
          "competition_id": 20,
          "competition_name": "Regional Code League",
          "years": [2024, 2026]
        }
      ]
      """

  Scenario: Retrieve an institution's performance progression in an event
    When I request GET "/institutions/1/events/100/performance?start_year=2023&end_year=2025"
    Then the response status should be 200
    And the response JSON should equal:
      """
      [
        {
          "year": 2023,
          "best_performance_rank": 2,
          "best_performance_team_id": 1,
          "best_performance_team_name": "Alpha Coders",
          "average_performance_rank": 2.0
        },
        {
          "year": 2024,
          "best_performance_rank": 1,
          "best_performance_team_id": 2,
          "best_performance_team_name": "Alpha Bytes",
          "average_performance_rank": 2.0
        },
        {
          "year": 2025,
          "best_performance_rank": 1,
          "best_performance_team_id": 1,
          "best_performance_team_name": "Alpha Coders",
          "average_performance_rank": 2.0
        }
      ]
      """

  Scenario: Return empty collections when no data exists for a valid scope
    When I request GET "/institutions/5/events/options"
    Then the response status should be 200
    And the response JSON should be an empty array
    When I request GET "/institutions/1/events/100/performance?start_year=2020&end_year=2022"
    Then the response status should be 200
    And the response JSON should be an empty array
    When I request GET "/competitions/40/location-stats?location_type=Country&year=2025"
    Then the response status should be 200
    And the response JSON should be an empty array
    When I request GET "/events/400/location-stats?location_type=Country&year=2025"
    Then the response status should be 200
    And the response JSON should be an empty array
