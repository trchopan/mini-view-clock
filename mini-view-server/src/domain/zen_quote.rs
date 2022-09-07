use serde::{Serialize, Deserialize};

pub type ZenQuote = Vec<ZenQuoteElement>;

#[derive(Serialize, Deserialize)]
pub struct ZenQuoteElement {
    pub q: String,
    pub a: String,
    pub h: String,
}

