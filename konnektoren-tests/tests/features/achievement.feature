Feature: Achievement System

  Scenario: Achievement with multiple criteria
    Given a user has completed 40 challenges
    And the user has 0 achievements
    When the user completes 10 more challenges
    Then the "Challenge Champion" achievement should be unlocked
    And the achievement count should be 1
