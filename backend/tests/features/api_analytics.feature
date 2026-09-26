Feature: API analytics queries
  Statistics and historical series should deterministically represent unique
  competitors without counting coaches, reserves, or the same contestant's
  repeated event registrations more than once per scope.

  Scenario: Aggregate annual statistics for a competition
    When I request competition 10 statistics for 2025
    Then the competition statistics should be:
      | institutions | teams | participants | female participants |
      | 3            | 4     | 8            | 4                   |

  Scenario: Aggregate annual statistics for an event
    When I request event 100 statistics for 2024
    Then the event statistics should be:
      | institutions | teams | participants | female participants |
      | 3            | 4     | 8            | 4                   |

  Scenario: Group competition statistics by country
    When I group competition 10 statistics by country for 2025
    Then the country statistics should be:
      | id | country   | institutions | teams | participants | female participants |
      | 3  | Argentina | 1            | 1     | 2            | 1                   |
      | 2  | Brazil    | 2            | 3     | 6            | 3                   |

  Scenario: Group event statistics by country
    When I group event 100 statistics by country for 2024
    Then the event country statistics should be:
      | country   | teams | female participants |
      | Argentina | 1     | 1                   |
      | Brazil    | 3     | 3                   |

  Scenario: List historical events available to an institution
    When I request the event history for institution 1
    Then the available event histories should be:
      | event id | event              | competition id | competition          | years            |
      | 100      | Algorithm Final    | 10             | Global Algorithm Cup | 2023, 2024, 2025 |
      | 101      | Algorithm Regional | 10             | Global Algorithm Cup | 2024, 2025       |
      | 200      | Regional Final     | 20             | Regional Code League | 2024, 2026       |

  Scenario: Retrieve an institution's performance progression in an event
    When I request institution 1's performance in event 100 from 2023 through 2025
    Then the performance history should be:
      | year | best rank | best team id | best team    | average rank |
      | 2023 | 2         | 1            | Alpha Coders | 2.0          |
      | 2024 | 1         | 2            | Alpha Bytes  | 2.0          |
      | 2025 | 1         | 1            | Alpha Coders | 2.0          |

  Scenario: Return empty collections when no data exists for a valid scope
    When I request the event history for institution 5
    Then no event history should be returned
    When I request institution 1's performance in event 100 from 2020 through 2022
    Then no performance history should be returned
    When I group competition 40 statistics by country for 2025
    Then no location statistics should be returned
    When I group event 400 statistics by country for 2025
    Then no location statistics should be returned
