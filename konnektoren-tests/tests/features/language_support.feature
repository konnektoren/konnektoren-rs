Feature: Language Support

  Scenario: Challenge in German
    Given a challenge with language set to "de"
    When the challenge is loaded
    Then all text elements should be in German
    And question help text should be in German

  Scenario: Challenge in English
    Given a challenge with language set to "en"
    When the challenge is loaded
    Then all text elements should be in English
    And question help text should be in English

  Scenario: Challenge Translation
    Given a challenge in German
    When the language is changed to English
    Then the challenge should be displayed in English
    And the meaning should be preserved
