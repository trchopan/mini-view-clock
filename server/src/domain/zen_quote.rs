use serde::{Deserialize, Serialize};

pub type ZenQuote = Vec<ZenQuoteElement>;

#[derive(Serialize, Deserialize)]
pub struct ZenQuoteElement {
    pub q: String,
    pub a: String,
    pub h: String,
}

impl ZenQuoteElement {
    pub fn to_org(&self) -> String {
        format!("* {}\n{}", self.a, self.q)
    }
}
