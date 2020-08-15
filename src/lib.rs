use rand::Rng;
use strength_reduce::StrengthReducedU128;
use strength_reduce::StrengthReducedU64;

#[allow(dead_code)]
#[inline(always)]
pub fn get_u64() -> String {
    let mut rng = rand::thread_rng();
    let num: u64 = rng.gen::<u64>();
    b10_to_b64_u64(num)
}

#[allow(dead_code)]
#[inline(always)]
pub fn get_u128() -> String {
    let mut rng = rand::thread_rng();
    let num: u128 = rng.gen::<u128>();
    b10_to_b64_u128(num)
}

const BASE64_CHARSET: &[u8] = b"0123456789\
                                ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                -_";

#[inline(always)]
fn b10_to_b64_u64(num: u64) -> String {
    let sr_262_144 = StrengthReducedU64::new(262_144);
    let sr_4_096 = StrengthReducedU64::new(4_096);
    let sr_64 = StrengthReducedU64::new(64);
    let mut _n = num;
    let mut vec: Vec<char> = Vec::with_capacity(11);
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

#[inline(always)]
fn b10_to_b64_u128(num: u128) -> String {
    let sr_262_144 = StrengthReducedU128::new(262_144);
    let sr_4_096 = StrengthReducedU128::new(4_096);
    let sr_64 = StrengthReducedU128::new(64);
    let mut _n = num;
    let mut vec: Vec<char> = Vec::with_capacity(22);
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

#[cfg(test)]
mod tests {
    use crate::{b10_to_b64_u128, b10_to_b64_u64};

    #[test]
    fn b10_to_b64_u64_test() {
        let tests = &[
            (u64::MIN, "0".to_string()),
            (1, "1".to_string()),
            (63, "_".to_string()),
            (64, "10".to_string()),
            (u64::MAX - 1, "F_________-".to_string()),
            (u64::MAX, "F__________".to_string()),
        ];
        for (input, want) in tests {
            let got = b10_to_b64_u64(*input);
            assert_eq!(got, *want);
        }
    }

    #[test]
    fn b10_to_b64_u128_test() {
        let tests = &[
            (u128::MIN, "0".to_string()),
            (1, "1".to_string()),
            (63, "_".to_string()),
            (64, "10".to_string()),
            (u128::MAX - 1, "3____________________-".to_string()),
            (u128::MAX, "3_____________________".to_string()),
        ];
        for (input, want) in tests {
            let got = b10_to_b64_u128(*input);
            assert_eq!(got, *want);
        }
    }
}
