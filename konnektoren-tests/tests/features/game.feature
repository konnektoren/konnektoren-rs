Feature: Game

  Scenario: Default game path is loaded from YAML
    Given default game path is loaded
    Then game path should be named "Konnektoren"
    And it should have at least 1 challenges