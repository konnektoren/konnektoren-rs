Feature: Game

  Scenario: Default game path is loaded from YAML
    Given default game path is loaded
    Then game path should be named "Konnektoren"
    And it should have at least 1 challenges

  Scenario: Default game is loaded
    Given default game is loaded
    And a multiple choice challenge is set up with a question of option 1
    When the multiple choice challenge is solved with option 1
    Then the result performance should be at least 100
    And the challenge history should have at least 1 entry

  Scenario: List all challenges
    Given default game is loaded
    Then the game should have at least 2 challenges
