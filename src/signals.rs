use std::fmt;

pub enum Signals {
    InducesSuicide,
    OccupiedCase,
    BreakingKo,
    BreakingSuperKo,
    OutsideBounds,
    GameOver,
    DoublePass,
}

impl fmt::Display for Signals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signals::InducesSuicide => write!(f, "Move induces suicide with no capture"),
            Signals::OccupiedCase => write!(f, "Move on an occupied case"),
            Signals::BreakingKo => write!(f, "Ko rule is not respected"),
            Signals::BreakingSuperKo => write!(f, "Super Ko rule is not respected"),
            Signals::OutsideBounds => write!(f, "Tried to place a stone outside of board"),
            Signals::GameOver => write!(f, "Game is over"),
            Signals::DoublePass => write!(f, "Both players have passed"),
        }
    }
}

