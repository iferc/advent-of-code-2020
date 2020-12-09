// Patterns to match:
// - COLOUR bags contain NUMBER COLOUR bags, NUMBER COLOUR bags.
//                       likely repetitive --^
// - COLOUR bags contain NUMBER COLOUR bag, NUMBER COLOUR bags.
// - COLOUR bags contain NUMBER COLOUR bag.
// - COLOUR bags contain no other bags.

use std::collections::HashMap;
use std::str::FromStr;

peg::parser!(pub grammar bags_parser() for str {
    rule number() -> i64
        = n:$(['0'..='9']+)
            { n.parse().unwrap() }

    rule colour() -> String
        = c:$(['a'..='z'|'A'..='Z']+ " " ['a'..='z'|'A'..='Z']+)
            { c.to_string() }

    rule bag_set() -> (i64, String)
        = n:number() " " c:colour() " bag" "s"? (", " / ".")
            { (n, c.to_string()) }

    rule bag_sets() -> Vec<(i64, String)>
        = bs:bag_set()+
            { bs }
        / "no other bags."
            { Vec::new() }

    pub rule line() -> (String, Vec<(i64, String)>)
        = mc:colour() " bags contain " bs:bag_sets()
            { ( mc.to_string(), bs ) }
});

pub struct Bag(pub String, pub Vec<(i64, String)>);
pub struct Bags(pub HashMap<String, Vec<(i64, String)>>);

impl Bags {
    pub fn find_parents(&self, mut map: HashMap<String, ()>, colour: &str) -> HashMap<String, ()> {
        for (key_colour, bag_rules) in &self.0 {
            for (_, bag_colour) in bag_rules {
                if bag_colour.as_str() == colour {
                    map.insert(key_colour.clone(), ());
                    map = self.find_parents(map, key_colour.as_str());
                    break;
                }
            }
        }

        map
    }

    pub fn silver_find(&self, colour: &str) -> usize {
        self.find_parents(HashMap::new(), colour).len()
    }

    pub fn gold_find(&self, colour: &str) -> i64 {
        let mut counter = 0;

        for (contains_count, contains_colour) in &self.0[colour] {
            counter += contains_count + contains_count * self.gold_find(contains_colour);
        }

        counter
    }
}

impl FromStr for Bag {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match bags_parser::line(text) {
            Err(error) => {
                return Err(format!(
                    "Failed to parse input string {:?}, error given: {:?}",
                    text, error
                ));
            }
            Ok(result) => Ok(Bag(result.0, result.1)),
        }
    }
}
