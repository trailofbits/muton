use chrono::DateTime;
use chrono::Utc;
use console::{StyledObject, style};
use strum::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
#[strum(serialize_all = "PascalCase")]
pub enum Status {
    // bad, mutant is valid
    Uncaught,
    // good, tests caught this mutant
    TestFail,
    // fine, this less severe mutant was skipped because a more severe mutatnt on the same line
    // was uncaught
    Skipped,
    // goodish, mutant broke the build (maybe mutant was bad?)
    BuildFail,
    // questionable, tests timed out before passing or failing
    Timeout,
}

impl Status {
    pub fn display(&self) -> StyledObject<String> {
        match &self {
            Status::Uncaught => style(self.to_string()).red().bold(),
            Status::TestFail => style(self.to_string()).green().bold(),
            Status::BuildFail => style(self.to_string()).yellow(),
            Status::Timeout => style(self.to_string()).yellow(),
            Status::Skipped => style(self.to_string()).blue(),
        }
    }
}

#[derive(Debug)]
pub struct Outcome {
    pub mutant_id: i64,
    pub status: Status,
    pub output: String,
    pub time: DateTime<Utc>,
    pub duration_ms: u32,
}
