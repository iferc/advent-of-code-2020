use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Fail,
}

impl LogLevel {
    pub fn help() -> &'static str {
        "Pass `-h` and you'll see me!"
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogLevel::Debug => "debug",
                LogLevel::Info => "info",
                LogLevel::Warn => "warn",
                LogLevel::Fail => "fail",
            }
        )
    }
}

impl FromStr for LogLevel {
    type Err = &'static str;

    fn from_str(level_str: &str) -> Result<Self, Self::Err> {
        match level_str {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "fail" => Ok(LogLevel::Fail),
            _ => Err("Invalid logging level"),
        }
    }
}
