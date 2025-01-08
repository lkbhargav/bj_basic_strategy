// * Returned decision should be used for both insurance and even money
pub fn should_i_take_insurance(
    cards_in_hand: &Vec<u8>,
    dealer_up_card: u8,
    true_count: isize,
) -> bool {
    return dealer_up_card == 11 && cards_in_hand.len() == 2 && true_count >= 3;
}
