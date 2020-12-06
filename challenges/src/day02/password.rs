use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct PasswordPolicy {
    pub amount_range: Range<usize>,
    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Password {
    pub policy: PasswordPolicy,
    pub password: String,
}

impl Password {
    pub fn new(password: &str) -> Result<Password, String> {
        let segments = password.split_whitespace().collect::<Vec<_>>();

        if segments.len() != 3 {
            return Err(format!("Unable to parse input line: {}", password));
        }

        let amount_range = str_to_range(segments[0])?;
        let symbol = {
            let mut symbol_chars = segments[1].to_string();
            symbol_chars.pop();
            symbol_chars
        };

        Ok(Password {
            policy: PasswordPolicy {
                amount_range,
                symbol,
            },
            password: segments[2].to_string(),
        })
    }
}

fn str_to_range(s: &str) -> Result<Range<usize>, String> {
    let pair: Result<Vec<usize>, _> = s
        .split('-')
        .map(|string| string.parse())
        .collect::<Vec<_>>()
        .into_iter()
        .collect();

    match pair {
        Err(_) => Err(format!("Failed to pair string to range: {}.", s)),
        Ok(endpoints) if endpoints.len() != 2 => {
            Err(format!("Failed to pair string to range: {}.", s))
        }
        Ok(endpoints) => Ok(Range {
            start: endpoints[0],
            end: endpoints[1],
        }),
    }
}
