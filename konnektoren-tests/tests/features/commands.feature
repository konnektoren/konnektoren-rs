Feature: Commands

  Scenario: Next challenge
    Given A new Session with id "1"
    And the current challenge is "konnektoren-1"
    When the next challenge is requested
    Then the current challenge is "konnektoren-2"

  Scenario: Previous challenge
    Given A new Session with id "1"
    And the current challenge is "konnektoren-1"
    When the next challenge is requested
    And the next challenge is requested
    And the previous challenge is requested
    Then the current challenge is "konnektoren-2"

  Scenario: Next challenge at the end
    Given A new Session with id "1"
    And the current challenge is the last challenge
    When the next challenge is requested
    Then an error should be raised with message "No more challenges"

  Scenario: Previous challenge at the beginning
    Given A new Session with id "1"
    And the current challenge is the first challenge
    When the previous challenge is requested
    Then an error should be raised with message "No previous challenges"

  Scenario: Solve challenge option
    Given A new Session with id "1"
    And the current challenge is "konnektoren-1"
    And the current task index is 0
    When the challenge is solved with option 0
    Then the current task index should be 1

  Scenario: Next task
    Given A new Session with id "1"
    And the current challenge is "konnektoren-1"
    And the current task index is 0
    When the next task is requested
    Then the current task index should be 1

  Scenario: Previous task
    Given A new Session with id "1"
    And the current challenge is "konnektoren-1"
    And the current task index is 1
    When the previous task is requested
    Then the current task index should be 0

  Scenario: Next task at the end
    Given A new Session with id "1"
    And the current challenge is "konnektoren-1"
    And the current task index is the last task index
    When the next task is requested
    Then an error should be raised with message "No more tasks"

  Scenario: Previous task at the beginning
    Given A new Session with id "1"
    And the current challenge is "konnektoren-1"
    And the current task index is 0
    When the previous task is requested
    Then an error should be raised with message "No previous tasks"
