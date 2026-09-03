Feature: Hierarchical structures, snapshots, and annual views
  General structures expose recent snapshots and preserve available years.
  Selecting an institution or team should retrieve its complete portfolio,
  while annual routes apply explicit year scopes.

  Background:
    Given the backend API is running with an isolated database
    And the deterministic BDD fixture is loaded

  Scenario: Get an organizer's competition snapshot
    When I request GET "/organizers/structures?organizer_ids=1"
    Then the response status should be 200
    And the JSON array at "$" should have length 1
    And the JSON value at "/0/name" should be "Algorithm League"
    And the JSON array at "/0/competitions" should have length 2
    And the values of field "name" in the JSON array at "/0/competitions" should be:
      | Global Algorithm Cup |
      | Regional Code League |
    And the JSON value at "/0/competitions/0/snapshot_year" should be 2025
    And the JSON array at "/0/competitions/0/years" should have length 3
    And the JSON value at "/0/competitions/0/years/0" should be 2023
    And the JSON value at "/0/competitions/0/years/2" should be 2025
    And the values of field "name" in the JSON array at "/0/competitions/0/events" should be:
      | Algorithm Final    |
      | Algorithm Regional |
    And the JSON value at "/0/competitions/0/events/0/total_participants" should be 6
    And the JSON value at "/0/competitions/0/events/0/female_participants" should be 3
    And the JSON value at "/0/competitions/0/events/1/total_participants" should be 4
    And the JSON value at "/0/competitions/0/events/1/female_participants" should be 3
    And the JSON value at "/0/competitions/1/snapshot_year" should be 2026
    And the JSON array at "/0/competitions/1/years" should have length 2
    And the JSON value at "/0/competitions/1/years/0" should be 2024
    And the JSON value at "/0/competitions/1/years/1" should be 2026

  Scenario: Return an empty structure for an organizer with no results
    When I request GET "/organizers/structures?organizer_ids=3"
    Then the response status should be 200
    And the response JSON should equal:
      """
      []
      """

  Scenario: Get the annual aggregate view of a competition within the organizer domain
    When I request GET "/organizers/competitions/10/structure?year=2024"
    Then the response status should be 200
    And the JSON array at "/events" should have length 2
    And the values of field "name" in the JSON array at "/events" should be:
      | Algorithm Final    |
      | Algorithm Regional |
    And the JSON value at "/events/0/date" should be "2024-09-10"
    And the JSON value at "/events/1/date" should be "2024-06-01"
    And the JSON value at "/events/0/total_participants" should be 8
    And the JSON value at "/events/0/female_participants" should be 4
    And the JSON value at "/events/1/total_participants" should be 4
    And the JSON value at "/events/1/female_participants" should be 1

  Scenario: Return an empty annual aggregate view when there are no events
    When I request GET "/organizers/competitions/10/structure?year=1999"
    Then the response status should be 200
    And the response JSON should equal:
      """
      {"location_types": [], "events": []}
      """

  Scenario: Get the detailed snapshot of a competition's latest year
    When I request GET "/competitions/structures?competition_ids=10"
    Then the response status should be 200
    And the JSON array at "$" should have length 1
    And the JSON value at "/0/name" should be "Global Algorithm Cup"
    And the JSON value at "/0/snapshot_year" should be 2025
    And the JSON array at "/0/years" should have length 3
    And the JSON value at "/0/years/0" should be 2023
    And the JSON value at "/0/years/2" should be 2025
    And the values of field "name" in the JSON array at "/0/events" should be:
      | Algorithm Final    |
      | Algorithm Regional |
    And the values of field "name" in the JSON array at "/0/events/0/teams" should be:
      | Alpha Coders |
      | Beta Stack   |
      | Alpha Bytes  |
    And the values of field "name" in the JSON array at "/0/events/1/teams" should be:
      | Alpha Bytes |
      | Gamma Graph |

  Scenario: Return an empty structure for a competition with no results
    When I request GET "/competitions/structures?competition_ids=40"
    Then the response status should be 200
    And the response JSON should equal:
      """
      []
      """

  Scenario: Get a competition's detailed structure for an explicit year
    When I request GET "/competitions/10/structure?year=2024"
    Then the response status should be 200
    And the JSON array at "/events" should have length 2
    And the values of field "name" in the JSON array at "/events" should be:
      | Algorithm Final    |
      | Algorithm Regional |
    And the values of field "name" in the JSON array at "/events/0/teams" should be:
      | Alpha Bytes  |
      | Beta Stack   |
      | Alpha Coders |
      | Gamma Graph  |
    And the values of field "name" in the JSON array at "/events/1/teams" should be:
      | Beta Stack   |
      | Alpha Coders |

  Scenario: Return an empty annual structure when a competition has no events that year
    When I request GET "/competitions/10/structure?year=1999"
    Then the response status should be 200
    And the response JSON should equal:
      """
      {"location_types": [], "events": []}
      """

  Scenario: Retrieve an institution's complete portfolio
    When I request GET "/institutions/structures?institution_ids=1"
    Then the response status should be 200
    And the JSON array at "$" should have length 1
    And the JSON value at "/0/name" should be "Alpha University"
    And the JSON array at "/0/competitions" should have length 2
    And the values of field "name" in the JSON array at "/0/competitions" should be:
      | Global Algorithm Cup |
      | Regional Code League |
    And the JSON value at "/0/competitions/0/snapshot_year" should be 2025
    And the JSON array at "/0/competitions/0/years" should have length 3
    And the values of field "name" in the JSON array at "/0/competitions/0/events" should be:
      | Algorithm Final    |
      | Algorithm Regional |
    And the JSON value at "/0/competitions/1/snapshot_year" should be 2026
    And the JSON array at "/0/competitions/1/years" should have length 2
    And the values of field "name" in the JSON array at "/0/competitions/1/events" should be:
      | Regional Final |

  Scenario: Return no structure for an institution with no participations
    When I request GET "/institutions/structures?institution_ids=5"
    Then the response status should be 200
    And the response JSON should equal:
      """
      []
      """

  Scenario: Retrieve a team's complete portfolio
    When I request GET "/teams/structures?team_ids=1"
    Then the response status should be 200
    And the JSON array at "$" should have length 1
    And the JSON value at "/0/name" should be "Alpha Coders"
    And the JSON array at "/0/competitions" should have length 2
    And the values of field "name" in the JSON array at "/0/competitions" should be:
      | Global Algorithm Cup |
      | Regional Code League |
    And the JSON value at "/0/competitions/0/snapshot_year" should be 2025
    And the JSON array at "/0/competitions/0/years" should have length 3
    And the values of field "name" in the JSON array at "/0/competitions/0/events" should be:
      | Algorithm Final |
    And the JSON value at "/0/competitions/1/snapshot_year" should be 2026
    And the JSON array at "/0/competitions/1/years" should have length 2
    And the values of field "name" in the JSON array at "/0/competitions/1/events" should be:
      | Regional Final |

  Scenario: Return no structure for a team with no participations
    When I request GET "/teams/structures?team_ids=6"
    Then the response status should be 200
    And the response JSON should equal:
      """
      []
      """

  Scenario: Get a team's annual performance in a competition
    When I request GET "/teams/1/competitions/10/structure?year=2024"
    Then the response status should be 200
    And the JSON value at "/total_members" should be 2
    And the JSON value at "/female_participants" should be 1
    And the JSON array at "/events" should have length 2
    And the values of field "name" in the JSON array at "/events" should be:
      | Algorithm Regional |
      | Algorithm Final    |
    And the JSON value at "/events/0/team_event_rank" should be 2
    And the JSON value at "/events/1/team_event_rank" should be 3

  Scenario: Return empty annual performance when the team did not compete that year
    When I request GET "/teams/1/competitions/10/structure?year=1999"
    Then the response status should be 200
    And the response JSON should equal:
      """
      {"total_members": 0, "female_participants": 0, "events": []}
      """

  Scenario: Use the latest available year by default for an event structure
    When I request GET "/events/100/structure"
    Then the response status should be 200
    And the JSON value at "/id" should be 100
    And the JSON value at "/name" should be "Algorithm Final"
    And the JSON value at "/competition/id" should be 10
    And the JSON value at "/competition/name" should be "Global Algorithm Cup"
    And the JSON value at "/year" should be 2025
    And the JSON array at "/years" should have length 3
    And the JSON value at "/years/0" should be 2023
    And the JSON value at "/years/2" should be 2025
    And the JSON array at "/instances" should have length 2
    And the JSON value at "/instances/0/id" should be 1002
    And the JSON value at "/instances/0/date" should be "2025-09-10"
    And the JSON value at "/instances/1/id" should be 1003
    And the JSON value at "/instances/1/date" should be "2025-09-11"

  Scenario: Honor the explicit year in an event structure
    When I request GET "/events/100/structure?year=2024"
    Then the response status should be 200
    And the JSON value at "/year" should be 2024
    And the JSON array at "/years" should have length 3
    And the JSON array at "/instances" should have length 1
    And the JSON value at "/instances/0/id" should be 1001
    And the JSON value at "/instances/0/date" should be "2024-09-10"

  Scenario: Preserve an event instance even when no teams participated
    When I request GET "/events/400/structure"
    Then the response status should be 200
    And the JSON value at "/name" should be "Unplayed Event"
    And the JSON value at "/year" should be 2025
    And the JSON array at "/location_types" should have length 0
    And the JSON array at "/instances" should have length 1
    And the JSON value at "/instances/0/id" should be 4000

  Scenario: Report not found when an event has no instance in the requested year
    When I request GET "/events/100/structure?year=1999"
    Then the response status should be 404
