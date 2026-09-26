Feature: Hierarchical selection and navigation options
  The options routes expose compact catalogs and help locate entities at
  successive hierarchy levels without defining the scope of the detailed
  result displayed after selection.

  Scenario: List organizers alphabetically
    When I list the available organizers
    Then the organizer options should be, in order:
      | id | organizer            |
      | 1  | Algorithm League     |
      | 3  | Inactive Organizer   |
      | 2  | Women Code Foundation |

  Scenario: List the complete competition catalog
    When I list all competitions
    Then the competition options should be, in order:
      | Global Algorithm Cup |
      | Regional Code League |
      | Unplayed Cup         |
      | Women Challenge      |

  Scenario: Restrict competitions to the selected organizers
    When I list competitions organized by organizer 1
    Then the competition details should be, in order:
      | id | competition          |
      | 10 | Global Algorithm Cup |
      | 20 | Regional Code League |

  Scenario: Return no competitions for a nonexistent organizer
    When I list competitions organized by organizer 999
    Then no competition options should be returned

  Scenario: Locate institutions that participated in a competition
    When I list institutions that participated in competition 10
    Then the institution options should be, in order:
      | Alpha University |
      | Beta Institute   |
      | Gamma University |

  Scenario: Include registered entities with no participations in complete catalogs
    When I list all institutions
    Then the institution options should be, in order:
      | Ada College       |
      | Alpha University  |
      | Beta Institute    |
      | Gamma University  |
      | Idle University   |
    When I list all teams
    Then the team options should be, in order:
      | Ada Lovelace |
      | Alpha Bytes  |
      | Alpha Coders |
      | Beta Stack   |
      | Gamma Graph  |
      | Idle Team    |

  Scenario: Return no institutions for a competition with no participations
    When I list institutions that participated in competition 40
    Then no institution options should be returned

  Scenario: Locate teams by competition and institution
    When I list teams from institution 1 that participated in competition 10
    Then the team details should be, in order:
      | id | team         | institution id | institution     | short name |
      | 2  | Alpha Bytes  | 1              | Alpha University | AU         |
      | 1  | Alpha Coders | 1              | Alpha University | AU         |

  Scenario: Apply union within lists and intersection across dimensions
    When I list teams for competitions 10 and 30 and institutions 1 and 4
    Then the team options should be, in order:
      | Ada Lovelace |
      | Alpha Bytes  |
      | Alpha Coders |

  Scenario: Filter teams only by institution
    When I list teams from institution 5
    Then the team details should be, in order:
      | id | team      | institution id | institution    | short name |
      | 6  | Idle Team | 5              | Idle University | IU         |

  Scenario: Return no teams when hierarchical filters are incompatible
    When I list teams from institution 2 that participated in competition 20
    Then no team options should be returned

  Scenario: List the events and years in which an institution participated
    When I request the event history for institution 1
    Then the event history summary should be:
      | event              | competition          | year count | first year | last year |
      | Algorithm Final    | Global Algorithm Cup | 3          | 2023       | 2025      |
      | Algorithm Regional | Global Algorithm Cup | 2          | 2024       | 2025      |
      | Regional Final     | Regional Code League | 2          | 2024       | 2026      |

  Scenario: Return an empty event list for an institution with no participations
    When I request the event history for institution 5
    Then no event history should be returned
