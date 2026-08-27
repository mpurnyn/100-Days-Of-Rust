// copywrite Marat Purnyn 2026

fn next_prime(n: i32) -> i32 {
    let mut candidate = n;
    // time complexity is O(sqrt(n)) for each candidate, 
    // and in the worst case we may have to check up to n candidates
    // space complexity is O(1) since we are using a constant amount of space
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true

}

fn main() {
    assert_eq!(next_prime(12), 13);
    assert_eq!(next_prime(24), 29);
    assert_eq!(next_prime(11), 11);
}
