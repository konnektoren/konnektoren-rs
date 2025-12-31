Feature: Contextual Choice Challenge Navigation

  Scenario: Navigate through contextual choice tasks
    Given A new Session with id "1"
    And a contextual choice challenge with 4 items is loaded
    And the current task index is 0
    When the next task is requested
    Then the current task index should be 1
    And 1 tasks should be completed
    When the next task is requested
    Then the current task index should be 2
    And 2 tasks should be completed

  Scenario: Solve contextual choice tasks and navigate
    Given A new Session with id "1"
    And a contextual choice challenge with 3 items is loaded
    And the current task index is 0
    When the contextual choice task is solved correctly
    Then the current task index should be 1
    And 1 tasks should be completed
    When the previous task is requested
    Then the current task index should be 0
    When the next task is requested
    Then the current task index should be 1

  Scenario: Skip contextual choice tasks maintains alignment
    Given A new Session with id "1"
    And a contextual choice challenge with 4 items is loaded
    And the current task index is 0
    When the next task is requested
    When the next task is requested
    Then 2 tasks should be completed
    When the previous task is requested
    Then the current task index should be 1
    When the contextual choice task is solved correctly
    Then 2 tasks should be completed
    And task 1 should be answered correctly for contextual choice
