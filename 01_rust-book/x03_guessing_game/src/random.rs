use std::time::{SystemTime, UNIX_EPOCH};

pub fn rand() -> i64 {
    let mut state = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u128;

    state = state
        .wrapping_mul(6364136223846793005_u128)
        .wrapping_add(1442695040888963407_u128);

    let result = (state >> 64) as i64;

    result.abs()
}
