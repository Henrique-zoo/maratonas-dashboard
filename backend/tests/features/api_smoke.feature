Feature: Hierarchical filters on the competition dashboard
  The filter cascade should help locate an entity without turning higher
  hierarchy levels into implicit constraints on the final result.

  Background:
    Given I am viewing the competition dashboard

  Scenario: Selecting an organizer delays the result refresh until filters are applied
    When I select "Algorithm League" in the organizer filter
    Then the dashboard should not navigate
    And the dashboard should wait for me to apply the filters before refreshing
    And the competition filter options should be:
      | Global Algorithm Cup |
      | Regional Code League |
    When I apply the filters
    Then the page address should represent the selected filters as "/?organizer=1"
    And the filter summary should show 2 competitions, 3 events, and 4 teams

  Scenario: Selecting a competition locates only participating institutions
    When I select "Algorithm League" in the organizer filter
    And I select "Global Algorithm Cup" in the competition filter
    Then the dashboard should not navigate
    And the dashboard should wait for me to apply the filters before refreshing
    And the institution filter options should be:
      | Alpha University |
      | Beta Institute   |
      | Gamma University |
    When I apply the filters
    Then the page address should represent the selected filters as "/?organizer=1&competition=10"
    And the filter summary should show 1 competition, 2 events, and 4 teams

  Scenario: Selecting a team displays its complete portfolio
    When I select "Algorithm League" in the organizer filter
    And I select "Global Algorithm Cup" in the competition filter
    And I select "Alpha University" in the institution filter
    Then the team filter options should be:
      | Alpha Bytes  |
      | Alpha Coders |
    When I select "Alpha Coders" in the team filter
    Then the dashboard should wait for me to apply the filters before refreshing
    When I apply the filters
    Then the page address should represent the selected filters as "/?organizer=1&competition=10&institution=1&team=1"
    And the filter summary should show 2 competitions, 2 events, and 1 team
