use crate::domain::{Note, ZenQuote};
use actix_web::{error, Error};
use chrono::Utc;
use std::fs;
use tracing::{debug, error};

pub struct NoteRepo {
    note_path: std::path::PathBuf,
}

impl NoteRepo {
    pub fn new(note_path: std::path::PathBuf) -> Self {
        Self { note_path }
    }

    pub async fn get_inspire_quote(&self) -> Result<Note, Error> {
        let err_fn = |err: reqwest::Error| {
            error!("request zenquotes.io: {}", err);
            error::ErrorInternalServerError("error request zenquotes.io")
        };

        let resp = reqwest::get("https://zenquotes.io/api/today")
            .await
            .map_err(|err| err_fn(err));
        resp?
            .json::<ZenQuote>()
            .await
            .map(|q| {
                let quote = q.first().unwrap();
                Note::from_org_to_html(quote.to_org())
            })
            .map_err(|err| err_fn(err))
    }

    fn get_today_note_path(&self) -> std::path::PathBuf {
        let now = Utc::now();
        let today_str = now.format("%Y-%m-%d");
        let mut note_path = self.note_path.clone();
        note_path.push(today_str.to_string());
        note_path.set_extension("org");
        note_path
    }

    pub fn get_note(&self) -> Result<Note, Error> {
        let note_path = self.get_today_note_path();
        fs::read_to_string(note_path)
            .map(|org_string| Note::from_org_to_html(org_string))
            .map_err(|err| {
                debug!("Error read daily note file: {:?}", err);
                error::ErrorNotFound("not found daily note file")
            })
    }
}
