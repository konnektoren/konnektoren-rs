Feature: Challenge

  Scenario: Loading a default challenge from YAML
    Given default challenge is loaded
    Then it should be a MultipleChoice challenge named "Konnektoren"
    And it should have exactly 5 options
    And it should have at least 1 questions

  Scenario: Create a challenge from default configuration
    Given the challenge factory is initialized
    And a default challenge is loaded to the factory
    When a challenge of "konnektoren" is created with 5 questions
    Then the challenge should have exactly 5 questions
    And the challenge be identified as "konnektoren"