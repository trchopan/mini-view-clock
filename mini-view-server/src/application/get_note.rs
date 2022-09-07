use crate::{
    domain::{MyAppError, Note},
    infrastructure::NoteRepo,
};
use actix_web::web::{self, Json};

pub async fn get_note_or_inspire(note_repo: web::Data<NoteRepo>) -> Result<Json<Note>, MyAppError> {
    if let Ok(note) = note_repo.get_note() {
        Ok(Json(note))
    } else {
        let note = note_repo.get_inspire_quote().await?;
        Ok(Json(note))
    }
}
