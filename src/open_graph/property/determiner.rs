use strum::EnumString;
use serde::Serialize;

#[derive(Debug, PartialEq, EnumString, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Determiner {
    A,
    An,
    Auto,
    The,
    #[strum(serialize = "")]
    Blank,
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn determiner_from_str() {
        assert_eq!(Determiner::from_str("a").unwrap(), Determiner::A);
        assert_eq!(Determiner::from_str("").unwrap(), Determiner::Blank);
    }
}