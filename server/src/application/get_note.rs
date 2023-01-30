use crate::{
    domain::NoteHeader,
    infrastructure::{NoteRepo, NoteRepoError},
};
use actix_web::{
    error,
    web::{self, Json},
    Error,
};

pub async fn get_notes(note_repo: web::Data<NoteRepo>) -> Result<Json<Vec<NoteHeader>>, Error> {
    match note_repo.get_notion_tasklist().await {
        Err(err) => match err {
            NoteRepoError::NotionAPIError => Err(error::ErrorInternalServerError(
                "error request api.notion.com",
            )),
        },
        Ok(notes) => {
            if notes.is_empty() {
                return Err(error::ErrorNotFound("empty result from notion"));
            }
            Ok(Json(notes))
        }
    }
}
