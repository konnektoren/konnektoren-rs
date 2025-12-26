Feature: Performance Calculation and Recording

  Background:
    Given the system tracks best performance for each challenge
  # ============================================================================
  # Single Challenge Performance Tests
  # ============================================================================

  Scenario: Vocabulary Challenge Shows 100% When Completed
    Given a user starts a vocabulary challenge
    When the user completes the vocabulary challenge
    Then the challenge should show 100% performance
    And the performance record should show 100% for that challenge

  Scenario: Multiple Choice Challenge Shows Exact Performance
    Given a user starts a multiple choice challenge with 10 questions
    When the user answers 9 questions correctly and 1 incorrectly
    Then the challenge should show 90% performance
    And the performance record should show 90% for that challenge

  Scenario: Multiple Choice Challenge With 50% Performance
    Given a user starts a multiple choice challenge with 10 questions
    When the user answers 5 questions correctly and 5 incorrectly
    Then the challenge should show 50% performance
    And the performance record should show 50% for that challenge

  Scenario: Informative Challenge Always Shows 100%
    Given a user starts an informative challenge
    When the user completes the informative challenge
    Then the challenge should show 100% performance
    And the performance record should show 100% for that challenge

  Scenario: Empty Challenge Shows 0% Performance
    Given a user starts a challenge
    When the user abandons the challenge without answers
    Then the challenge should show 0% performance
  # ============================================================================
  # Best Performance Tracking Tests
  # ============================================================================

  Scenario: Performance Record Tracks Best Attempt
    Given a user completes a challenge with 50% performance
    When the user completes the same challenge with 90% performance
    And the user completes the same challenge with 70% performance
    Then the performance record should show 90% for that challenge
    And the performance record should have 1 unique challenge

  Scenario: Performance Record Uses Best Time For Same Performance
    Given a user completes a challenge with 90% performance in 10 seconds
    When the user completes the same challenge with 90% performance in 5 seconds
    Then the performance record should show 90% for that challenge
    And the performance record should show 5 seconds for that challenge
  # ============================================================================
  # Overall Performance Calculation Tests (Average of Best)
  # ============================================================================

  Scenario: Overall Performance Is Average Of Best Performances
    Given a user completes challenge A with 100% performance
    And the user completes challenge B with 80% performance
    When the performance record is calculated
    Then the overall performance should be 90%

  Scenario: Multiple Attempts Use Only Best Performance In Average
    Given a user completes challenge A with 50% performance
    And the user completes challenge A with 100% performance
    And the user completes challenge B with 80% performance
    When the performance record is calculated
    Then the overall performance should be 90%
    And the performance record should have 2 unique challenges

  Scenario: Three Challenges Average Correctly
    Given a user completes a vocabulary challenge with 100% performance
    And the user completes a multiple choice challenge with 50% performance
    And the user completes an informative challenge with 100% performance
    When the performance record is calculated
    Then the overall performance should be 83%
    And the performance record should have 3 unique challenges
  # ============================================================================
  # Different Challenge Types - 50% Tests
  # ============================================================================

  Scenario: Contextual Choice Challenge Shows 50%
    Given a contextual choice challenge with 6 items
    When the user answers 3 items correctly and 3 incorrectly
    Then the challenge should show 50% performance

  Scenario: Ordering Challenge Shows 50%
    Given an ordering challenge with 8 items
    When the user orders 4 items correctly and 4 incorrectly
    Then the challenge should show 50% performance

  Scenario: SortTable Challenge Shows 50%
    Given a sort table challenge with 10 rows
    When the user sorts 5 rows correctly and 5 incorrectly
    Then the challenge should show 50% performance
  # ============================================================================
  # Edge Cases
  # ============================================================================

  Scenario: Different Question Counts Show Correct 50%
    Given a user starts a multiple choice challenge with 4 questions
    When the user answers 2 questions correctly and 2 incorrectly
    Then the challenge should show 50% performance
    Given a user starts a multiple choice challenge with 20 questions
    When the user answers 10 questions correctly and 10 incorrectly
    Then the challenge should show 50% performance

  Scenario: Perfect Score Across Multiple Challenge Types
    Given a user completes a vocabulary challenge with 100% performance
    And the user completes a multiple choice challenge with 100% performance
    And the user completes an informative challenge with 100% performance
    When the performance record is calculated
    Then the overall performance should be 100%
    And the performance record should have 3 unique challenges

  Scenario: Failed Attempts Don't Lower Best Performance
    Given a user completes a challenge with 100% performance
    When the user completes the same challenge with 0% performance
    And the user completes the same challenge with 50% performance
    Then the performance record should show 100% for that challenge
    And the performance record should have 1 unique challenge
  # ============================================================================
  # Leaderboard Consistency Tests
  # ============================================================================

  Scenario: Leaderboard Shows Same As Performance Record
    Given a user starts a multiple choice challenge with 10 questions
    When the user answers 7 questions correctly and 3 incorrectly
    Then the challenge should show 70% performance
    And the performance record should show 70% for that challenge
    And the leaderboard should show 70% for that challenge

  Scenario: Leaderboard Reflects Best Performance
    Given a user completes a challenge with 60% performance
    When the user completes the same challenge with 90% performance
    Then the leaderboard should show 90% for that challenge
