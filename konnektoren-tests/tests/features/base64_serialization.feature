Feature: Base64 Serialization

  Scenario: Encode and decode a challenge to/from base64
    Given a multiple choice challenge is set up with a question of option 1
    When the challenge is encoded to base64
    Then the base64 string should not be empty
    When the base64 string is decoded back to a challenge
    Then the decoded challenge should match the original challenge

  Scenario: Decode invalid base64 data
    Given invalid base64 data "This is not base64!"
    When attempting to decode the base64 data
    Then a base64 decode error should be raised

  Scenario: Decode valid base64 with invalid YAML
    Given valid base64 data containing invalid YAML
    When attempting to decode the base64 data
    Then a deserialization error should be raised

  Scenario: Load challenge from base64 in factory
    Given the challenge factory is initialized
    And a default challenge is loaded to the factory
    When the first challenge is exported to base64
    And a new factory is created
    And the base64 challenge is imported to the new factory
    Then the new factory should have the imported challenge

  Scenario: Round-trip serialization preserves challenge data
    Given default challenge is loaded
    When the challenge is serialized to base64 and back
    Then all challenge properties should be preserved
    And the challenge should have the same id
    And the challenge should have the same name
    And the challenge should have the same number of options
    And the challenge should have the same number of questions
