// use std::collections::HashSet;

use crate::{compute_hand, Decision, GameType, Rules};

pub fn soft_totals_chart(
    cards_in_hand: &Vec<u8>,
    dealer_up_card: u8,
    running_count: isize,
    true_count: isize,
    rules: &Rules,
) -> Option<Decision> {
    let mut tmp = cards_in_hand.clone();

    let (total, res) = compute_hand(&mut tmp);

    if !res || (!tmp.contains(&11) && total > 7) {
        return None;
    }

    if total < 8 && !tmp.contains(&11) {
        return Some(Decision::Hit);
    }

    // A9
    if total == 20 {
        return Some(Decision::Stand);
    }

    // A8
    if total == 19 {
        if cards_in_hand.len() == 2 {
            match rules.game_type {
                GameType::Hit17 if running_count < 0 && dealer_up_card == 6 => {
                    return Some(Decision::Stand)
                }
                GameType::Stand17 => {
                    if dealer_up_card == 6 && rules.is_double_allowed.any() {
                        if true_count >= 1 {
                            return Some(Decision::Double);
                        }

                        return Some(Decision::Stand);
                    }
                }

                _ => (),
            };

            if (dealer_up_card == 6
                || (true_count >= 3 && dealer_up_card == 4)
                || (true_count >= 1 && dealer_up_card == 5))
                && rules.is_double_allowed.any()
            {
                return Some(Decision::Double);
            }
        }

        return Some(Decision::Stand);
    }

    // A7
    if total == 18 {
        if dealer_up_card >= 2
            && dealer_up_card <= 6
            && rules.is_double_allowed.any()
            && cards_in_hand.len() == 2
        {
            return Some(Decision::Double);
        }

        if dealer_up_card >= 9 && dealer_up_card <= 11 {
            return Some(Decision::Hit);
        }
    }

    // A6
    if total == 17 {
        if rules.is_double_allowed.any()
            && ((dealer_up_card >= 3 && dealer_up_card <= 6)
                || (dealer_up_card == 2 && true_count >= 1))
            && cards_in_hand.len() == 2
        {
            return Some(Decision::Double);
        }

        return Some(Decision::Hit);
    }

    // A5 and A4
    if total == 16 || total == 15 {
        if rules.is_double_allowed.any()
            && dealer_up_card >= 4
            && dealer_up_card <= 6
            && cards_in_hand.len() == 2
        {
            return Some(Decision::Double);
        }

        return Some(Decision::Hit);
    }

    // A3 and A2
    if total == 14 || total == 13 {
        if rules.is_double_allowed.any()
            && dealer_up_card >= 5
            && dealer_up_card <= 6
            && cards_in_hand.len() == 2
        {
            return Some(Decision::Double);
        }

        return Some(Decision::Hit);
    }

    Some(Decision::Stand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soft_totals_3_card_magic() {
        let rules = Rules::default();

        assert_eq!(soft_totals_chart(&vec![11, 4, 9], 3, 0, 0, &rules), None);
    }

    #[test]
    fn soft_totals_invalid() {
        let rules = Rules::default();

        for duc in 2..=11 {
            assert_eq!(soft_totals_chart(&vec![10, 9], duc, 0, 0, &rules), None);
        }
    }

    #[test]
    fn soft_totals_crazy_example() {
        let rules = Rules::default();

        // evaluates to a8
        let example_inp = vec![11, 11, 11, 11, 11, 4];

        assert_eq!(
            soft_totals_chart(&example_inp, 6, 0, 0, &rules),
            Some(Decision::Stand)
        );

        assert_eq!(
            soft_totals_chart(&example_inp, 5, 0, 0, &rules),
            Some(Decision::Stand)
        );
    }

    #[test]
    fn yet_another_example() {
        let rules = Rules::default();

        // evaluates to a3
        let example_inp = vec![11, 11, 2];

        assert_eq!(
            soft_totals_chart(&example_inp, 6, 0, 0, &rules),
            Some(Decision::Hit)
        );

        // evaluates to a3
        let example_inp = vec![11, 3];

        assert_eq!(
            soft_totals_chart(&example_inp, 6, 0, 0, &rules),
            Some(Decision::Double)
        );
    }

    #[test]
    fn soft_totals_a9_valid() {
        let rules = Rules::default();

        for duc in 2..=11 {
            assert_eq!(
                soft_totals_chart(&vec![11, 9], duc, 0, 0, &rules),
                Some(Decision::Stand)
            );
        }
    }

    #[test]
    fn soft_totals_a8_valid() {
        let rules = Rules::default();

        for duc in 2..=11 {
            if duc == 6 {
                assert_eq!(
                    soft_totals_chart(&vec![11, 8], duc, 0, 0, &rules),
                    Some(Decision::Double)
                );

                continue;
            }

            assert_eq!(
                soft_totals_chart(&vec![11, 8], duc, 0, 0, &rules),
                Some(Decision::Stand)
            );
        }
    }

    #[test]
    fn soft_totals_a7_valid() {
        let rules = Rules::default();

        for duc in 2..=11 {
            if duc >= 2 && duc <= 6 {
                assert_eq!(
                    soft_totals_chart(&vec![11, 7], duc, 0, 0, &rules),
                    Some(Decision::Double)
                );

                continue;
            }

            if duc == 7 || duc == 8 {
                assert_eq!(
                    soft_totals_chart(&vec![11, 7], duc, 0, 0, &rules),
                    Some(Decision::Stand)
                );

                continue;
            }

            assert_eq!(
                soft_totals_chart(&vec![11, 7], duc, 0, 0, &rules),
                Some(Decision::Hit)
            );
        }
    }

    #[test]
    fn soft_totals_a6_valid() {
        let rules = Rules::default();

        for duc in 2..=11 {
            if duc >= 3 && duc <= 6 {
                assert_eq!(
                    soft_totals_chart(&vec![11, 6], duc, 0, 0, &rules),
                    Some(Decision::Double)
                );

                continue;
            }

            assert_eq!(
                soft_totals_chart(&vec![11, 6], duc, 0, 0, &rules),
                Some(Decision::Hit)
            );
        }
    }

    #[test]
    fn soft_totals_a5_a4_valid() {
        let rules = Rules::default();

        for duc in 2..=11 {
            if duc >= 4 && duc <= 6 {
                assert_eq!(
                    soft_totals_chart(&vec![11, 5], duc, 0, 0, &rules),
                    Some(Decision::Double)
                );

                assert_eq!(
                    soft_totals_chart(&vec![11, 4], duc, 0, 0, &rules),
                    Some(Decision::Double)
                );

                continue;
            }

            assert_eq!(
                soft_totals_chart(&vec![11, 5], duc, 0, 0, &rules),
                Some(Decision::Hit)
            );

            assert_eq!(
                soft_totals_chart(&vec![11, 4], duc, 0, 0, &rules),
                Some(Decision::Hit)
            );
        }
    }

    #[test]
    fn soft_totals_a3_a2_valid() {
        let rules = Rules::default();

        for duc in 2..=11 {
            if duc >= 5 && duc <= 6 {
                assert_eq!(
                    soft_totals_chart(&vec![11, 3], duc, 0, 0, &rules),
                    Some(Decision::Double)
                );

                assert_eq!(
                    soft_totals_chart(&vec![11, 2], duc, 0, 0, &rules),
                    Some(Decision::Double)
                );

                continue;
            }

            assert_eq!(
                soft_totals_chart(&vec![11, 3], duc, 0, 0, &rules),
                Some(Decision::Hit)
            );

            assert_eq!(
                soft_totals_chart(&vec![11, 2], duc, 0, 0, &rules),
                Some(Decision::Hit)
            );
        }
    }

    #[test]
    fn soft_totals_a6_deviation() {
        let rules = Rules::default();

        // without deviation
        assert_eq!(
            soft_totals_chart(&vec![11, 6], 2, 10, 0, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            soft_totals_chart(&vec![11, 6], 2, 10, 2, &rules),
            Some(Decision::Double)
        );
    }

    #[test]
    fn soft_totals_a8_v_6_deviation() {
        let mut rules = Rules::default();

        // without deviation
        assert_eq!(
            soft_totals_chart(&vec![11, 8], 6, 10, 0, &rules),
            Some(Decision::Double)
        );

        assert_eq!(
            soft_totals_chart(&vec![11, 8], 6, -1, 0, &rules),
            Some(Decision::Stand)
        );

        rules.game_type = GameType::Stand17;

        assert_eq!(
            soft_totals_chart(&vec![11, 8], 6, 10, 0, &rules),
            Some(Decision::Stand)
        );

        assert_eq!(
            soft_totals_chart(&vec![11, 8], 6, 10, 1, &rules),
            Some(Decision::Double)
        );
    }

    #[test]
    fn soft_totals_a8_v_5_deviation() {
        let rules = Rules::default();

        // before
        assert_eq!(
            soft_totals_chart(&vec![11, 8], 5, 10, 0, &rules),
            Some(Decision::Stand)
        );

        assert_eq!(
            soft_totals_chart(&vec![11, 8], 5, 10, 1, &rules),
            Some(Decision::Double)
        );
    }

    #[test]
    fn soft_totals_a8_v_4_deviation() {
        let rules = Rules::default();

        // before
        assert_eq!(
            soft_totals_chart(&vec![11, 8], 4, 10, 0, &rules),
            Some(Decision::Stand)
        );

        assert_eq!(
            soft_totals_chart(&vec![11, 8], 4, 10, 3, &rules),
            Some(Decision::Double)
        );
    }

    #[test]
    fn soft_totals_hard_total() {
        let rules = Rules::default();

        assert_eq!(
            soft_totals_chart(&vec![2, 4], 10, -9, -2, &rules),
            Some(Decision::Hit)
        );
    }

    #[test]
    fn soft_totals_hard_total_2() {
        let rules = Rules::default();

        assert_eq!(
            soft_totals_chart(&vec![2, 4, 11], 10, -9, -2, &rules),
            Some(Decision::Hit)
        );
    }

    #[test]
    fn soft_totals_hard_total_3() {
        let rules = Rules::default();

        assert_eq!(
            soft_totals_chart(&vec![3, 4, 11], 10, -9, -2, &rules),
            Some(Decision::Hit)
        );
    }

    #[test]
    fn soft_totals_hard_total_4() {
        let rules = Rules::default();

        assert_eq!(
            soft_totals_chart(&vec![3, 4, 1, 11], 10, -9, -2, &rules),
            Some(Decision::Stand)
        );
    }

    #[test]
    fn soft_totals_hard_total_5() {
        let rules = Rules::default();

        assert_eq!(
            soft_totals_chart(&vec![3, 3], 4, -9, -2, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(soft_totals_chart(&vec![5, 5], 4, -9, -2, &rules), None);
    }
}
