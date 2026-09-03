Feature: Hierarchical filters on the home page
  The filter cascade should help locate an entity without turning higher
  hierarchy levels into implicit constraints on the final result.

  Background:
    Given the backend API is running with an isolated database
    And the deterministic BDD fixture is loaded
    And the home page filter context is loaded
    And I am on the home page

  Scenario: Selecting an organizer updates only the local context
    Given an organizer named "Algorithm League" exists
    When I select "Algorithm League" in the organizer filter
    Then I should remain on page "/"
    And the selection should not send a request to the backend
    And the competition filter options should be:
      | Global Algorithm Cup |
      | Regional Code League |
    When I click Apply Filters
    Then I should be on page "/?organizer=1"
    And I should see 2 competitions in the filter summary
    And I should see 3 events in the filter summary
    And I should see 4 teams in the filter summary

  Scenario: Selecting a competition locates only participating institutions
    Given an organizer named "Algorithm League" exists
    And a competition named "Global Algorithm Cup" exists
    When I select "Algorithm League" in the organizer filter
    And I select "Global Algorithm Cup" in the competition filter
    Then I should remain on page "/"
    And the selection should not send a request to the backend
    And the institution filter options should be:
      | Alpha University |
      | Beta Institute   |
      | Gamma University |
    When I click Apply Filters
    Then I should be on page "/?organizer=1&competition=10"
    And I should see 1 competition in the filter summary
    And I should see 2 events in the filter summary
    And I should see 4 teams in the filter summary

  Scenario: Selecting a team displays its complete portfolio
    Given an organizer named "Algorithm League" exists
    And a competition named "Global Algorithm Cup" exists
    And an institution named "Alpha University" exists
    And a team named "Alpha Coders" exists
    When I select "Algorithm League" in the organizer filter
    And I select "Global Algorithm Cup" in the competition filter
    And I select "Alpha University" in the institution filter
    Then the team filter options should be:
      | Alpha Bytes  |
      | Alpha Coders |
    When I select "Alpha Coders" in the team filter
    Then the selection should not send a request to the backend
    When I click Apply Filters
    Then I should be on page "/?organizer=1&competition=10&institution=1&team=1"
    And I should see 2 competitions in the filter summary
    And I should see 2 events in the filter summary
    And I should see 1 team in the filter summary
