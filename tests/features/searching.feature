Feature: as a user, I want to search a term via the seach text field and see videos. I want to do this so I can find fun music videos to listen to from the good old animay days of the 90's and stuff

  Scenario: Search YT via invidious and see results
    Given I have a new application instance
    Then the current page should be the "Index" page
    When I search "Kaze Fuiteru"
    Then the I should see the search results entries

  Scenario: Clicking a search result navigates to the video detail page
    Given I have a new application instance
    When I search "Kaze Fuiteru"
    And I click the video "Kaze Fuiteru - Official Music Video"
    Then I should be taken to the "Video Detail" page
    And I should see the video title "Kaze Fuiteru - Official Music Video"

  Scenario: Search fails gracefully when the API returns an error
    Given I have a new application instance
    When I search and the API returns an error
    Then I should see an error message modal
