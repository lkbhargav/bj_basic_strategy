// * Returned decision should be used for both insurance and even money
pub fn should_i_take_insurance(dealer_up_card: u8, true_count: isize) -> bool {
    return dealer_up_card == 11 && true_count >= 3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_take_insurance() {
        assert!(should_i_take_insurance(11, 3));
        assert!(should_i_take_insurance(11, 5));
        assert!(should_i_take_insurance(11, 4));
    }

    #[test]
    fn should_not_take_insurance() {
        assert!(!should_i_take_insurance(11, 1));
        assert!(!should_i_take_insurance(11, 2));
        assert!(!should_i_take_insurance(11, 1));

        assert!(!should_i_take_insurance(10, 5));
    }
}
