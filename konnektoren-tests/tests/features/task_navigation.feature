Feature: Task Navigation

  Scenario: Navigate through tasks with exact count
    Given A new Session with id "1"
    And the current challenge is "konnektoren-1"
    And the current task index is 0
    When the next task is requested
    Then the current task index should be 1
    When the next task is requested
    Then the current task index should be 2

  Scenario: Navigate through tasks with range pattern
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..25" is loaded
    And the current task index is 0
    When the challenge is loaded
    Then the current task should be valid
    And the task count should be 16
    When the next task is requested
    Then the current task should be valid
    When the next task is requested
    Then the current task should be valid

  Scenario: Solve all tasks in range pattern
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..25" is loaded
    And the current task index is 0
    When all tasks in the range are solved
    Then 16 tasks should be completed
    And all completed tasks should have valid answers

  Scenario: Task index bounds with range pattern
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..25" is loaded
    And the current task index is at the last task in range
    When the next task is requested
    Then an error should be raised with message "No more tasks"

  Scenario: Navigate backwards through range pattern
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..25" is loaded
    And the current task index is 5
    When the previous task is requested
    Then the current task index should be 4
    When the previous task is requested
    Then the current task index should be 3

  Scenario: Random task selection
    Given A new Session with id "1"
    And a konnektoren challenge with random pattern "5:10..25" is loaded
    Then the task count should be 5
    When all tasks in the range are solved
    Then 5 tasks should be completed

  Scenario: Exact task count
    Given A new Session with id "1"
    And a konnektoren challenge with exact pattern "10" is loaded
    Then the task count should be 10
    When all tasks in the range are solved
    Then 10 tasks should be completed

  Scenario: Navigate through exact tasks
    Given A new Session with id "1"
    And a konnektoren challenge with exact pattern "3" is loaded
    And the current task index is 0
    When the next task is requested
    Then the current task index should be 1
    When the next task is requested
    Then the current task index should be 2
    When the next task is requested
    Then an error should be raised with message "No more tasks"

  Scenario: Verify range pattern questions are correct subset
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..12" is loaded
    Then the task count should be 3
    And the first question should be "als ob"
    And the second question should be "als wenn"
    And the third question should be "anstatt dass"

  Scenario: Answer first task, navigate forward, then backward to verify consistency
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..13" is loaded
    And the current task index is 0
    And the current question is "als ob"
    When the task is solved correctly
    Then the current task index should be 1
    And the current question is "als wenn"
    When the next task is requested
    Then the current task index should be 2
    And the current question is "anstatt dass"
    When the task is solved correctly
    Then the current task index should be 3
    And the current question is "auch wenn"
    And 3 tasks should be completed
    When the previous task is requested
    Then the current task index should be 2
    When the previous task is requested
    Then the current task index should be 1
    And the current question is "als wenn"
    When the task is solved correctly
    Then the current task index should be 2
    And 3 tasks should be completed
    When the next task is requested
    Then the current task index should be 3
    And the current question is "auch wenn"
    When the task is solved correctly
    Then the current task index should be 3
    And 4 tasks should be completed
    And all completed tasks should have valid answers

  Scenario: Non-sequential task solving with navigation
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..14" is loaded
    And the current task index is 0
    When the task is solved correctly
    And the next task is requested
    And the next task is requested
    And the task is solved correctly
    Then 4 tasks should be completed
    When the previous task is requested
    And the previous task is requested
    And the task is solved correctly
    Then 4 tasks should be completed
    And task 0 should be answered
    And task 1 should be answered
    And task 2 should be answered

  Scenario: Verify task answers persist after navigation
    Given A new Session with id "1"
    And a konnektoren challenge with exact pattern "5" is loaded
    When task 0 is solved correctly
    And task 1 is solved correctly
    And task 2 is solved correctly
    Then 3 tasks should be completed
    When the previous task is requested
    Then the current task index should be 2
    And the task should show as already answered
    When the previous task is requested
    Then the current task index should be 1
    And the task should show as already answered
    When the previous task is requested
    Then the current task index should be 0
    And the task should show as already answered

  Scenario: Verify range pattern questions are correct subset
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..13" is loaded
    Then the task count should be 4
    And the first question should be "als ob"
    And the second question should be "als wenn"
    And the third question should be "anstatt dass"

  Scenario: Verify solutions are marked as correct or incorrect
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..13" is loaded
    And the current task index is 0
    And the current question is "als ob"
    When the task is solved correctly
    Then the current task index should be 1
    And task 0 should be answered correctly
    When the previous task is requested
    Then the current task index should be 0
    When the task is solved incorrectly
    Then the current task index should be 1
    And task 0 should be answered incorrectly
    When the next task is requested
    Then the current task index should be 2
    And the current question is "anstatt dass"
    When the task is solved correctly
    Then the current task index should be 3
    And task 2 should be answered correctly
    And 3 tasks should be completed
    And the challenge performance should be 25

  Scenario: Quickfix - Navigate without answering adds default answer
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..13" is loaded
    And the current task index is 0
    When the next task is requested
    Then the current task index should be 1
    And 1 tasks should be completed
    And task 0 should be answered
    When the next task is requested
    Then the current task index should be 2
    And 2 tasks should be completed
    And task 0 should be answered
    And task 1 should be answered
    When the task is solved correctly
    Then the current task index should be 3
    And 3 tasks should be completed
    And task 2 should be answered correctly

  Scenario: Quickfix - Skip tasks then go back maintains alignment
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..13" is loaded
    And the current task index is 0
    When the next task is requested
    When the next task is requested
    When the next task is requested
    Then the current task index should be 3
    And 3 tasks should be completed
    When the previous task is requested
    Then the current task index should be 2
    And the current question is "anstatt dass"
    When the task is solved correctly
    Then 3 tasks should be completed
    And task 2 should be answered correctly

  Scenario: Verify correctness check works after skipping tasks
    Given A new Session with id "1"
    And a konnektoren challenge with range pattern "10..13" is loaded
    And the current task index is 0
    And the current question is "als ob"
    When the next task is requested
    Then 1 tasks should be completed
    And the current task index should be 1
    And the current question is "als wenn"
    When the task is solved correctly
    Then 2 tasks should be completed
    And the current task index should be 2
    And task 0 should be answered
    And task 1 should be answered correctly
    When the next task is requested
    Then 3 tasks should be completed
    And the current task index should be 3
    And the current question is "auch wenn"
    When the task is solved correctly
    Then 4 tasks should be completed
    And task 3 should be answered correctly
    And all completed tasks should have valid answers
