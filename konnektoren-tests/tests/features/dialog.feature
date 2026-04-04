Feature: Dialog Challenge

  Background:
    Given a Dialog challenge in Quiz mode is loaded

  Scenario: Observer mode always awards full score
    Given a Dialog challenge in Observer mode is loaded
    When the dialog is observed
    Then the dialog performance should be 100

  Scenario: Quiz mode - all answers correct
    When the player answers all interactive turns correctly
    Then the dialog performance should be 100
    And 3 dialog answers should be recorded

  Scenario: Quiz mode - partial score
    When the player answers the first interactive turn correctly
    And the player answers the second interactive turn incorrectly
    Then the dialog performance should be 33
    And 2 dialog answers should be recorded

  Scenario: Quiz mode - correct answer is recognized
    When the player answers interactive turn 1 with option 0
    Then the answer should be correct

  Scenario: Quiz mode - wrong answer is recognized
    When the player answers interactive turn 1 with option 2
    Then the answer should be incorrect

  Scenario: Out-of-bounds turn index returns an error
    When the player answers turn 999 with option 0
    Then solving the dialog turn should return an error
