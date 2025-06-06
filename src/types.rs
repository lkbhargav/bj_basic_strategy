use proc_macros::{Random, ValueAssigner};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug, Default, PartialEq, Random, ValueAssigner)]
pub enum GameType {
    #[default]
    Hit17,
    Stand17,
}

#[derive(Clone, Debug, Default, Random, ValueAssigner)]
pub enum SplitAces {
    No,
    #[default]
    SplitAcesOnce,
    ReSplitAces,
}

#[derive(Clone, Debug, Default, Random, ValueAssigner)]
pub enum BlackjackPayout {
    #[default]
    ThreeToTwo,
    SixToFive,
    Even,
}

#[derive(Clone, Debug, Default, PartialEq, Random, ValueAssigner)]
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

#[derive(Clone, Debug, Default, Random, ValueAssigner)]
pub enum PlayVariation {
    #[default]
    PlayEverything,
    AvoidPlayingNegativeTrueCounts,
    PlayAboveNegativeTrueTwo,
    PlayOnlyPositiveTrueCounts,
    PlayAnyPositiveRunningCount,
}

impl PlayVariation {
    pub fn play(&self, true_count: i32, running_count: i32) -> bool {
        match self {
            PlayVariation::PlayEverything => true,
            PlayVariation::AvoidPlayingNegativeTrueCounts if true_count >= 0 => true,
            PlayVariation::PlayAboveNegativeTrueTwo if true_count >= -1 => true,
            PlayVariation::PlayOnlyPositiveTrueCounts if true_count > 0 => true,
            PlayVariation::PlayAnyPositiveRunningCount if running_count > 0 => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Default, Random, ValueAssigner)]
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
    pub number_of_players: u8,
    pub enable_deviations: bool,
    pub play_variation: PlayVariation,
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
            number_of_players: 1,
            enable_deviations: false,
            play_variation: Default::default(),
        }
    }
}

impl Rules {
    pub fn enable_deviations(mut self, enable_deviations: bool) -> Self {
        self.enable_deviations = enable_deviations;
        self
    }

    pub fn set_number_of_players(mut self, number_of_players: i32) -> Result<Self, String> {
        if number_of_players < 1 || number_of_players > 5 {
            return Err(
                "Valid number of players can be 1 to 5, please fix the error and try again"
                    .to_string(),
            );
        }

        self.number_of_players = number_of_players as u8;

        Ok(self)
    }

    pub fn encore_boston_playable() -> Self {
        let mut rules = Rules::default();

        rules.game_type = GameType::Hit17;
        rules.split_aces = SplitAces::SplitAcesOnce;
        rules.decks = 8;
        rules.blackjack_payout = BlackjackPayout::ThreeToTwo;
        rules.deck_pen = DeckPen::Two;
        rules.enable_deviations = true;

        rules = rules.set_number_of_players(1).unwrap();

        rules
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Random, ValueAssigner)]
#[wasm_bindgen]
pub enum Decision {
    #[default]
    Stand,
    Hit,
    Split,
    Double,
    Surrender,
    GotBJ,
}

impl Copy for Decision {}
