use crate::{domain::Note, infrastructure::NoteRepo};
use actix_web::{
    web::{self, Json},
    Error,
};

pub async fn get_note_or_inspire(note_repo: web::Data<NoteRepo>) -> Result<Json<Note>, Error> {
    if let Ok(note) = note_repo.get_note() {
        Ok(Json(note))
    } else {
        let note = note_repo.get_inspire_quote().await?;
        Ok(Json(note))
    }
}
