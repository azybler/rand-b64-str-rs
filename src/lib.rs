use rand::Rng;
use strength_reduce::StrengthReducedU128;
use strength_reduce::StrengthReducedU64;

const BASE64_CHARSET: &[u8] = b"0123456789\
                                ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                -_";

macro_rules! b10_to_b64 {
    ($input: expr, $b: expr, $sr: path) => {
        {
            let sr_262_144 = $sr(262_144);
            let sr_4_096 = $sr(4_096);
            let sr_64 = $sr(64);

            let mut _n = $input;
            let mut vec: Vec<char> = Vec::with_capacity($b);
            while _n >= 262_144 {
                vec.push(BASE64_CHARSET[(_n % sr_64) as usize] as char);
                vec.push(BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char);
                vec.push(BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char);
                _n = _n / sr_262_144;
            }
            while _n >= 4_096 {
                vec.push(BASE64_CHARSET[(_n % sr_64) as usize] as char);
                vec.push(BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char);
                _n = _n / sr_4_096;
            }
            while _n >= 64 {
                vec.push(BASE64_CHARSET[(_n % sr_64) as usize] as char);
                _n = _n / sr_64;
            }
            vec.push(BASE64_CHARSET[_n as usize] as char);
            vec.iter().rev().collect::<String>()
        }
    };
}

macro_rules! get {
    ($type: ty, $b: expr, $sr: path) => {
        {
            let mut rng = rand::thread_rng();
            let num: $type = rng.gen::<$type>();
            b10_to_b64!(num, $b, $sr)
        }
    };
}

#[allow(dead_code)]
#[inline(always)]
pub fn get_u64() -> String {
    get!(u64, 11, StrengthReducedU64::new)
}

#[allow(dead_code)]
#[inline(always)]
pub fn get_u128() -> String {
    get!(u128, 22, StrengthReducedU128::new)
}

#[cfg(test)]
mod tests {
    const BASE64_CHARSET: &[u8] = b"0123456789\
                                ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                -_";

    #[test]
    fn b10_to_b64_u64_test() {
        use strength_reduce::StrengthReducedU64;

        let tests = &[
            (u64::MIN, "0".to_string()),
            (1, "1".to_string()),
            (63, "_".to_string()),
            (64, "10".to_string()),
            (u64::MAX - 1, "F_________-".to_string()),
            (u64::MAX, "F__________".to_string()),
        ];
        for (input, want) in tests {
            let got = b10_to_b64!(*input, 11, StrengthReducedU64::new);
            assert_eq!(got, *want);
        }
    }

    #[test]
    fn b10_to_b64_u128_test() {
        use strength_reduce::StrengthReducedU128;

        let tests = &[
            (u128::MIN, "0".to_string()),
            (1, "1".to_string()),
            (63, "_".to_string()),
            (64, "10".to_string()),
            (u128::MAX - 1, "3____________________-".to_string()),
            (u128::MAX, "3_____________________".to_string()),
        ];
        for (input, want) in tests {
            let got = b10_to_b64!(*input, 22, StrengthReducedU128::new);
            assert_eq!(got, *want);
        }
    }
}
