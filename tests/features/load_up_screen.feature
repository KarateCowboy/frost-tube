Feature: Fresh load of the application without any settings or anything

  Scenario: Load up the application and land on the index screen
    Given I have a new application instance
    Then the current page should be the "Index" page
