workspace "Konnektoren" "A modular learning platform for German grammar connectors" {
    !docs src
    !identifiers hierarchical

    model {
        user = person "User" "A learner using the Konnektoren platform"

        konnektoren-rs = softwareSystem "Konnektoren" "A modular learning platform for German grammar connectors" {
            core = container "Core" "Core library for Konnektoren functionality" {
                // Challenges Module
                challenges = component "Challenges" "Core challenge functionality" {
                    description "Includes Challenge Factory (creates challenge instances), Challenge Types (defines different types of challenges), and Challenge History (tracks completed challenges)."
                }

                // Game Module
                game = component "Game" "Game state and management" {
                    description "Includes Game State (maintains current game state) and Game Path (defines learning paths)."
                }

                // Commands Module
                commands = component "Commands" "Command handling system" {
                    description "Includes Command Bus (routes commands), Game Commands (game-level commands), and Challenge Commands (challenge-level commands)."
                }

                // Events Module
                events = component "Events" "Event handling system" {
                    description "Includes Event Bus (routes events), Game Events (game-level events), and Challenge Events (challenge-level events)."
                }

                // Additional Features
                certificates = component "Certificates" "Certificate generation and validation"
                achievements = component "Achievements" "Achievement system"
                analytics = component "Analytics" "Analytics and metrics"
            }

            platform = container "Platform" "Platform-specific functionality" {
                // Domain Module
                domain = component "Domain" "Domain management" {
                    description "Includes Domain Config (domain configuration) and Language Domain (language-specific domain handling)."
                }

                // I18n Module
                i18n = component "I18n" "Internationalization" {
                    description "Includes I18n Config (i18n configuration), Translation Asset (translation file handling), and Language (language management)."
                }

                // Tools Module
                tools = component "Tools" "Development tools" {
                    description "Includes I18n Checker (translation completeness checker)."
                }
            }
        }

        # Relationships
        user -> konnektoren-rs.core "Uses"
        konnektoren-rs.platform -> konnektoren-rs.core "Depends on"

        # Core Internal Relationships
        konnektoren-rs.core.challenges -> konnektoren-rs.core.game "Updates"
        konnektoren-rs.core.commands -> konnektoren-rs.core.game "Modifies"
        konnektoren-rs.core.commands -> konnektoren-rs.core.challenges "Controls"
        konnektoren-rs.core.events -> konnektoren-rs.core.game "Monitors"
        konnektoren-rs.core.events -> konnektoren-rs.core.challenges "Tracks"

        # Platform Internal Relationships
        konnektoren-rs.platform.i18n -> konnektoren-rs.platform.domain "Configures"
        konnektoren-rs.platform.tools -> konnektoren-rs.platform.i18n "Analyzes"
    }

    views {
        systemLandscape "SystemLandscape" {
            include *
            autoLayout
        }

        systemContext konnektoren-rs "KonnektorenContext" {
            include *
            autoLayout
        }

        container konnektoren-rs "KonnektorenContainers" {
            include *
            autoLayout
        }

        styles {
            element "Software System" {
                background #1168bd
                color #ffffff
            }
            element "Person" {
                shape person
                background #08427b
                color #ffffff
            }
            element "Container" {
                background #438dd5
                color #ffffff
            }
            element "Component" {
                background #85bbf0
                color #000000
            }
        }
    }
}
