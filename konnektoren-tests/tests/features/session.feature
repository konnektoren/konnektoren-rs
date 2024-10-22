Feature: Session feature

  Scenario: Defaults on new Session
    Given A new Session with id "1"
    Then player profile id is "1"
    And player profile xp is 0

  Scenario: Create a new session with a player profile
    Given A new Session with id "1" and name "John Doe"
    Then the player profile id should be "1"
    And the player profile name should be "John Doe"

  Scenario: Update player profile name
    Given A new Session with id "1" and name "John Doe"
    When the player profile name is updated to "Jane Smith"
    Then the player profile name should be "Jane Smith"

  Scenario: Update player profile XP
    Given A new Session with id "1"
    And the player profile xp is 100
    When the player profile xp is increased by 50
    Then the player profile xp should be 150
