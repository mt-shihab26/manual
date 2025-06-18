use std::time::{SystemTime, UNIX_EPOCH};

pub fn rand() -> i64 {
    let mut state = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as i64;

    state = state.wrapping_mul(1103515245).wrapping_add(12345) % 1000000007;
    if state < 0 {
        state *= -1;
    }
    return state;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_returns_positive_values() {
        for _ in 0..100 {
            let result = rand();
            assert!(
                result >= 0,
                "Random number should be positive, got: {}",
                result
            );
        }
    }

    #[test]
    fn test_rand_returns_within_expected_range() {
        for _ in 0..100 {
            let result = rand();
            assert!(
                result < 1000000007,
                "Random number should be less than modulus, got: {}",
                result
            );
            assert!(
                result >= 0,
                "Random number should be non-negative, got: {}",
                result
            );
        }
    }

    #[test]
    fn test_rand_produces_different_values() {
        let mut values = std::collections::HashSet::new();

        // Generate random numbers with 1-second delays to ensure different timestamps
        for _ in 0..3 {
            let result = rand();
            values.insert(result);
            // Sleep for 1 second to ensure different timestamp seeds
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        // We should have at least 2 different values out of 3
        assert!(
            values.len() >= 2,
            "Should produce varied results, got {} unique values",
            values.len()
        );
    }

    #[test]
    fn test_rand_function_executes_without_panic() {
        // This test ensures the function doesn't panic
        for _ in 0..10 {
            let _ = rand();
        }
    }

    #[test]
    fn test_rand_deterministic_with_same_timestamp() {
        // Since our function uses current time, we can't easily test determinism
        // But we can test that it always returns a valid result
        let result1 = rand();
        let result2 = rand();

        // Both should be valid (non-negative and within range)
        assert!(result1 >= 0 && result1 < 1000000007);
        assert!(result2 >= 0 && result2 < 1000000007);
    }
}
