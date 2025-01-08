use crate::types::{GameType, Rules};

pub fn should_i_surrender(
    cards_in_hand: &Vec<u8>,
    dealer_up_card: u8,
    running_count: isize,
    true_count: isize,
    rules: &Rules,
) -> bool {
    let total: u8 = cards_in_hand.iter().sum();

    if cards_in_hand.len() > 2 || cards_in_hand.contains(&11) || (total < 14 || total > 17) {
        return false;
    }

    if total == 17 && rules.game_type == GameType::Hit17 && dealer_up_card == 11 {
        return true;
    }

    if total == 16 {
        if dealer_up_card == 9 || dealer_up_card == 10 || dealer_up_card == 11 {
            if dealer_up_card == 9 && true_count <= -1 {
                return false;
            }

            return true;
        }

        if dealer_up_card == 8 && true_count >= 4 {
            return true;
        }
    }

    if total == 15 {
        if dealer_up_card == 10 {
            if running_count < 0 {
                return false;
            }

            return true;
        }

        if dealer_up_card == 9 && true_count >= 2 {
            return true;
        }

        if dealer_up_card == 11 {
            match rules.game_type {
                GameType::Hit17 if true_count >= -1 => return true,
                GameType::Stand17 if true_count >= 2 => return true,
                _ => (),
            }
        }
    }

    if total == 14 {
        match rules.game_type {
            GameType::Hit17 => {
                if (true_count >= 6 && dealer_up_card == 9)
                    || (true_count >= 4 && dealer_up_card == 10)
                    || (true_count >= 4 && dealer_up_card == 11)
                {
                    return true;
                }
            }
            GameType::Stand17 => {
                if (true_count >= 7 && dealer_up_card == 9)
                    || (true_count >= 4 && dealer_up_card == 10)
                    || (true_count >= 6 && dealer_up_card == 11)
                {
                    return true;
                }
            }
        }
    };

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surrender_15_v_10() {
        let rules = Rules::default();

        assert!(should_i_surrender(&vec![10, 5], 10, 0, 0, &rules));

        // deviation
        assert!(!should_i_surrender(&vec![10, 5], 10, -1, 0, &rules));
    }

    #[test]
    fn test_surrender_15_v_9() {
        let rules = Rules::default();

        assert!(!should_i_surrender(&vec![10, 5], 9, 10, 1, &rules));

        // deviation
        assert!(should_i_surrender(&vec![10, 5], 9, 10, 2, &rules));
    }

    #[test]
    fn test_surrender_15_v_11() {
        let mut rules = Rules::default();

        assert!(!should_i_surrender(&vec![10, 5], 11, 10, -2, &rules));

        // deviation
        assert!(should_i_surrender(&vec![10, 5], 11, 10, -1, &rules));

        rules.game_type = GameType::Stand17;

        assert!(!should_i_surrender(&vec![10, 5], 11, 10, 1, &rules));

        assert!(should_i_surrender(&vec![10, 5], 11, 10, 2, &rules));
    }

    #[test]
    fn test_surrender_17_v_11() {
        let mut rules = Rules::default();

        assert!(should_i_surrender(&vec![10, 7], 11, 10, 0, &rules));

        rules.game_type = GameType::Stand17;

        // deviation
        assert!(!should_i_surrender(&vec![10, 7], 11, 10, 5, &rules));
    }

    #[test]
    fn test_surrender_16_v_8() {
        let mut rules = Rules::default();

        assert!(!should_i_surrender(&vec![10, 6], 8, 10, 3, &rules));

        assert!(should_i_surrender(&vec![10, 6], 8, 10, 4, &rules));

        rules.game_type = GameType::Stand17;

        assert!(!should_i_surrender(&vec![10, 6], 8, 10, 3, &rules));

        assert!(should_i_surrender(&vec![10, 6], 8, 10, 4, &rules));
    }

    #[test]
    fn test_surrender_16_v_9() {
        let mut rules = Rules::default();

        assert!(!should_i_surrender(&vec![10, 6], 9, 10, -1, &rules));

        assert!(should_i_surrender(&vec![10, 6], 9, 10, 0, &rules));

        rules.game_type = GameType::Stand17;

        assert!(!should_i_surrender(&vec![10, 6], 9, 10, -1, &rules));

        assert!(should_i_surrender(&vec![10, 6], 9, 10, 0, &rules));
    }

    #[test]
    fn test_surrender_16_v_10() {
        let mut rules = Rules::default();

        assert!(should_i_surrender(&vec![10, 6], 10, 10, -10, &rules));

        assert!(should_i_surrender(&vec![10, 6], 10, 10, 10, &rules));

        rules.game_type = GameType::Stand17;

        assert!(should_i_surrender(&vec![10, 6], 10, 10, -10, &rules));

        assert!(should_i_surrender(&vec![10, 6], 10, 10, 10, &rules));
    }

    #[test]
    fn test_surrender_16_v_11() {
        let mut rules = Rules::default();

        assert!(should_i_surrender(&vec![10, 6], 11, 10, -10, &rules));

        assert!(should_i_surrender(&vec![10, 6], 11, 10, 10, &rules));

        rules.game_type = GameType::Stand17;

        assert!(should_i_surrender(&vec![10, 6], 11, 10, -10, &rules));

        assert!(should_i_surrender(&vec![10, 6], 11, 10, 10, &rules));
    }

    #[test]
    fn test_surrender_14_v_9() {
        let mut rules = Rules::default();

        assert!(!should_i_surrender(&vec![10, 4], 9, 10, 0, &rules));

        assert!(should_i_surrender(&vec![10, 4], 9, 10, 6, &rules));

        rules.game_type = GameType::Stand17;

        assert!(!should_i_surrender(&vec![10, 4], 9, 10, 0, &rules));

        assert!(should_i_surrender(&vec![10, 4], 9, 10, 7, &rules));
    }

    #[test]
    fn test_surrender_14_v_10() {
        let mut rules = Rules::default();

        assert!(!should_i_surrender(&vec![10, 4], 10, 10, 0, &rules));

        assert!(should_i_surrender(&vec![10, 4], 10, 10, 4, &rules));

        rules.game_type = GameType::Stand17;

        assert!(!should_i_surrender(&vec![10, 4], 10, 10, 0, &rules));

        assert!(should_i_surrender(&vec![10, 4], 10, 10, 4, &rules));
    }

    #[test]
    fn test_surrender_14_v_11() {
        let mut rules = Rules::default();

        assert!(!should_i_surrender(&vec![10, 4], 11, 10, 0, &rules));

        assert!(should_i_surrender(&vec![10, 4], 11, 10, 4, &rules));

        rules.game_type = GameType::Stand17;

        assert!(!should_i_surrender(&vec![10, 4], 11, 10, 0, &rules));

        assert!(should_i_surrender(&vec![10, 4], 11, 10, 6, &rules));
    }
}
