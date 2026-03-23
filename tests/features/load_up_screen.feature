Feature: Fresh load of the application without any settings or anything

  Scenario: Load up the application and land on the index screen
    Given I have a new application instance
    Then the current page should be the "Index" page

  @focus
  Scenario: Land on the index screen
    Given I have a new application instance
    And I am on the "Index" page
    Then I should see a text field with a label of "Search"
    When I click the button "Go"
    Then I should be taken to the "Search Results" page

