use actix_web::{error, Error};

#[derive(Debug)]
pub enum View {
    Clock,
    Note,
}

impl TryFrom<String> for View {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "clock" => Ok(View::Clock),
            "note" => Ok(View::Note),
            _ => Err(error::ErrorBadRequest("invalid view value")),
        }
    }
}
