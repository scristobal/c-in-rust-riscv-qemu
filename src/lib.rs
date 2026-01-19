mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn gcd(n: i32, m: i32) -> i32 {
    let pair = &mut bindings::Pair { n, m };
    unsafe { bindings::gcd(pair) }
}

/// Continued fraction representation of a rational number.
/// A rational p/q is represented as [a0; a1, a2, ...] where:
/// p/q = a0 + 1/(a1 + 1/(a2 + 1/(...)))
pub struct ContinuedFraction {
    coefficients: Vec<i32>,
}

impl ContinuedFraction {
    /// Convert a rational number p/q to its continued fraction representation.
    pub fn from_rational(mut p: i32, mut q: i32) -> Self {
        let d = gcd(p.abs(), q.abs());
        p /= d;
        q /= d;

        if q < 0 {
            p = -p;
            q = -q;
        }

        let mut coefficients = Vec::new();

        while q != 0 {
            coefficients.push(p / q);
            (p, q) = (q, p % q);
        }

        Self { coefficients }
    }

    /// Convert the continued fraction back to a rational number (p, q).
    pub fn to_rational(&self) -> (i32, i32) {
        if self.coefficients.is_empty() {
            return (0, 1);
        }

        let mut num = *self.coefficients.last().unwrap();
        let mut den = 1;

        for &coef in self.coefficients.iter().rev().skip(1) {
            (num, den) = (coef * num + den, num);
        }

        (num, den)
    }

    /// Get the continued fraction coefficients [a0; a1, a2, ...]
    pub fn coefficients(&self) -> &[i32] {
        &self.coefficients
    }

    /// Compute convergents pairs (h_n, k_n) where h_n/k_n approaches the original value.
    pub fn convergents(&self) -> Vec<(i32, i32)> {
        let mut result = Vec::new();

        // h_{-2} = 0, h_{-1} = 1
        // k_{-2} = 1, k_{-1} = 0
        // h_n = a_n * h_{n-1} + h_{n-2}
        // k_n = a_n * k_{n-1} + k_{n-2}

        let (mut h_2, mut k_2) = (0, 1); // h_{n-2}, k_{n-2}
        let (mut h_1, mut k_1) = (1, 0); // h_{n-1}, k_{n-1}

        for &a in &self.coefficients {
            let h = a * h_1 + h_2;
            let k = a * k_1 + k_2;

            result.push((h, k));

            (h_2, k_2) = (h_1, k_1);
            (h_1, k_1) = (h, k);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(9, 3), 3);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(7, 7), 7);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(17, 13), 1);
    }

    #[test]
    fn test_continued_fraction() {
        let cf = ContinuedFraction::from_rational(3, 1);
        assert_eq!(cf.coefficients(), &[3]);

        let cf = ContinuedFraction::from_rational(1, 2);
        assert_eq!(cf.coefficients(), &[0, 2]);

        let cf = ContinuedFraction::from_rational(89, 55);
        assert_eq!(cf.coefficients(), &[1, 1, 1, 1, 1, 1, 1, 1, 2]);

        let test_cases = [(3, 7), (22, 7), (1, 3), (5, 1), (89, 55)];

        for (p, q) in test_cases {
            let cf = ContinuedFraction::from_rational(p, q);
            let (p2, q2) = cf.to_rational();

            assert_eq!(p * q2, p2 * q, "Roundtrip failed for {}/{}", p, q);
        }

        let cf = ContinuedFraction::from_rational(22, 7);
        let conv = cf.convergents();
        assert_eq!(conv, vec![(3, 1), (22, 7)]);
    }
}
