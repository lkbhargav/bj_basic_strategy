pub mod hard_totals_chart;
pub mod insurance;
pub mod soft_totals_chart;
pub mod splits_chart;
pub mod surrender;
pub mod types;

pub fn compute_hand(cards: &mut Vec<u8>) -> (u8, bool) {
    // Check for blackjack first (natural 21)
    if cards.len() == 2 {
        let total = cards.iter().sum::<u8>();
        if total == 21 {
            return (21, true);
        }
    }

    // Count aces and calculate total with all aces as 1
    let mut ace_count = 0;
    let mut total = 0;

    for &card in cards.iter() {
        if card == 11 {
            ace_count += 1;
            total += 1; // Count ace as 1 initially
        } else {
            total += card;
        }
    }

    // Try to use one ace as 11 if it doesn't bust
    if ace_count > 0 && total + 10 <= 21 {
        total += 10; // Convert one ace from 1 to 11

        // Update the cards vector to reflect this
        let mut aces_converted = 0;
        for card in cards.iter_mut() {
            if *card == 11 {
                if aces_converted == 0 {
                    *card = 11; // Keep one ace as 11
                    aces_converted += 1;
                } else {
                    *card = 1; // Convert remaining aces to 1
                }
            }
        }
    } else {
        // Convert all aces to 1 in the cards vector
        for card in cards.iter_mut() {
            if *card == 11 {
                *card = 1;
            }
        }
    }

    (total, total <= 21)
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_compute_hand() {
        let cards = &mut vec![11, 10];

        let v = compute_hand(cards);

        assert!(v.1);
        assert_eq!(v.0, 21);
    }

    #[test]
    fn test_compute_hand_2() {
        let cards = &mut vec![11, 10, 9];

        let v = compute_hand(cards);

        assert!(v.1);
        assert_eq!(v.0, 20);
        assert_eq!(cards, &mut vec![1, 10, 9]);
    }

    #[test]
    fn test_compute_hand_3() {
        let cards = &mut vec![11, 10, 6, 11];

        let v = compute_hand(cards);

        assert!(v.1);
        assert_eq!(v.0, 18);
        assert_eq!(cards, &mut vec![1, 10, 6, 1]);
    }

    #[test]
    fn test_compute_hand_4() {
        let cards = &mut vec![10, 6, 10];

        let v = compute_hand(cards);

        assert!(!v.1);
        assert_eq!(v.0, 26);
        assert_eq!(cards, &mut vec![10, 6, 10]);
    }

    #[test]
    fn test_compute_hand_5() {
        let cards = &mut vec![11, 5, 2, 8, 10];

        let v = compute_hand(cards);

        assert_eq!(v.0, 26);
        assert!(!v.1);
        assert_eq!(cards, &mut vec![1, 5, 2, 8, 10]);
    }

    #[test]
    fn test_compute_hand_6() {
        let cards = &mut vec![11, 11, 11, 11, 11, 4];

        let v = compute_hand(cards);

        assert_eq!(v.0, 19);
        assert!(v.1);
        assert_eq!(cards, &mut vec![11, 1, 1, 1, 1, 4]);
    }

    #[test]
    fn test_compute_hand_7() {
        let cards = &mut vec![11, 2, 4, 11];

        let v = compute_hand(cards);

        assert_eq!(v.0, 18);
        assert!(v.1);
        assert_eq!(cards, &mut vec![11, 2, 4, 1]);
    }

    #[test]
    fn test_compute_hand_8() {
        let cards = &mut vec![11, 2, 4, 11, 10];

        let v = compute_hand(cards);

        assert_eq!(v.0, 18);
        assert!(v.1);
        assert_eq!(cards, &mut vec![1, 2, 4, 1, 10]);
    }

    #[test]
    fn test_compute_hand_9() {
        let cards = &mut vec![11, 5, 2, 8];

        let v = compute_hand(cards);

        assert_eq!(v.0, 16);
        assert!(v.1);
        assert_eq!(cards, &mut vec![1, 5, 2, 8]);
    }

    #[test]
    fn test_compute_hand_10() {
        let cards = &mut vec![11, 6];

        let v = compute_hand(cards);

        assert_eq!(v.0, 17);
        assert!(v.1);
        assert_eq!(cards, &mut vec![11, 6]);
    }

    #[test]
    fn test_compute_hand_11() {
        let cards = &mut vec![11, 9];

        let v = compute_hand(cards);

        assert_eq!(v.0, 20);
        assert!(v.1);
        assert_eq!(cards, &mut vec![11, 9]);
    }
}
