Feature: Session feature

  Scenario: Defaults on new Session
    Given A new Session with id "1"
    Then player profile name is "Anonymous"
    And player profile id is "1"
    And player profile xp is 0
