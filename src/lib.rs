pub mod hard_totals_chart;
pub mod insurance;
pub mod soft_totals_chart;
pub mod splits_chart;
pub mod surrender;
pub mod types;

pub fn compute_hand(cards: &mut Vec<u8>) -> (u8, bool) {
    let total = cards.iter().sum::<u8>();

    if cards.len() == 2 && total == 21 {
        return (21, true);
    }

    if total > 21 && !cards.contains(&11) {
        return (total, false);
    }

    if cards.contains(&10) && cards.contains(&11) {
        for elem in &mut *cards {
            if *elem == 11 {
                *elem = 1;
            }
        }
    }

    let mut contains_a_11 = false;

    if !cards.contains(&10) && cards.contains(&11) {
        for elem in &mut *cards {
            if *elem == 11 {
                if contains_a_11 {
                    *elem = 1;
                    continue;
                }

                contains_a_11 = true;
                *elem = 11;
            }
        }
    }

    let mut total = cards.iter().sum();

    // if total goes over 21 and if it contains a 11, then adjust the return
    if contains_a_11 && total > 21 {
        total = total - 10;

        for card in cards {
            if card == &11 {
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
