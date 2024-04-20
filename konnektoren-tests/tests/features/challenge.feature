Feature: Challenge

  Scenario: Loading a default challenge from YAML
    Given default challenge is loaded
    Then it should be a MultipleChoice challenge named "Konnektoren"
    And it should have exactly 5 options
    And it should have at least 1 questions
