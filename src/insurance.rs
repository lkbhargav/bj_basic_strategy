// * Returned decision should be used for both insurance and even money
pub fn should_i_take_insurance(
    cards_in_hand: &Vec<u8>,
    dealer_up_card: u8,
    true_count: isize,
) -> bool {
    return dealer_up_card == 11 && cards_in_hand.len() == 2 && true_count >= 3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_take_insurance() {
        assert!(should_i_take_insurance(&vec![9, 2], 11, 3));
        assert!(should_i_take_insurance(&vec![10, 10], 11, 5));
        assert!(should_i_take_insurance(&vec![2, 2], 11, 4));
    }

    #[test]
    fn should_not_take_insurance() {
        assert!(!should_i_take_insurance(&vec![9, 2], 11, 1));
        assert!(!should_i_take_insurance(&vec![10, 10], 11, 2));
        assert!(!should_i_take_insurance(&vec![2, 2], 11, 1));

        assert!(!should_i_take_insurance(&vec![10, 10], 10, 5));
    }
}
