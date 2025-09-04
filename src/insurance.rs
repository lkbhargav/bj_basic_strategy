use crate::types::Rules;

// * Returned decision should be used for both insurance and even money
pub fn should_i_take_insurance(dealer_up_card: u8, true_count: isize, rules: &Rules) -> bool {
    if !rules.enable_deviations() {
        return false;
    }

    dealer_up_card == 11 && true_count >= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_take_insurance() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(true);

        assert!(should_i_take_insurance(11, 3, &rules));
        assert!(should_i_take_insurance(11, 5, &rules));
        assert!(should_i_take_insurance(11, 4, &rules));
    }

    #[test]
    fn should_not_take_insurance() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(true);

        assert!(!should_i_take_insurance(11, 1, &rules));
        assert!(!should_i_take_insurance(11, 2, &rules));
        assert!(!should_i_take_insurance(11, 1, &rules));

        assert!(!should_i_take_insurance(10, 5, &rules));
    }

    #[test]
    fn disbaled_deviations() {
        let mut rules = Rules::default();
        rules.set_enable_deviations(false);

        assert!(!should_i_take_insurance(11, 3, &rules));
        assert!(!should_i_take_insurance(11, 5, &rules));
        assert!(!should_i_take_insurance(11, 4, &rules));
    }
}
