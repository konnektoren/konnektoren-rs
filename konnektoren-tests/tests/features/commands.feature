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