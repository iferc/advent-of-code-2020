mod day01;
pub use day01::Day01;

pub trait SilverChallenge {
    type Answer;
    fn attempt_silver(&self) -> Result<Self::Answer, String>;
}
pub trait GoldChallenge {
    type Answer;
    fn attempt_gold(&self) -> Result<Self::Answer, String>;
}
