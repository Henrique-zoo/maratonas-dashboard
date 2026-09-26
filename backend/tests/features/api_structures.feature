Feature: Hierarchical structures, snapshots, and annual views
  General structures expose recent snapshots and preserve available years.
  Selecting an institution or team should retrieve its complete portfolio,
  while annual views apply explicit year scopes.

  Scenario: Get an organizer's competition snapshot
    When I request organizer 1's current competition overview
    Then the organizer overview should contain:
      | organizer        | competition          | snapshot year | year count | first year | last year |
      | Algorithm League | Global Algorithm Cup | 2025          | 3          | 2023       | 2025      |
      | Algorithm League | Regional Code League | 2026          | 2          | 2024       | 2026      |
    And "Global Algorithm Cup" should summarize these events:
      | event              | participants | female participants |
      | Algorithm Final    | 6            | 3                   |
      | Algorithm Regional | 4            | 3                   |

  Scenario: Return an empty structure for an organizer with no results
    When I request organizer 3's current competition overview
    Then no organizer overview should be returned

  Scenario: Get the annual aggregate view of a competition within the organizer domain
    When I request competition 10's organizer overview for 2024
    Then the annual event overview should be:
      | event              | date       | participants | female participants |
      | Algorithm Final    | 2024-09-10 | 8            | 4                   |
      | Algorithm Regional | 2024-06-01 | 4            | 1                   |

  Scenario: Return an empty annual aggregate view when there are no events
    When I request competition 10's organizer overview for 1999
    Then the annual overview should contain no location types or events

  Scenario: Get the detailed snapshot of a competition's latest year
    When I request competition 10's latest detailed view
    Then the competition view should describe:
      | competition          | snapshot year | year count | first year | last year |
      | Global Algorithm Cup | 2025          | 3          | 2023       | 2025      |
    And the competition events should contain these teams, in order:
      | event              | teams                                   |
      | Algorithm Final    | Alpha Coders; Beta Stack; Alpha Bytes   |
      | Algorithm Regional | Alpha Bytes; Gamma Graph                |

  Scenario: Return an empty structure for a competition with no results
    When I request competition 40's latest detailed view
    Then no competition view should be returned

  Scenario: Get a competition's detailed structure for an explicit year
    When I request competition 10's detailed view for 2024
    Then the competition events should contain these teams, in order:
      | event              | teams                                                |
      | Algorithm Final    | Alpha Bytes; Beta Stack; Alpha Coders; Gamma Graph   |
      | Algorithm Regional | Beta Stack; Alpha Coders                             |

  Scenario: Return an empty annual structure when a competition has no events that year
    When I request competition 10's detailed view for 1999
    Then the annual competition view should contain no location types or events

  Scenario: Retrieve an institution's complete portfolio
    When I request institution 1's complete portfolio
    Then the institution portfolio should contain:
      | institution      | competition          | snapshot year | year count | events                                |
      | Alpha University | Global Algorithm Cup | 2025          | 3          | Algorithm Final; Algorithm Regional   |
      | Alpha University | Regional Code League | 2026          | 2          | Regional Final                        |

  Scenario: Return no structure for an institution with no participations
    When I request institution 5's complete portfolio
    Then no institution portfolio should be returned

  Scenario: Retrieve a team's complete portfolio
    When I request team 1's complete portfolio
    Then the team portfolio should contain:
      | team         | competition          | snapshot year | year count | events          |
      | Alpha Coders | Global Algorithm Cup | 2025          | 3          | Algorithm Final |
      | Alpha Coders | Regional Code League | 2026          | 2          | Regional Final  |

  Scenario: Return no structure for a team with no participations
    When I request team 6's complete portfolio
    Then no team portfolio should be returned

  Scenario: Get a team's annual performance in a competition
    When I request team 1's performance in competition 10 for 2024
    Then the team performance should report 2 members and 1 female participant
    And the event ranks should be:
      | event              | rank |
      | Algorithm Regional | 2    |
      | Algorithm Final    | 3    |

  Scenario: Return empty annual performance when the team did not compete that year
    When I request team 1's performance in competition 10 for 1999
    Then the team performance should contain no members or events

  Scenario: Use the latest available year by default for an event structure
    When I request event 100's current view
    Then the event view should describe:
      | event id | event           | competition id | competition          | selected year | year count | first year | last year |
      | 100      | Algorithm Final | 10             | Global Algorithm Cup | 2025          | 3          | 2023       | 2025      |
    And the event occurrences should be:
      | id   | date       |
      | 1002 | 2025-09-10 |
      | 1003 | 2025-09-11 |

  Scenario: Honor the explicit year in an event structure
    When I request event 100's view for 2024
    Then the event view should select 2024 from 3 available years
    And the event occurrences should be:
      | id   | date       |
      | 1001 | 2024-09-10 |

  Scenario: Preserve an event instance even when no teams participated
    When I request event 400's current view
    Then the view should preserve the unplayed event "Unplayed Event" for 2025 as occurrence 4000 without location types

  Scenario: Report not found when an event has no instance in the requested year
    When I request event 100's view for 1999
    Then the event view should not be found
