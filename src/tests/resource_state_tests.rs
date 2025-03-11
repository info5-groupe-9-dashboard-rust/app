#[cfg(test)]
mod tests {
    use crate::models::data_structure::resource::ResourceState;
    use std::fmt::Write;

    #[test]
    /// Checks that the Display implementation works correctly
    fn test_resource_state_display() {
        // Test each possible state value
        assert_eq!(ResourceState::Dead.to_string(), "Dead");
        assert_eq!(ResourceState::Alive.to_string(), "Alive");
        assert_eq!(ResourceState::Absent.to_string(), "Absent");
        assert_eq!(ResourceState::Unknown.to_string(), "Unknown");

        // Test with format! to verify fmt::Display implementation
        let mut output = String::new();
        write!(output, "State: {}", ResourceState::Alive).unwrap();
        assert_eq!(output, "State: Alive");
    }

    #[test]
    /// Checks that the Clone implementation works correctly
    fn test_resource_state_clone() {
        // Test for each state value
        let state1 = ResourceState::Dead;
        let state2 = state1.clone();
        assert!(matches!(state2, ResourceState::Dead));

        let state1 = ResourceState::Alive;
        let state2 = state1.clone();
        assert!(matches!(state2, ResourceState::Alive));

        let state1 = ResourceState::Absent;
        let state2 = state1.clone();
        assert!(matches!(state2, ResourceState::Absent));

        let state1 = ResourceState::Unknown;
        let state2 = state1.clone();
        assert!(matches!(state2, ResourceState::Unknown));
    }

    #[test]
    /// Checks that equality works correctly
    fn test_resource_state_equality() {
        assert_eq!(ResourceState::Dead, ResourceState::Dead);
        assert_eq!(ResourceState::Alive, ResourceState::Alive);
        assert_eq!(ResourceState::Absent, ResourceState::Absent);
        assert_eq!(ResourceState::Unknown, ResourceState::Unknown);

        assert_ne!(ResourceState::Dead, ResourceState::Alive);
        assert_ne!(ResourceState::Dead, ResourceState::Absent);
        assert_ne!(ResourceState::Dead, ResourceState::Unknown);
        assert_ne!(ResourceState::Alive, ResourceState::Absent);
        assert_ne!(ResourceState::Alive, ResourceState::Unknown);
        assert_ne!(ResourceState::Absent, ResourceState::Unknown);
    }

    #[test]
    /// Checks that copy works correctly (ResourceState implements Copy)
    fn test_resource_state_copy() {
        let state1 = ResourceState::Alive;
        let state2 = state1; // Copy semantics

        // If Copy is correctly implemented, both state1 and state2 are valid
        assert_eq!(state1, ResourceState::Alive);
        assert_eq!(state2, ResourceState::Alive);
    }

    #[test]
    /// Checks conversions from a string
    fn test_resource_state_from_string() {
        // Utility function simulating the behavior observed in application_context.rs
        fn string_to_resource_state(state_str: &str) -> ResourceState {
            match state_str {
                "Dead" => ResourceState::Dead,
                "Alive" => ResourceState::Alive,
                "Absent" => ResourceState::Absent,
                _ => ResourceState::Unknown,
            }
        }

        assert_eq!(string_to_resource_state("Dead"), ResourceState::Dead);
        assert_eq!(string_to_resource_state("Alive"), ResourceState::Alive);
        assert_eq!(string_to_resource_state("Absent"), ResourceState::Absent);
        assert_eq!(string_to_resource_state("Unknown"), ResourceState::Unknown);

        // Special cases
        assert_eq!(string_to_resource_state(""), ResourceState::Unknown);
        assert_eq!(string_to_resource_state("invalid"), ResourceState::Unknown);
        assert_eq!(string_to_resource_state("ALIVE"), ResourceState::Unknown); // Case sensitive
    }

    #[test]
    /// Checks the use of ResourceState in a collection context
    fn test_resource_state_in_collection() {
        let states = vec![
            ResourceState::Dead,
            ResourceState::Alive,
            ResourceState::Absent,
            ResourceState::Unknown,
        ];

        assert_eq!(states.len(), 4);
        assert!(states.contains(&ResourceState::Dead));
        assert!(states.contains(&ResourceState::Alive));
        assert!(states.contains(&ResourceState::Absent));
        assert!(states.contains(&ResourceState::Unknown));
    }
}
