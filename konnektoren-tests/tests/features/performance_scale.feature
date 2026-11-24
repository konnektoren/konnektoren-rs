Feature: Performance Scale
  A performance (0–100 %) is judged on a scale: not passed, passed, good or mastered,
  with 0 to 3 stars. The default passes from 60 %, is good from 70 % and mastered from
  80 %. Apps can override each threshold.

  # ============================================================================
  # Bands and stars
  # ============================================================================

  Scenario Outline: Default scale judges a performance
    Given the default performance scale
    When a performance of <performance>% is judged
    Then the performance band should be "<band>"
    And the performance should earn <stars> stars

    Examples:
      | performance | band      | stars |
      | 0           | NotPassed | 0     |
      | 59          | NotPassed | 0     |
      | 60          | Passed    | 1     |
      | 69          | Passed    | 1     |
      | 70          | Good      | 2     |
      | 79          | Good      | 2     |
      | 80          | Mastered  | 3     |
      | 100         | Mastered  | 3     |

  Scenario: An app passes from a lower threshold
    Given an app scale that passes from 50%
    When a performance of 55% is judged
    Then the performance band should be "Passed"
    And the performance should earn 1 stars

  Scenario: An app masters from a higher threshold
    Given an app scale that masters from 90%
    When a performance of 85% is judged
    Then the performance band should be "Good"
    And the performance should earn 2 stars

  # ============================================================================
  # XP follows the stars
  # ============================================================================

  Scenario Outline: XP reward is performance times stars
    Given a user completes a challenge with <performance>% performance
    Then the XP reward for the last attempt should be <xp>

    Examples:
      | performance | xp  |
      | 50          | 0   |
      | 60          | 60  |
      | 70          | 140 |
      | 100         | 300 |

  # ============================================================================
  # Analytics: success means mastered
  # ============================================================================

  Scenario: Success rate counts mastered attempts
    Given a user completes a challenge with 100% performance
    And a user completes a challenge with 70% performance
    And a user completes a challenge with 80% performance
    And a user completes a challenge with 50% performance
    Then the success rate should be 50%

  Scenario: Success rate follows the app scale
    Given a user completes a challenge with 100% performance
    And a user completes a challenge with 70% performance
    And a user completes a challenge with 80% performance
    And a user completes a challenge with 50% performance
    And an app scale that masters from 70%
    Then the success rate should be 75%

  # ============================================================================
  # Achievements: passed and mastered challenges
  # ============================================================================

  Scenario: Statistics count passed and mastered attempts
    Given a user completes a challenge with 90% performance
    And a user completes a challenge with 70% performance
    And a user completes a challenge with 50% performance
    Then the user should have 2 passed and 1 mastered challenges

  Scenario: Achievement unlocks with mastered challenges
    Given a user completes a challenge with 90% performance
    And a user completes a challenge with 70% performance
    And an achievement with the condition "mastered_challenges >= 1 & passed_challenges >= 2"
    Then the achievement should be unlocked

  Scenario: Achievement stays locked on a stricter app scale
    Given a user completes a challenge with 90% performance
    And a user completes a challenge with 70% performance
    And an achievement with the condition "mastered_challenges >= 1"
    And an app scale that masters from 95%
    Then the achievement should not be unlocked
