use std::str::FromStr;

#[derive(Debug)]
pub enum View {
    Clock,
    Note,
}

impl FromStr for View {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "clock" => Ok(View::Clock),
            "note" => Ok(View::Note),
            _ => Err("invalid view value. valid values: clock, note".to_owned()),
        }
    }
}

impl ToString for View {
    fn to_string(&self) -> String {
        match self {
            View::Clock => "clock".to_owned(),
            View::Note => "note".to_owned(),
        }
    }
}
