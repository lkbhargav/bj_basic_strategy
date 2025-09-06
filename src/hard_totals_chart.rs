use crate::{
    compute_hand,
    types::{Decision, GameType, Rules},
};

pub fn hard_totals_chart(
    cards_in_hand: &Vec<u8>,
    dealer_up_card: u8,
    running_count: isize,
    true_count: isize,
    rules: &Rules,
) -> Option<Decision> {
    if cards_in_hand.len() == 0 && cards_in_hand[0] == cards_in_hand[1] && cards_in_hand[0] != 5 {
        return None;
    }

    let mut tmp = cards_in_hand.clone();
    let (total, res) = compute_hand(&mut tmp);

    if !res {
        return None;
    }

    if total >= 17
        || (total == 16 && dealer_up_card >= 2 && dealer_up_card <= 6)
        || (total == 15 && dealer_up_card >= 2 && dealer_up_card <= 6)
        || (total == 14 && dealer_up_card >= 2 && dealer_up_card <= 6)
    {
        return Some(Decision::Stand);
    }

    if total == 16 && rules.enable_deviations() > 0 {
        if (dealer_up_card == 9 && true_count >= 4)
            || (dealer_up_card == 10 && running_count > 0)
            || (dealer_up_card == 11 && true_count >= 3 && rules.game_type() == &GameType::Hit17)
        {
            return Some(Decision::Stand);
        }
    }

    if total == 15 && rules.enable_deviations() > 0 {
        if (dealer_up_card == 10 && true_count >= 4)
            || (dealer_up_card == 11 && true_count >= 5 && rules.game_type() == &GameType::Hit17)
        {
            return Some(Decision::Stand);
        }
    }

    if total == 13 {
        if dealer_up_card == 2 && true_count <= -1 && rules.enable_deviations() > 0 {
            return Some(Decision::Hit);
        }

        if dealer_up_card >= 2 && dealer_up_card <= 6 {
            return Some(Decision::Stand);
        }
    }

    if total == 12 {
        if dealer_up_card == 4 && running_count < 0 && rules.enable_deviations() > 0 {
            return Some(Decision::Hit);
        }

        if (dealer_up_card == 2 && true_count >= 3)
            || (dealer_up_card == 3 && true_count >= 2) && rules.enable_deviations() > 0
        {
            return Some(Decision::Stand);
        }

        if dealer_up_card >= 4 && dealer_up_card <= 6 {
            return Some(Decision::Stand);
        }
    }

    if total == 11 {
        if rules.enable_deviations() > 0 {
            match rules.game_type() {
                GameType::Stand17 if true_count < 1 && dealer_up_card == 11 => {
                    return Some(Decision::Hit);
                }
                _ => (),
            };
        }

        if cards_in_hand.len() == 2 {
            return Some(Decision::Double);
        }
    }

    if total == 10 {
        if ((dealer_up_card >= 2 && dealer_up_card <= 9)
            || ((dealer_up_card == 10 && true_count >= 4)
                || (dealer_up_card == 11
                    && true_count >= 3
                    && rules.game_type() == &GameType::Hit17)
                || (dealer_up_card == 11
                    && true_count >= 4
                    && rules.game_type() == &GameType::Stand17))
                && rules.enable_deviations() > 0)
            && cards_in_hand.len() == 2
        {
            return Some(Decision::Double);
        }
    }

    if total == 9 {
        if ((dealer_up_card >= 3 && dealer_up_card <= 6)
            || ((dealer_up_card == 2 && true_count >= 1)
                || (dealer_up_card == 7 && true_count >= 3))
                && rules.enable_deviations() > 0)
            && cards_in_hand.len() == 2
        {
            return Some(Decision::Double);
        }
    }

    if total == 8 && rules.enable_deviations() > 0 && cards_in_hand.len() == 2 {
        if dealer_up_card == 6 && true_count >= 2 {
            return Some(Decision::Double);
        }

        if dealer_up_card == 5 && true_count >= 4 && rules.enable_deviations() > 1 {
            return Some(Decision::Double);
        }
    }

    Some(Decision::Hit)
}

#[cfg(test)]
mod tests {
    use crate::types::Deviations;

    use super::*;

    #[test]
    fn test_hard_totals_chart_invalid() {
        let rules = Rules::default();

        assert_eq!(hard_totals_chart(&vec![10, 5, 9], 8, 0, 0, &rules), None);
    }

    #[test]
    fn test_hard_17() {
        let rules = Rules::default();

        for duc in 2..=11 {
            assert_eq!(
                hard_totals_chart(&vec![10, 7], duc, 0, 0, &rules),
                Some(Decision::Stand)
            );
        }
    }

    #[test]
    fn test_hard_16() {
        let rules = Rules::default();

        for duc in 2..=11 {
            match duc {
                2..=6 => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 6], duc, 0, 0, &rules),
                        Some(Decision::Stand)
                    );
                }
                _ => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 6], duc, 0, 0, &rules),
                        Some(Decision::Hit)
                    );
                }
            }
        }
    }

    #[test]
    fn test_hard_15() {
        let rules = Rules::default();

        for duc in 2..=11 {
            match duc {
                2..=6 => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 5], duc, 0, 0, &rules),
                        Some(Decision::Stand)
                    );
                }
                _ => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 5], duc, 0, 0, &rules),
                        Some(Decision::Hit)
                    );
                }
            }
        }
    }

    #[test]
    fn test_hard_14() {
        let rules = Rules::default();

        for duc in 2..=11 {
            match duc {
                2..=6 => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 4], duc, 0, 0, &rules),
                        Some(Decision::Stand)
                    );
                }
                _ => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 4], duc, 0, 0, &rules),
                        Some(Decision::Hit)
                    );
                }
            }
        }
    }

    #[test]
    fn test_hard_13() {
        let rules = Rules::default();

        for duc in 2..=11 {
            match duc {
                2..=6 => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 3], duc, 0, 0, &rules),
                        Some(Decision::Stand)
                    );
                }
                _ => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 3], duc, 0, 0, &rules),
                        Some(Decision::Hit)
                    );
                }
            }
        }
    }

    #[test]
    fn test_hard_12() {
        let rules = Rules::default();

        for duc in 2..=11 {
            match duc {
                4..=6 => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 2], duc, 0, 0, &rules),
                        Some(Decision::Stand)
                    );
                }
                _ => {
                    assert_eq!(
                        hard_totals_chart(&vec![10, 2], duc, 0, 0, &rules),
                        Some(Decision::Hit)
                    );
                }
            }
        }
    }

    #[test]
    fn test_hard_11() {
        let rules = Rules::default();

        for duc in 2..=11 {
            match duc {
                2..=11 => {
                    assert_eq!(
                        hard_totals_chart(&vec![5, 6], duc, 0, 0, &rules),
                        Some(Decision::Double)
                    );
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_hard_10() {
        let rules = Rules::default();

        for duc in 2..=11 {
            match duc {
                2..=9 => {
                    assert_eq!(
                        hard_totals_chart(&vec![5, 5], duc, 0, 0, &rules),
                        Some(Decision::Double)
                    );
                }
                _ => {
                    assert_eq!(
                        hard_totals_chart(&vec![5, 5], duc, 0, 0, &rules),
                        Some(Decision::Hit)
                    );
                }
            }
        }
    }

    #[test]
    fn test_hard_9() {
        let rules = Rules::default();

        for duc in 2..=11 {
            match duc {
                3..=6 => {
                    assert_eq!(
                        hard_totals_chart(&vec![5, 4], duc, 0, 0, &rules),
                        Some(Decision::Double)
                    );
                }
                _ => {
                    assert_eq!(
                        hard_totals_chart(&vec![5, 4], duc, 0, 0, &rules),
                        Some(Decision::Hit)
                    );
                }
            }
        }
    }

    #[test]
    fn test_hard_8() {
        let rules = Rules::default();

        for duc in 2..=11 {
            assert_eq!(
                hard_totals_chart(&vec![5, 3], duc, 0, 0, &rules),
                Some(Decision::Hit)
            );
        }
    }

    #[test]
    fn test_hard_deviations_12() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(Deviations::Standard);

        assert_eq!(
            hard_totals_chart(&vec![10, 2], 4, -1, 0, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![10, 2], 2, 30, 3, &rules),
            Some(Decision::Stand)
        );

        assert_eq!(
            hard_totals_chart(&vec![10, 2], 3, 30, 2, &rules),
            Some(Decision::Stand)
        );
    }

    #[test]
    fn test_hard_deviations_13() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(Deviations::Standard);

        assert_eq!(
            hard_totals_chart(&vec![10, 3], 2, 30, -1, &rules),
            Some(Decision::Hit)
        );
    }

    #[test]
    fn test_hard_deviations_10() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(Deviations::Standard);

        assert_eq!(
            hard_totals_chart(&vec![7, 3], 10, 30, 4, &rules),
            Some(Decision::Double)
        );

        assert_eq!(
            hard_totals_chart(&vec![7, 3], 11, 30, 3, &rules),
            Some(Decision::Double)
        );

        rules.set_game_type(GameType::Stand17);

        assert_eq!(
            hard_totals_chart(&vec![7, 3], 11, 30, 4, &rules),
            Some(Decision::Double)
        );
    }

    #[test]
    fn test_hard_deviations_9() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(Deviations::Standard);

        // before
        assert_eq!(
            hard_totals_chart(&vec![4, 5], 2, 30, 0, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![4, 5], 2, 30, 1, &rules),
            Some(Decision::Double)
        );

        // before
        assert_eq!(
            hard_totals_chart(&vec![4, 5], 7, 30, 2, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![4, 5], 7, 30, 3, &rules),
            Some(Decision::Double)
        );
    }

    #[test]
    fn test_hard_deviations_8() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(Deviations::Standard);

        // before
        assert_eq!(
            hard_totals_chart(&vec![4, 4], 6, 30, 0, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![4, 4], 6, 30, 2, &rules),
            Some(Decision::Double)
        );

        // before
        assert_eq!(
            hard_totals_chart(&vec![3, 5], 5, 30, 3, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![3, 5], 5, 30, 4, &rules),
            Some(Decision::Hit)
        );

        rules.set_enable_deviations(Deviations::Extended1);

        // before
        assert_eq!(
            hard_totals_chart(&vec![3, 5], 5, 30, 3, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![3, 5], 5, 30, 4, &rules),
            Some(Decision::Double)
        );
    }

    #[test]
    fn test_hard_deviations_16() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(Deviations::Standard);

        // before
        assert_eq!(
            hard_totals_chart(&vec![10, 6], 9, 30, 3, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![10, 6], 9, 30, 4, &rules),
            Some(Decision::Stand)
        );

        // before
        assert_eq!(
            hard_totals_chart(&vec![10, 6], 10, -1, 0, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![10, 6], 10, 1, 0, &rules),
            Some(Decision::Stand)
        );

        // before
        assert_eq!(
            hard_totals_chart(&vec![10, 6], 11, 20, 2, &rules),
            Some(Decision::Hit)
        );

        rules.set_game_type(GameType::Stand17);

        assert_eq!(
            hard_totals_chart(&vec![10, 6], 11, 20, 3, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![10, 6], 11, 20, 7, &rules),
            Some(Decision::Hit)
        );

        rules.set_game_type(GameType::Hit17);

        assert_eq!(
            hard_totals_chart(&vec![10, 6], 11, 20, 3, &rules),
            Some(Decision::Stand)
        );
    }

    #[test]
    fn test_hard_deviations_15() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(Deviations::Standard);

        // before
        assert_eq!(
            hard_totals_chart(&vec![10, 2, 3], 10, 30, 3, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![10, 3, 2], 10, 30, 4, &rules),
            Some(Decision::Stand)
        );

        // before
        assert_eq!(
            hard_totals_chart(&vec![10, 2, 3], 11, 30, 4, &rules),
            Some(Decision::Hit)
        );

        assert_eq!(
            hard_totals_chart(&vec![10, 3, 2], 11, 30, 5, &rules),
            Some(Decision::Stand)
        );

        rules.set_game_type(GameType::Stand17);

        assert_eq!(
            hard_totals_chart(&vec![10, 3, 2], 11, 30, 9, &rules),
            Some(Decision::Hit)
        );
    }
}
