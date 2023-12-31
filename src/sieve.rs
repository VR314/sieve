pub fn largest_prime(limit: usize) -> usize {
    let list = list_of_primes(limit);
    list.last().copied().unwrap()
}

pub fn is_prime(n: usize) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    let limit: usize = f64::sqrt(n as f64) as usize + 1;
    let mut i = 2;
    while i <= limit {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

pub fn list_of_primes(limit: usize) -> Vec<usize> {
    let mut sieve = vec![true; limit + 1 as usize];
    let mut list: Vec<usize> = Vec::new();
    for i in 2..(limit + 1) {
        if sieve[i] {
            list.push(i);
            let mut j = i * i;
            while j <= limit {
                // println!("j: {j}");
                sieve[j] = false;
                j += i;
            }
        }
    }
    list
}

pub fn count_primes(limit: usize) -> usize {
    let mut sieve = vec![true; limit + 1 as usize];
    let mut count = 0;
    for i in 2..(limit + 1) {
        // printing the first 10 elements for debugging:
        // let first_10: Vec<_> = sieve.iter().take(10).collect();
        // println!("inner -- i: {i}, {:?}", &first_10);
        if sieve[i] {
            count += 1;
            let mut j = i * i;
            while j <= limit {
                // println!("j: {j}");
                sieve[j] = false;
                j += i;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        // use the primal library to test every number up to 1,000,000
        for i in 2..1_000_000 {
            // println!("Testing {}", i);
            assert_eq!(is_prime(i), primal::is_prime(i as u64));
        }
    }

    #[test]
    fn test_count_and_list_primes() {
        let limit = 1_000_000;
        let primal_sieve = primal::Sieve::new(limit);
        for i in (1..limit.ilog10()).map(|i| 10_usize.pow(i)) {
            let primal_count = primal_sieve.prime_pi(i);
            let my_count = count_primes(i);
            // println!("i: {i}, primal: {primal_count}, sieve: {my_count}");
            assert_eq!(primal_count, my_count);

            let list = list_of_primes(i);
            let list_len = list.len();
            // println!("i: {i}, sieve: {my_count}, list_len: {list_len}");
            assert_eq!(my_count, list_len);
        }
    }

    #[test]
    fn test_largest_prime() {
        let limit = 1_000_000;
        let primal_sieve = primal::Sieve::new(limit);
        for i in (1..limit.ilog10()).map(|i| 10_usize.pow(i)) {
            let primal_largest = primal_sieve
                .primes_from(0)
                .take_while(|&p| p < i)
                .last()
                .unwrap();
            let my_largest = largest_prime(i);
            println!("i: {i}, primal: {primal_largest}, sieve: {my_largest}");
            assert_eq!(primal_largest, my_largest);
        }
    }
}
