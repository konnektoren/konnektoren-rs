Feature: Game Progression

  Scenario: Unlocking Challenges
    Given a user with 5 XP
    When the user attempts to access a challenge requiring 10 XP
    Then access should be denied with message "Insufficient XP"

  Scenario: Successfully Unlocking Challenge
    Given a user with 5 XP
    When the user earns 5 more XP
    And attempts to access a challenge requiring 10 XP
    Then access should be granted

  Scenario: Game Path Completion
    Given a user has completed all but one challenge in a path
    When the user completes the final challenge
    Then the game path should be marked as complete
    And the user should earn a path completion bonus

  Scenario: Game Path Progress Tracking
    Given a user has completed 3 of 5 challenges in a path
    When the path progress is checked
    Then it should show 60% completion
