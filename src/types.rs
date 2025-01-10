use serde::{Deserialize, Serialize};

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
    pub enable_deviations: bool,
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
            enable_deviations: false,
        }
    }
}

impl Rules {
    pub fn enable_deviations(mut self, enable_deviations: bool) -> Self {
        self.enable_deviations = enable_deviations;
        self
    }

    pub fn encore_boston_playable() -> Self {
        let mut rules = Rules::default();

        rules.game_type = GameType::Hit17;
        rules.split_aces = SplitAces::SplitAcesOnce;
        rules.decks = 8;
        rules.blackjack_payout = BlackjackPayout::ThreeToTwo;
        rules.deck_pen = DeckPen::Two;
        rules.number_of_hands = 2;
        rules.enable_deviations = true;

        rules
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Decision {
    Stand,
    Hit,
    Split,
    Double,
    Surrender,
    GotBJ,
}
