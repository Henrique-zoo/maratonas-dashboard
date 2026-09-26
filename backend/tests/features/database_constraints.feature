@isolated_database
Feature: Team identity and contestant registration integrity
  A team is identified by its name, institution, and order-independent official
  roster. Once the team participates, its identity is fixed. A contestant may
  represent different teams in different competition-years, but only one team
  in each competition-year and only one occurrence of an event in each year.

  Scenario: Allow homonymous teams from one institution when their rosters differ
    Given member 17 exists
    And member 18 exists
    And team 101 named "Shared Name" from institution 1 has official roster 17
    And team 102 named "Shared Name" from institution 1 has official roster 18
    Then there should be 2 teams named "Shared Name" from institution 1
    And team 101 should have official roster 17
    And team 102 should have official roster 18

  Scenario: Reject a duplicate team identity independently of roster order
    Given member 17 exists
    And member 18 exists
    And team 101 named "Order Independent" from institution 1 has official roster 17, 18
    When I try to create team 102 named "Order Independent" from institution 1 with official roster 18, 17
    Then the duplicate team identity should be rejected
    And team 102 should not exist

  Scenario: Populate every event participation from the official roster
    Then participation 3 should have exactly the official roster of team 1
    And participation 10 should have exactly the official roster of team 1
    And member 1 should have 2 contestant participations for team 1 in competition 10 in 2024

  Scenario: Keep a team's official roster immutable after participation
    Given member 17 exists
    When I try to add member 17 to team 1's official roster
    Then the roster change should be rejected because the team has participated
    And member 17 should not belong to team 1's official roster

  Scenario: Keep a participating team's name and institution immutable
    When I try to rename team 1 to "Renamed Team" and move it to institution 2
    Then the identity change should be rejected because the team has participated
    And team 1 should still be named "Alpha Coders" at institution 1

  Scenario: Reject event-level changes that diverge from the official roster
    When I try to remove contestant 1 from participation 3
    Then the participation roster change should be rejected
    And participation 3 should have exactly the official roster of team 1

  Scenario: Allow a contestant to change teams across competitions and years
    Given member 17 exists
    And team 101 named "Annual Alpha" from institution 1 has official roster 17
    And team 102 named "Annual Beta" from institution 1 has official roster 17
    And team 103 named "Regional Gamma" from institution 1 has official roster 17
    When team 101 participates in event occurrence 1001
    And team 102 participates in event occurrence 1002
    And team 103 participates in event occurrence 2000
    Then member 17 should represent 1 team in competition 10 in 2024
    And member 17 should represent 1 team in competition 10 in 2025
    And member 17 should represent 1 team in competition 20 in 2024

  Scenario: Reject a team change within one competition-year
    Given member 17 exists
    And team 101 named "First Representation" from institution 1 has official roster 17
    And team 102 named "Second Representation" from institution 1 has official roster 17
    When team 101 participates in event occurrence 1001
    And I try to register team 102 in event occurrence 1010
    Then the registration should be rejected because the contestant already represents a team in that competition-year
    And team 102 should not participate in event occurrence 1010
    And member 17 should represent 1 team in competition 10 in 2024

  Scenario: Reject two instances of the same event in one year
    When I try to register team 1 in event occurrence 1003
    Then the registration should be rejected because a contestant already entered that event in the year
    And team 1 should not participate in event occurrence 1003

  Scenario: Apply contestant restrictions only to the Contestant role
    When member 1 serves as Coach in participation 4
    And member 2 serves as Reserve in participation 4
    Then member 1 should have role Coach in participation 4
    And member 2 should have role Reserve in participation 4
    When I try to change member 1 in participation 4 to Contestant
    Then the registration should be rejected because the contestant already represents a team in that competition-year
    And member 1 should have role Coach in participation 4

  Scenario: Serialize concurrent team registrations for one competition-year
    Given member 17 exists
    And team 101 named "Concurrent Alpha" from institution 1 has official roster 17
    And team 102 named "Concurrent Beta" from institution 1 has official roster 17
    When teams 101 and 102 are registered simultaneously in event occurrences 1001 and 1010
    Then exactly one simultaneous team registration should succeed
    And the other registration should be rejected because the contestant already represents a team in that competition-year
    And member 17 should represent 1 team in competition 10 in 2024

  Scenario: Serialize the first participation against a concurrent roster change
    Given member 17 exists
    And member 18 exists
    And team 101 named "Concurrent Roster" from institution 1 has official roster 17
    When team 101 is registered in event occurrence 1001 while member 18 is simultaneously added to its official roster
    Then the simultaneous team registration should succeed
    And the roster change should either precede participation or be rejected as immutable
    And team 101 in event occurrence 1001 should have exactly its official roster
