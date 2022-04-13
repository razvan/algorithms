/// Compute all primes less then `n` using the classic Eratostenes sieve.
pub fn primes_lt(n: usize) -> Vec<usize> {
    if n < 2 {
        vec![]
    } else {
        let mut primes: Vec<bool> = (0..n).map(|_| true).collect();
        primes[0] = false;
        primes[1] = false;

        for i in 2..(1 + (n as f64).sqrt() as usize) {
            if primes[i] {
                for j in (i * i..n).step_by(i) {
                    primes[j] = false;
                }
            }
        }
        primes
            .iter()
            .enumerate()
            .flat_map(|(index, prime)| match prime {
                false => None,
                true => Some(index),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::primes_lt;

    #[test]
    fn test_primes_lt() {
        let result = primes_lt(20);
        assert_eq!(&result, &[2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
