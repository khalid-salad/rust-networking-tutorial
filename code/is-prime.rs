fn is_prime(n: u32) -> bool {
    for i in (2..n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
