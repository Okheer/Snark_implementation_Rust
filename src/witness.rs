use ark_bn254::Fr;
use std::sync::LazyLock;

pub fn WITNESS() -> [Fr; 3] {
    // No LazyLock needed, and NO semicolon at the end!
    [1u64.into(), 16u64.into(), 4u64.into()]
}
//using direct fn or global catching comes down to efficiency. If u had to call fn thousand time, compputer executes the program inside that fn 1000 times, but static assigns the value