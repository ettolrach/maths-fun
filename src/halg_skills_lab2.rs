// From uutils, copied here. Allowed because they licensed it using the MIT licence :)
// https://github.com/uutils/coreutils
fn gcd(mut n: usize, mut m: usize) -> usize {
    use std::cmp::min;
    use std::mem::swap;
    // Stein's binary GCD algorithm
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if n == 0 {
        return m;
    } else if m == 0 {
        return n;
    }

    // Extract common factor-2: gcd(2ⁱ n, 2ⁱ m) = 2ⁱ gcd(n, m)
    // and reducing until odd gcd(2ⁱ n, m) = gcd(n, m) if m is odd
    let k = {
        let k_n = n.trailing_zeros();
        let k_m = m.trailing_zeros();
        n >>= k_n;
        m >>= k_m;
        min(k_n, k_m)
    };

    loop {
        // Invariant: n odd
        debug_assert!(n % 2 == 1, "n = {n} is even");

        if n > m {
            swap(&mut n, &mut m);
        }
        m -= n;

        if m == 0 {
            return n << k;
        }

        m >>= m.trailing_zeros();
    }
}

/// Returns the prime numbers up to (but NOT including!) *n*.
///
/// # Examples
///
/// ```
/// use maths_fun::halg_skills_lab2::sieve_of_eratosthenes;
/// let primes_to_23 = sieve_of_eratosthenes(23);
/// assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19], primes_to_23);
/// ```
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut to_return: Vec<Option<usize>> = (2..n).map(Option::from).collect();
    let mut p = 2;
    while p <= n {
        let mut i = 2 * p;
        while i < n {
            to_return[i - 2] = None;
            i += p;
        }
        match to_return.iter()
            .flatten()
            .filter(|n| n > &&p)
            .min() {
            Some(num) => p = *num,
            None => break,
        }
    }
    to_return.iter()
        .flatten()
        .copied()
        .collect()
}

fn exp_with_mod(x: usize, n: usize, p: usize) -> usize {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return x;
    }
    if n % 2 == 0 {
        return exp_with_mod((x * x) % p, n / 2, p)
    }
    (x * exp_with_mod((x * x) % p, (n - 1) / 2, p)) % p

}

fn find_order_of(g: usize, p: usize) -> usize {
    assert!(g > 0, "Cannot find order of 0.");
    assert!(p >= 2, "p cannot be 1 or 0.");
    for i in 1..=p {
        if exp_with_mod(g, i, p) == 1 {
            return i;
        }
    }
    unreachable!()
}

fn euler_phi(m: usize) -> usize {
    (0..m).filter(|i| gcd(*i, m) == 1).count()
}

fn generators_of_int_mod(p: usize) -> Vec<usize> {
    (1..p).filter(|n| find_order_of(*n, p) == p - 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sieve() {
        assert_eq!(
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271],
            sieve_of_eratosthenes(272)
        );
    }

    #[test]
    fn exp_mod() {
        assert_eq!(1, exp_with_mod(3, 4, 5));
    }

    #[test]
    fn order_of_fields() {
        assert_eq!(1227, find_order_of(2, 120247));
        assert_eq!(34, find_order_of(3, 307));
        assert_eq!(210, find_order_of(7, 211));
    }

    #[test]
    fn euler_phi_test() {
        assert_eq!(1, euler_phi(1));
        assert_eq!(8, euler_phi(30));
        assert_eq!(8, euler_phi(20));
        assert_eq!(36, euler_phi(76));
        assert_eq!(40, euler_phi(100));
    }

    #[test]
    fn list_generator_claim() {
        let primes = sieve_of_eratosthenes(1000);
        assert!(std::iter::zip(primes.iter().map(|p| generators_of_int_mod(*p).len()), primes.iter().map(|p| euler_phi(*p - 1))).map(|(a, b)| a == b).all(|b| b))
    }
}