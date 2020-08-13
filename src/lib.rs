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
    let sr_281_474_976_710_656: StrengthReducedU64 = StrengthReducedU64::new(281_474_976_710_656);
    let sr_4_398_046_511_104: StrengthReducedU64 = StrengthReducedU64::new(4_398_046_511_104);
    let sr_68_719_476_736: StrengthReducedU64 = StrengthReducedU64::new(68_719_476_736);
    let sr_1_073_741_824: StrengthReducedU64 = StrengthReducedU64::new(1_073_741_824);
    let sr_16_777_216: StrengthReducedU64 = StrengthReducedU64::new(16_777_216);
    let sr_262_144: StrengthReducedU64 = StrengthReducedU64::new(262_144);
    let sr_4_096: StrengthReducedU64 = StrengthReducedU64::new(4_096);
    let sr_64: StrengthReducedU64 = StrengthReducedU64::new(64);
    let mut _n = num;
    let mut res: String = "".to_string();
    while _n >= 281_474_976_710_656 {
        res = format!(
            "{}{}{}{}{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_4_398_046_511_104 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_68_719_476_736 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_1_073_741_824 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_16_777_216 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_281_474_976_710_656;
    }
    while _n >= 4_398_046_511_104 {
        res = format!(
            "{}{}{}{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_68_719_476_736 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_1_073_741_824 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_16_777_216 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_4_398_046_511_104;
    }
    while _n >= 68_719_476_736 {
        res = format!(
            "{}{}{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_1_073_741_824 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_16_777_216 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_68_719_476_736;
    }
    while _n >= 1_073_741_824 {
        res = format!(
            "{}{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_16_777_216 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_1_073_741_824;
    }
    while _n >= 16_777_216 {
        res = format!(
            "{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_16_777_216;
    }
    while _n >= 262_144 {
        res = format!(
            "{}{}{}{}",
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_262_144;
    }
    while _n >= 4096 {
        res = format!(
            "{}{}{}",
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_4_096;
    }
    while _n >= 64 {
        res = format!("{}{}", BASE64_CHARSET[(_n % sr_64) as usize] as char, res);
        _n = _n / sr_64;
    }
    res = format!("{}{}", BASE64_CHARSET[_n as usize] as char, res);
    return res;
}

#[inline(always)]
fn b10_to_b64_u128(num: u128) -> String {
    let sr_281_474_976_710_656: StrengthReducedU128 = StrengthReducedU128::new(281_474_976_710_656);
    let sr_4_398_046_511_104: StrengthReducedU128 = StrengthReducedU128::new(4_398_046_511_104);
    let sr_68_719_476_736: StrengthReducedU128 = StrengthReducedU128::new(68_719_476_736);
    let sr_1_073_741_824: StrengthReducedU128 = StrengthReducedU128::new(1_073_741_824);
    let sr_16_777_216: StrengthReducedU128 = StrengthReducedU128::new(16_777_216);
    let sr_262_144: StrengthReducedU128 = StrengthReducedU128::new(262_144);
    let sr_4_096: StrengthReducedU128 = StrengthReducedU128::new(4_096);
    let sr_64: StrengthReducedU128 = StrengthReducedU128::new(64);
    let mut _n = num;
    let mut res: String = "".to_string();
    while _n >= 281_474_976_710_656 {
        res = format!(
            "{}{}{}{}{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_4_398_046_511_104 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_68_719_476_736 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_1_073_741_824 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_16_777_216 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_281_474_976_710_656;
    }
    while _n >= 4_398_046_511_104 {
        res = format!(
            "{}{}{}{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_68_719_476_736 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_1_073_741_824 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_16_777_216 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_4_398_046_511_104;
    }
    while _n >= 68_719_476_736 {
        res = format!(
            "{}{}{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_1_073_741_824 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_16_777_216 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_68_719_476_736;
    }
    while _n >= 1_073_741_824 {
        res = format!(
            "{}{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_16_777_216 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_1_073_741_824;
    }
    while _n >= 16_777_216 {
        res = format!(
            "{}{}{}{}{}",
            BASE64_CHARSET[(_n / sr_262_144 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_16_777_216;
    }
    while _n >= 262_144 {
        res = format!(
            "{}{}{}{}",
            BASE64_CHARSET[(_n / sr_4_096 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_262_144;
    }
    while _n >= 4096 {
        res = format!(
            "{}{}{}",
            BASE64_CHARSET[(_n / sr_64 % sr_64) as usize] as char,
            BASE64_CHARSET[(_n % sr_64) as usize] as char,
            res,
        );
        _n = _n / sr_4_096;
    }
    while _n >= 64 {
        res = format!("{}{}", BASE64_CHARSET[(_n % sr_64) as usize] as char, res);
        _n = _n / sr_64;
    }
    res = format!("{}{}", BASE64_CHARSET[_n as usize] as char, res);
    return res;
}

#[cfg(test)]
mod tests {
    use crate::b10_to_b64_u128;
    use crate::b10_to_b64_u64;

    #[test]
    fn b10_to_b64_u64_test() {
        assert_eq!(b10_to_b64_u64(u64::MIN), "0".to_string());
        assert_eq!(b10_to_b64_u64(1), "1".to_string());
        assert_eq!(b10_to_b64_u64(63), "_".to_string());
        assert_eq!(b10_to_b64_u64(64), "10".to_string());
        assert_eq!(b10_to_b64_u64(u64::MAX), "F__________".to_string());
    }

    #[test]
    fn b10_to_b64_u128_test() {
        assert_eq!(b10_to_b64_u128(u128::MIN), "0".to_string());
        assert_eq!(b10_to_b64_u128(1), "1".to_string());
        assert_eq!(b10_to_b64_u128(63), "_".to_string());
        assert_eq!(b10_to_b64_u128(64), "10".to_string());
        assert_eq!(
            b10_to_b64_u128(u128::MAX),
            "3_____________________".to_string()
        );
    }
}
