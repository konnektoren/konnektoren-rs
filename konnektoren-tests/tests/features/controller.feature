Feature: Controller
  These tests verify the functionality of the game controller

  Scenario: Controller Executes Next Challenge Command
    Given a new controller is initialized
    When the controller executes the "NextChallenge" game command
    Then the controller's current challenge index should be 1

  Scenario: Controller Executes Previous Challenge Command
    Given a new controller is initialized
    And the controller's current challenge index is 2
    When the controller executes the "PreviousChallenge" game command
    Then the controller's current challenge index should be 1

  Scenario: Controller Executes Next Task Command
    Given a new controller is initialized
    When the controller executes the "NextTask" challenge command
    Then the controller's current task index should be 1

  Scenario: Controller Executes Previous Task Command
    Given a new controller is initialized
    And the controller's current task index is 2
    When the controller executes the "PreviousTask" challenge command
    Then the controller's current task index should be 1

  Scenario: Controller Handles Solving Option
    Given a new controller is initialized
    When the controller executes the "SolveOption" challenge command with option 0
    Then the controller's challenge result should have 1 answer

  Scenario: Controller Executes Commands at Boundaries
    Given a new controller is initialized
    And the controller's current challenge is the last challenge
    When the controller executes the "NextChallenge" game command
    Then a controller error should be raised with message "No more challenges"
    Given a new controller is initialized
    And the controller's current task is the last task
    When the controller executes the "NextTask" challenge command
    Then a controller error should be raised with message "No more tasks"

  Scenario: Controller Tracks Challenge History
    Given a new controller is initialized
    When the controller executes the "Finish" challenge command
    Then the controller's challenge history should have 1 entry
