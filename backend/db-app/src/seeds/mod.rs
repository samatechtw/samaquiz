use lib_api::db::db_error::DbError;
use sqlx::PgPool;

pub mod s010_users;
pub mod s020_quizzes;
pub mod s025_quiz_assets;
pub mod s030_questions;
pub mod s035_question_assets;
pub mod s040_answers;

pub async fn seed_all(db: &PgPool) -> Result<(), DbError> {
    s010_users::seed(db).await?;
    s020_quizzes::seed(db).await?;
    s025_quiz_assets::seed(db).await?;
    s030_questions::seed(db).await?;
    s035_question_assets::seed(db).await?;
    s040_answers::seed(db).await?;
    Ok(())
}
