pub fn create_string(n: i32, k: i32) -> String {
    if !is_possible(n, k) { return String::from(""); }
    let mut ab_string = vec![b'B'; n as usize];
    let mut res: i32 = k - n;
    let mut step: i32 = 0;
    while res >= -step {
        ab_string[step as usize] = b'A';
        step += 1;
        res = res - (n - (step * 2));
    }
    ab_string[-(res + 1) as usize] = b'A';
    let res = String::from_utf8(ab_string);
    return if res.is_ok() { res.unwrap() } else { "".to_string() };
}

pub fn is_possible(n: i32, k: i32) -> bool {
    let first_part: i32 = n / 2;
    let last_part: i32 = n - first_part;
    return (first_part * last_part) >= k;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn control_pairs(result_string: String, mut n: i32, k: i32) -> bool {
        if !is_possible(n, k) {
            n = 0;
            if result_string.is_empty() { return true; } //otherwise next if catch the incorrect result
        }
        if result_string.len() != n as usize {
            panic!("N {} doesn't match with result.length ({})", n, result_string.len());
        }
        let mut pair_count = 0;
        let result = result_string.as_bytes();
        for i in 0..n as usize {
            if result[i] == b'A' {
                for j in (i + 1)..n as usize {
                    if result[j] == b'B' {
                        pair_count += 1;
                    }
                }
            }
        }

        if pair_count == k {
            return true;
        } else {
            panic!("Result must contain K ({}) number of pair(s) but found pairCount ({}) isn't correct", k, pair_count);
        }
    }

    #[test]
    fn control_pairs_test() {
        assert!(control_pairs("ABAAB".to_string(), 5, 4));
        assert!(control_pairs("BA".to_string(), 2, 0));
        assert!(control_pairs("AB".to_string(), 2, 1));
        assert!(control_pairs("".to_string(), 5, 8));
        assert!(control_pairs("BAABBABAAB".to_string(), 10, 12));
    }

    #[test]
    fn n5k4() {
        assert!(control_pairs(create_string(5, 4), 5, 4));
    }

    #[test]
    fn n2k0() {
        assert!(control_pairs(create_string(2, 0), 2, 0));
    }

    #[test]
    fn n2k1() {
        assert!(control_pairs(create_string(2, 1), 2, 1));
    }

    #[test]
    fn n5k8() {
        assert!(control_pairs(create_string(5, 8), 5, 8));
    }

    #[test]
    fn n10k12() {
        assert!(control_pairs(create_string(10, 12), 10, 12));
    }

    #[test]
    fn ab_create_string_n9k0() {
        assert_eq!("BBBBBBBBA".to_string(), create_string(9, 0));
    }

    #[test]
    fn ab_create_string_n9k1() {
        assert_eq!("BBBBBBBAB".to_string(), create_string(9, 1));
    }

    #[test]
    fn ab_create_string_n9k2() {
        assert_eq!("BBBBBBABB".to_string(), create_string(9, 2));
    }

    #[test]
    fn ab_create_string_n9k3() {
        assert_eq!("BBBBBABBB".to_string(), create_string(9, 3));
    }

    #[test]
    fn ab_create_string_n9k4() {
        assert_eq!("BBBBABBBB".to_string(), create_string(9, 4));
    }

    #[test]
    fn ab_create_string_n9k5() {
        assert_eq!("BBBABBBBB".to_string(), create_string(9, 5));
    }

    #[test]
    fn ab_create_string_n9k6() {
        assert_eq!("BBABBBBBB".to_string(), create_string(9, 6));
    }

    #[test]
    fn ab_create_string_n9k7() {
        assert_eq!("BABBBBBBB".to_string(), create_string(9, 7));
    }

    #[test]
    fn ab_create_string_n9k8() {
        assert_eq!("ABBBBBBBB".to_string(), create_string(9, 8));
    }

    #[test]
    fn ab_create_string_n9k9() {
        assert_eq!("ABBBBBABB".to_string(), create_string(9, 9));
    }

    #[test]
    fn ab_create_string_n9k10() {
        assert_eq!("ABBBBABBB".to_string(), create_string(9, 10));
    }

    #[test]
    fn ab_create_string_n9k11() {
        assert_eq!("ABBBABBBB".to_string(), create_string(9, 11));
    }

    #[test]
    fn ab_create_string_n9k12() {
        assert_eq!("ABBABBBBB".to_string(), create_string(9, 12));
    }

    #[test]
    fn ab_create_string_n9k13() {
        assert_eq!("ABABBBBBB".to_string(), create_string(9, 13));
    }

    #[test]
    fn ab_create_string_n9k14() {
        assert_eq!("AABBBBBBB".to_string(), create_string(9, 14));
    }

    #[test]
    fn ab_create_string_n9k15() {
        assert_eq!("AABBBABBB".to_string(), create_string(9, 15));
    }

    #[test]
    fn ab_create_string_n9k16() {
        assert_eq!("AABBABBBB".to_string(), create_string(9, 16));
    }

    #[test]
    fn ab_create_string_n9k17() {
        assert_eq!("AABABBBBB".to_string(), create_string(9, 17));
    }

    #[test]
    fn ab_create_string_n9k18() {
        assert_eq!("AAABBBBBB".to_string(), create_string(9, 18));
    }

    #[test]
    fn ab_create_string_n9k19() {
        assert_eq!("AAABABBBB".to_string(), create_string(9, 19));
    }

    #[test]
    fn ab_create_string_n9k20() {
        assert_eq!("AAAABBBBB".to_string(), create_string(9, 20));
    }

    #[test]
    fn ab_control_string_n9k0() {
        assert!(control_pairs(create_string(9, 0), 9, 0));
    }

    #[test]
    fn ab_control_string_n9k1() {
        assert!(control_pairs(create_string(9, 1), 9, 1));
    }

    #[test]
    fn ab_control_string_n9k2() {
        assert!(control_pairs(create_string(9, 2), 9, 2));
    }

    #[test]
    fn ab_control_string_n9k3() {
        assert!(control_pairs(create_string(9, 3), 9, 3));
    }

    #[test]
    fn ab_control_string_n9k4() {
        assert!(control_pairs(create_string(9, 4), 9, 4));
    }

    #[test]
    fn ab_control_string_n9k5() {
        assert!(control_pairs(create_string(9, 5), 9, 5));
    }

    #[test]
    fn ab_control_string_n9k6() {
        assert!(control_pairs(create_string(9, 6), 9, 6));
    }

    #[test]
    fn ab_control_string_n9k7() {
        assert!(control_pairs(create_string(9, 7), 9, 7));
    }

    #[test]
    fn ab_control_string_n9k8() {
        assert!(control_pairs(create_string(9, 8), 9, 8));
    }

    #[test]
    fn ab_control_string_n9k9() {
        assert!(control_pairs(create_string(9, 9), 9, 9));
    }

    #[test]
    fn ab_control_string_n9k10() {
        assert!(control_pairs(create_string(9, 10), 9, 10));
    }

    #[test]
    fn ab_control_string_n9k11() {
        assert!(control_pairs(create_string(9, 11), 9, 11));
    }

    #[test]
    fn ab_control_string_n9k12() {
        assert!(control_pairs(create_string(9, 12), 9, 12));
    }

    #[test]
    fn ab_control_string_n9k13() {
        assert!(control_pairs(create_string(9, 13), 9, 13));
    }

    #[test]
    fn ab_control_string_n9k14() {
        assert!(control_pairs(create_string(9, 14), 9, 14));
    }

    #[test]
    fn ab_control_string_n9k15() {
        assert!(control_pairs(create_string(9, 15), 9, 15));
    }

    #[test]
    fn ab_control_string_n9k16() {
        assert!(control_pairs(create_string(9, 16), 9, 16));
    }

    #[test]
    fn ab_control_string_n9k17() {
        assert!(control_pairs(create_string(9, 17), 9, 17));
    }

    #[test]
    fn ab_control_string_n9k18() {
        assert!(control_pairs(create_string(9, 18), 9, 18));
    }

    #[test]
    fn ab_control_string_n9k19() {
        assert!(control_pairs(create_string(9, 19), 9, 19));
    }

    #[test]
    fn ab_control_string_n9k20() {
        assert!(control_pairs(create_string(9, 20), 9, 20));
    }
}
