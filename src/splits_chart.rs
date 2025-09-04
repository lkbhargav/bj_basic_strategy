use crate::types::Rules;

pub fn should_i_split(
    cards_in_hand: &Vec<u8>,
    dealer_up_card: u8,
    true_count: isize,
    rules: &Rules,
) -> bool {
    if cards_in_hand.len() < 2 || cards_in_hand.len() > 2 || cards_in_hand[0] != cards_in_hand[1] {
        return false;
    }

    let pc = cards_in_hand[0];

    // 11 & 8
    if pc == 11 || pc == 8 {
        return true;
    }

    // 10
    if pc == 10 && rules.enable_deviations() {
        if (true_count == 4 && dealer_up_card == 6)
            || (true_count == 5 && (dealer_up_card >= 5 && dealer_up_card <= 6))
            || (true_count >= 6 && (dealer_up_card >= 4 && dealer_up_card <= 6))
        {
            return true;
        }
    }

    // 9
    if pc == 9
        && ((dealer_up_card >= 2 && dealer_up_card <= 6)
            || (dealer_up_card >= 8 && dealer_up_card <= 9))
    {
        return true;
    }

    // 7, 2 and 3
    if (pc == 7 || pc == 2 || pc == 3) && dealer_up_card >= 2 && dealer_up_card <= 7 {
        if !rules.double_after_split()
            && (pc == 2 || pc == 3)
            && (dealer_up_card == 2 || dealer_up_card == 3)
        {
            return false;
        }

        return true;
    }

    // 6
    if pc == 6 && dealer_up_card >= 2 && dealer_up_card <= 6 {
        if !rules.double_after_split() && dealer_up_card == 2 {
            return false;
        }

        return true;
    }

    // 4
    if pc == 4 && dealer_up_card >= 5 && dealer_up_card <= 6 {
        if !rules.double_after_split() {
            return false;
        }

        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_i_split_default_valid() {
        let rules = Rules::default();

        for duc in 2..=11 {
            assert!(should_i_split(&vec![11, 11], duc, 0, &rules));
            assert!(!should_i_split(&vec![10, 10], duc, 0, &rules));

            if duc == 7 || duc == 10 || duc == 11 {
                assert!(!should_i_split(&vec![9, 9], duc, 0, &rules));
            } else {
                assert!(should_i_split(&vec![9, 9], duc, 0, &rules));
            }

            assert!(should_i_split(&vec![8, 8], duc, 0, &rules));

            if duc > 7 {
                assert!(!should_i_split(&vec![7, 7], duc, 0, &rules));
                assert!(!should_i_split(&vec![2, 2], duc, 0, &rules));
                assert!(!should_i_split(&vec![3, 3], duc, 0, &rules));
            } else {
                assert!(should_i_split(&vec![7, 7], duc, 0, &rules));
                assert!(should_i_split(&vec![2, 2], duc, 0, &rules));
                assert!(should_i_split(&vec![3, 3], duc, 0, &rules));
            }

            if duc > 6 {
                assert!(!should_i_split(&vec![6, 6], duc, 0, &rules));
            } else {
                assert!(should_i_split(&vec![6, 6], duc, 0, &rules));
            }

            assert!(!should_i_split(&vec![5, 5], duc, 0, &rules));

            if duc < 5 || duc > 6 {
                assert!(!should_i_split(&vec![4, 4], duc, 0, &rules));
            } else {
                assert!(should_i_split(&vec![4, 4], duc, 0, &rules));
            }
        }
    }

    #[test]
    fn should_i_split_10_true_count() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(true);

        let pc = &vec![10, 10];

        let tc = 3;

        for duc in 2..=11 {
            assert!(!should_i_split(pc, duc, tc, &rules));
        }

        let tc = 4;

        for duc in 2..=11 {
            if duc == 6 {
                assert!(should_i_split(pc, duc, tc, &rules));
                continue;
            }

            assert!(!should_i_split(pc, duc, tc, &rules));
        }

        let tc = 5;

        for duc in 2..=11 {
            if duc == 6 || duc == 5 {
                assert!(should_i_split(pc, duc, tc, &rules));
                continue;
            }
        }

        let tc = 6;

        for duc in 2..=11 {
            if duc == 6 || duc == 5 || duc == 4 {
                assert!(should_i_split(pc, duc, tc, &rules));
                continue;
            }

            assert!(!should_i_split(pc, duc, tc, &rules));
        }
    }
}
