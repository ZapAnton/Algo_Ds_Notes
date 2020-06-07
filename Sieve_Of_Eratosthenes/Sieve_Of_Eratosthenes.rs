fn sieve_of_eratosthenes(limit: usize) {
    let mut prime_factors = vec![true; limit + 1];
    let limit_sqrt = (limit as f64).sqrt().ceil() as usize;
    (2..limit_sqrt).for_each(|i| {
        if prime_factors[i] {
            for j in (i * i..=limit).step_by(i) {
                prime_factors[j] = false;
            }
        }
    });
    prime_factors
        .iter()
        .enumerate()
        .skip(2)
        .filter(|(_, &is_prime)| is_prime)
        .for_each(|(prime_index, _)| print!("{} ", prime_index));
}

fn main() {
    let limit = 30;
    println!("Prime numbers up to {}:", limit);
    sieve_of_eratosthenes(limit);
}

/* Output
 Prime numbers up to 30:
 2 3 5 7 11 13 17 19 23 29
*/
