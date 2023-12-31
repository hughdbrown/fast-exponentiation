pub fn fast_exp(base: u64, pow: u64) -> u64 {
    let mut multiplier: u64 = base;
    let mut tmp_pow: u64 = pow;
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
pub fn fast_exp_mod(base: u64, pow: u64, modulus: u64) -> u64 {
    let mut multiplier: u64 = base;
    let mut tmp_pow: u64 = pow;
    let mut result = 1;

    while tmp_pow != 0 {
        if (tmp_pow & 1) == 1 {
            result *= multiplier;
            result %= modulus;
        }
        multiplier *= multiplier;
        multiplier %= modulus;
        tmp_pow >>= 1;
    }
    result
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
            assert_eq!(result, 1u64 << i);
        }
    }

    #[test]
    fn fast_exp_general3() {
        for i in 1..20 {
            let result = fast_exp(3, i);
            assert_eq!(result, 3u64.pow(i as u32));
        }
    }


    #[test]
    fn fast_exp_mod_8_1_10() {
        let result = fast_exp_mod(8, 1, 10);
        assert_eq!(result, 8);
    }

    #[test]
    fn fast_exp_mod_8_2_10() {
        let result = fast_exp_mod(8, 2, 10);
        assert_eq!(result, 4);
    }

    #[test]
    fn fast_exp_mod_8_3_10() {
        let result = fast_exp_mod(8, 3, 10);
        assert_eq!(result, 2);
    }

    #[test]
    fn fast_exp_mod_8_4_10() {
        let result = fast_exp_mod(8, 4, 10);
        assert_eq!(result, 6);
    }

    #[test]
    fn fast_exp_mod_8_5_10() {
        let result = fast_exp_mod(8, 5, 10);
        assert_eq!(result, 8);
    }

    #[test]
    fn fast_exp_mod_8_6_10() {
        let result = fast_exp_mod(8, 6, 10);
        assert_eq!(result, 4);
    }

    #[test]
    fn fast_exp_mod_1234567_1652899_17276041() {
        let result = fast_exp_mod(1234567, 1652899, 17276041);
        assert_eq!(result, 9062798);
    }

}
