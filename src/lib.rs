pub fn fast_exp(base: i64, pow: i64) -> i64 {
    let mut multiplier: i64 = base;
    let mut tmp_pow: i64 = pow;
    let mut result = 1;

    while tmp_pow != 0 {
        if (tmp_pow & 1) == 1 {
            result *= multiplier;
        }
        multiplier *= multiplier;
        tmp_pow >>= 1;
    }
    result
}

// Perform fast exponentiation in a modulus.
pub fn fast_exp_mod(num: i64, pow: i64, modulus: i64) -> i64 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_exp_2_0() {
        let result = fast_exp(2, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn fast_exp_2_1() {
        let result = fast_exp(2, 1);
        assert_eq!(result, 2);
    }

    #[test]
    fn fast_exp_2_2() {
        let result = fast_exp(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn fast_exp_2_3() {
        let result = fast_exp(2, 3);
        assert_eq!(result, 8);
    }

    #[test]
    fn fast_exp_2_4() {
        let result = fast_exp(2, 4);
        assert_eq!(result, 16);
    }

    #[test]
    fn fast_exp_2_5() {
        let result = fast_exp(2, 5);
        assert_eq!(result, 32);
    }

    #[test]
    fn fast_exp_2_6() {
        let result = fast_exp(2, 6);
        assert_eq!(result, 64);
    }

    #[test]
    fn fast_exp_2_7() {
        let result = fast_exp(2, 7);
        assert_eq!(result, 128);
    }

    #[test]
    fn fast_exp_general2() {
        for i in 1..32 {
            let result = fast_exp(2, i);
            assert_eq!(result, 1i64 << i);
        }
    }

    #[test]
    fn fast_exp_general3() {
        for i in 1..20 {
            let result = fast_exp(3, i);
            assert_eq!(result, 3i64.pow(i as u32));
        }
    }
}
