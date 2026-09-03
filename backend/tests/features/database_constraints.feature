Feature: Team identity and contestant registration integrity
  A team is identified by its name, institution and order-independent set of
  Contestants. Its roster is fixed once it participates. A Contestant may
  represent different teams in different competitions or years, but only one
  team in a competition-year and only one instance of an event in a year.

  Background:
    Given the backend API is running with an isolated database
    And the deterministic BDD fixture is loaded

  Scenario: Allow homonymous teams from one institution when their rosters differ
    Given member 17 exists
    And member 18 exists
    And team 101 named "Shared Name" from institution 1 has canonical Contestants 17
    And team 102 named "Shared Name" from institution 1 has canonical Contestants 18
    Then there should be 2 teams named "Shared Name" from institution 1
    And team 101 should have canonical Contestants 17
    And team 102 should have canonical Contestants 18

  Scenario: Reject a duplicate team identity independently of roster order
    Given member 17 exists
    And member 18 exists
    And team 101 named "Order Independent" from institution 1 has canonical Contestants 17, 18
    When I try to create team 102 named "Order Independent" from institution 1 with canonical Contestants 18, 17
    Then PostgreSQL should reject the duplicate team identity
    And team 102 should not exist

  Scenario: Populate every event participation from the canonical roster
    Then team event 3 should have exactly the canonical Contestants of team 1
    And team event 10 should have exactly the canonical Contestants of team 1
    And member 1 should have 2 Contestant participations for team 1 in competition 10 in 2024

  Scenario: Keep a team's canonical roster immutable after participation
    Given member 17 exists
    When I try to add member 17 to the canonical Contestants of team 1
    Then PostgreSQL should reject the canonical roster change
    And member 17 should not be a canonical Contestant of team 1

  Scenario: Keep a participating team's name and institution immutable
    When I try to rename team 1 to "Renamed Team" and move it to institution 2
    Then PostgreSQL should reject the participating team identity change
    And team 1 should still be named "Alpha Coders" at institution 1

  Scenario: Reject event-level changes that diverge from the canonical roster
    When I try to remove member 1 as Contestant from team event 3
    Then PostgreSQL should reject the event roster change
    And team event 3 should have exactly the canonical Contestants of team 1

  Scenario: Allow a Contestant to change teams across competitions and years
    Given member 17 exists
    And team 101 named "Annual Alpha" from institution 1 has canonical Contestants 17
    And team 102 named "Annual Beta" from institution 1 has canonical Contestants 17
    And team 103 named "Regional Gamma" from institution 1 has canonical Contestants 17
    When I register team 101 in event instance 1001
    And I register team 102 in event instance 1002
    And I register team 103 in event instance 2000
    Then member 17 should represent 1 team in competition 10 in 2024
    And member 17 should represent 1 team in competition 10 in 2025
    And member 17 should represent 1 team in competition 20 in 2024

  Scenario: Reject a team change within one competition-year
    Given member 17 exists
    And team 101 named "First Representation" from institution 1 has canonical Contestants 17
    And team 102 named "Second Representation" from institution 1 has canonical Contestants 17
    When I register team 101 in event instance 1001
    And I try to register team 102 in event instance 1010
    Then PostgreSQL should reject the contestant registration as a uniqueness violation
    And team 102 should not be registered in event instance 1010
    And member 17 should represent 1 team in competition 10 in 2024

  Scenario: Reject two instances of the same event in one year
    When I try to register team 1 in event instance 1003
    Then PostgreSQL should reject the duplicate event-year registration
    And team 1 should not be registered in event instance 1003

  Scenario: Apply contestant restrictions only to the Contestant role
    When I register member 1 as Coach in team event 4
    And I register member 2 as Reserve in team event 4
    Then member 1 should have role Coach in team event 4
    And member 2 should have role Reserve in team event 4
    When I try to change member 1 in team event 4 to Contestant
    Then PostgreSQL should reject the contestant registration as a uniqueness violation
    And member 1 should have role Coach in team event 4

  Scenario: Serialize concurrent team registrations for one competition-year
    Given member 17 exists
    And team 101 named "Concurrent Alpha" from institution 1 has canonical Contestants 17
    And team 102 named "Concurrent Beta" from institution 1 has canonical Contestants 17
    When concurrent transactions register teams 101 and 102 in event instances 1001 and 1010
    Then exactly one concurrent team registration should succeed
    And the rejected concurrent registration should be a uniqueness violation
    And member 17 should represent 1 team in competition 10 in 2024

  Scenario: Serialize the first participation against a concurrent roster change
    Given member 17 exists
    And member 18 exists
    And team 101 named "Concurrent Roster" from institution 1 has canonical Contestants 17
    When concurrent transactions register team 101 in event instance 1001 and add member 18 to its canonical Contestants
    Then the concurrent team registration should succeed
    And the concurrent roster change should either precede participation or be rejected as immutable
    And team 101 in event instance 1001 should have exactly its canonical Contestants
