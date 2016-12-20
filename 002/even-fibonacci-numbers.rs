// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

fn naive_solution() -> u64 {
    let mut fib = (1, 1);
    let mut sum = 0;
    while fib.1 < 4_000_000 {
        fib = (fib.1, fib.0 + fib.1);
        if fib.1 % 2 == 0 {
            sum += fib.1;
        }
    }
    sum
}

#[test]
fn naive_solution_works() {
    assert_eq!(4613732, naive_solution());
}

