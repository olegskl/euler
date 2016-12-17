// Find the sum of all the multiples of 3 or 5 below 1000.

// O(n)
fn naive_solution() -> u64 {
    let mut sum = 0;
    for n in 0..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}

#[test]
fn naive_solution_is_correct() {
    assert_eq!(233168, naive_solution());
}

fn arithmetic_series(divisor: u64) -> u64 {
    let n = 999 / divisor;
    divisor * n * (n + 1) / 2
}

// O(1)
fn optimal_solution() -> u64 {
    arithmetic_series(3) + arithmetic_series(5) - arithmetic_series(3 * 5)
}

#[test]
fn optimal_solution_is_correct() {
    assert_eq!(233168, optimal_solution());
}
