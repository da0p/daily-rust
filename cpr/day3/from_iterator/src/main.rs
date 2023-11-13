fn main() {
    let primes = vec![2, 3, 5, 7];
    // With FromIterator trait can help to build a collection from
    // an iterator
    let prime_squares = primes
        .into_iter()
        .map(|prime| prime * prime)
        .collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");
}
