pub const PASSPORT_SEPARATOR: &'static str = "\n\n";

#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementUnit {
    Centimeters,
    Inches,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    length: u32,
    unit: MeasurementUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicPassport {
    birth_year: Option<String>,      // byr (Birth Year)
    issue_year: Option<String>,      // iyr (Issue Year)
    expiration_year: Option<String>, // eyr (Expiration Year)
    height: Option<String>,          // hgt (Height)
    hair_colour: Option<String>,     // hcl (Hair Color)
    eye_colour: Option<String>,      // ecl (Eye Color)
    passport_id: Option<String>,     // pid (Passport ID)
    country_id: Option<String>,      // cid (Country ID)
}

impl BasicPassport {
    pub fn new(possible_passport: &str) -> Result<Self, String> {
        let field_pairs: Vec<_> = possible_passport.split_whitespace().collect();

        let mut passport = BasicPassport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_colour: None,
            eye_colour: None,
            passport_id: None,
            country_id: None,
        };

        for field_pair in field_pairs {
            let field_components: Vec<_> = field_pair.split(":").collect();

            if field_components.len() != 2 {
                return Err("Key-value pair has too many deliminators.".to_string());
            }

            match field_components[0] {
                "byr" => passport.birth_year = Some(field_components[1].to_string()),
                "iyr" => passport.issue_year = Some(field_components[1].to_string()),
                "eyr" => passport.expiration_year = Some(field_components[1].to_string()),
                "hgt" => passport.height = Some(field_components[1].to_string()),
                "hcl" => passport.hair_colour = Some(field_components[1].to_string()),
                "ecl" => passport.eye_colour = Some(field_components[1].to_string()),
                "pid" => passport.passport_id = Some(field_components[1].to_string()),
                "cid" => passport.country_id = Some(field_components[1].to_string()),
                _ => return Err("Unrecognized key in key-value pair.".to_string()),
            }
        }

        Ok(passport)
    }

    pub fn has_required_fields(&self) -> bool {
        self.birth_year != None
            && self.issue_year != None
            && self.expiration_year != None
            && self.height != None
            && self.hair_colour != None
            && self.eye_colour != None
            && self.passport_id != None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StrictPassport {
    birth_year: Option<u32>,      // byr (Birth Year)
    issue_year: Option<u32>,      // iyr (Issue Year)
    expiration_year: Option<u32>, // eyr (Expiration Year)
    height: Option<Measurement>,  // hgt (Height)
    hair_colour: Option<String>,  // hcl (Hair Color)
    eye_colour: Option<String>,   // ecl (Eye Color)
    passport_id: Option<String>,  // pid (Passport ID)
    country_id: Option<String>,   // cid (Country ID)
}

impl StrictPassport {
    pub fn is_valid(&self) -> bool {
        self.birth_year != None
            && self.issue_year != None
            && self.expiration_year != None
            && self.height != None
            && self.hair_colour != None
            && self.eye_colour != None
            && self.passport_id != None
    }
}

impl From<BasicPassport> for StrictPassport {
    fn from(basic_passport: BasicPassport) -> Self {
        let _debug = basic_passport.clone();
        let birth_year: Option<u32> = match basic_passport.birth_year {
            None => None,
            Some(year_text) => match year_text.parse() {
                Ok(year) if year >= 1920 && year <= 2002 => Some(year),
                _ => None,
            },
        };

        let issue_year: Option<u32> = match basic_passport.issue_year {
            None => None,
            Some(year_text) => match year_text.parse() {
                Ok(year) if year >= 2010 && year <= 2020 => Some(year),
                _ => None,
            },
        };

        let expiration_year: Option<u32> = match basic_passport.expiration_year {
            None => None,
            Some(year_text) => match year_text.parse() {
                Ok(year) if year >= 2020 && year <= 2030 => Some(year),
                _ => None,
            },
        };

        let height: Option<Measurement> = match basic_passport.height {
            None => None,
            Some(height_text) => {
                let digits: Vec<_> = height_text.chars().collect();
                let digits_length = digits.len();

                let unit_text: String = digits.iter().rev().take(2).rev().collect();
                let unit = match unit_text.as_str() {
                    "cm" => Some(MeasurementUnit::Centimeters),
                    "in" => Some(MeasurementUnit::Inches),
                    _ => None,
                };

                let mut my_god_fucking_damn_length_ffs = None;
                if let Some(unit) = &unit {
                    let value: String = digits.iter().take(digits_length - 2).collect();
                    my_god_fucking_damn_length_ffs = match value.parse() {
                        Ok(length)
                            if unit == &MeasurementUnit::Centimeters
                                && length >= 150
                                && length <= 193 =>
                        {
                            Some(length)
                        }
                        Ok(length)
                            if unit == &MeasurementUnit::Inches && length >= 59 && length <= 76 =>
                        {
                            Some(length)
                        }
                        _x => {
                            // println!("x is {:#?} from {:?}", x, value);
                            None
                        }
                    };
                }

                match (my_god_fucking_damn_length_ffs, unit) {
                    (Some(length), Some(unit)) => Some(Measurement { length, unit }),
                    _ => None,
                }
            }
        };

        let hair_colour: Option<String> = match basic_passport.hair_colour {
            None => None,
            Some(colour_text) => {
                let mut is_valid = true;

                if colour_text.len() == 7 {
                    let digits: Vec<_> = colour_text.chars().collect();
                    if digits[0] == '#' {
                        for key in 1..digits.len() {
                            let ch = &digits[key];
                            if !('0'..='9').contains(ch)
                                && !('a'..='f').contains(ch)
                                && !('A'..='F').contains(ch)
                            {
                                is_valid = false;
                            }
                        }
                    } else {
                        is_valid = false;
                    }
                } else {
                    is_valid = false;
                }

                if is_valid {
                    Some(colour_text)
                } else {
                    None
                }
            }
        };

        let eye_colour: Option<String> = match basic_passport.eye_colour {
            None => None,
            Some(colour) => match colour.as_str() {
                "amb" => Some(colour),
                "blu" => Some(colour),
                "brn" => Some(colour),
                "gry" => Some(colour),
                "grn" => Some(colour),
                "hzl" => Some(colour),
                "oth" => Some(colour),
                _ => None,
            },
        };

        let passport_id: Option<String> = match basic_passport.passport_id {
            Some(id) if id.len() == 9 => match id.parse::<u64>() {
                Ok(_id_value) => Some(id),
                _ => None,
            },
            _ => None,
        };

        let x = Self {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_colour,
            eye_colour,
            passport_id,
            country_id: basic_passport.country_id,
        };

        // println!("{:#?}\n{:#?}\n", debug, x);
        x
    }
}
