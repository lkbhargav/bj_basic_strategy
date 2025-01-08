#[derive(Clone, Debug, Default, PartialEq)]
pub enum GameType {
    #[default]
    Hit17,
    Stand17,
}

#[derive(Clone, Debug, Default)]
pub enum SplitAces {
    No,
    #[default]
    SplitAcesOnce,
    ReSplitAces,
}

#[derive(Clone, Debug, Default)]
pub enum BlackjackPayout {
    #[default]
    ThreeToTwo,
    SixToFive,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum IsDoubleAllowed {
    No,
    NineAndTenOnly,
    NineTenAndAceOnly,
    #[default]
    Any,
}

impl IsDoubleAllowed {
    pub fn any(&self) -> bool {
        match self {
            IsDoubleAllowed::Any => true,
            _ => false,
        }
    }

    pub fn no(&self) -> bool {
        match self {
            IsDoubleAllowed::No => true,
            _ => false,
        }
    }

    pub fn double_on_nine(&self) -> bool {
        match self {
            IsDoubleAllowed::NineAndTenOnly
            | IsDoubleAllowed::NineTenAndAceOnly
            | IsDoubleAllowed::Any => true,
            _ => false,
        }
    }

    pub fn double_on_ten(&self) -> bool {
        match self {
            IsDoubleAllowed::NineAndTenOnly
            | IsDoubleAllowed::NineTenAndAceOnly
            | IsDoubleAllowed::Any => true,
            _ => false,
        }
    }

    pub fn double_on_ace(&self) -> bool {
        match self {
            IsDoubleAllowed::NineTenAndAceOnly | IsDoubleAllowed::Any => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum DeckPen {
    Quater = 13,
    Half = 26,
    ThreeFourth = 39,
    #[default]
    One = 52,
    OneQuater = 65,
    OneAndHalf = 78,
    OneThreeFourth = 91,
    Two = 104,
}

impl DeckPen {
    pub fn value(&self) -> usize {
        match self {
            DeckPen::Quater => 13,
            DeckPen::Half => 26,
            DeckPen::ThreeFourth => 39,
            DeckPen::One => 52,
            DeckPen::OneQuater => 65,
            DeckPen::OneAndHalf => 78,
            DeckPen::OneThreeFourth => 91,
            DeckPen::Two => 104,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rules {
    pub game_type: GameType,
    pub double_after_split: bool,
    pub split_aces: SplitAces,
    pub surrender: bool,
    pub decks: u8,
    pub blackjack_payout: BlackjackPayout,
    pub is_double_allowed: IsDoubleAllowed,
    pub max_splits_allowed: u8,
    pub deck_pen: DeckPen,
    pub number_of_hands: u8,
    // pub number_of_players_simulated: u8,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            game_type: Default::default(),
            double_after_split: true,
            split_aces: Default::default(),
            surrender: true,
            decks: 6,
            blackjack_payout: Default::default(),
            is_double_allowed: Default::default(),
            max_splits_allowed: 3,
            deck_pen: DeckPen::default(),
            number_of_hands: 1,
            // number_of_players_simulated: 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decision {
    Stand,
    Hit,
    Double,
    Surrender,
    GotBJ,
}
