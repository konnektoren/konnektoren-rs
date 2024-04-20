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

  Scenario: Successfully solving a Multiple Choice Challenge
    Given a multiple choice challenge is set up with a question of option 1
    When the multiple choice challenge is solved with option 1
    Then the result performance should be at least 100
