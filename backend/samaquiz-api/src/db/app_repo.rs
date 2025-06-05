use std::sync::Arc;

use lib_api::{
    db::{db_error::DbError, db_setup::app_db_connect},
    error::api_error::ApiError,
};
use sqlx::{PgPool, Postgres, Transaction};

use super::{
    answer_repo::{AnswerRepo, DynAnswerRepo},
    participant_repo::{DynParticipantRepo, ParticipantRepo},
    question_repo::{DynQuestionRepo, QuestionRepo},
    quiz_asset_repo::{DynQuizAssetRepo, QuizAssetRepo},
    quiz_repo::{DynQuizRepo, QuizRepo},
    quiz_response_repo::{DynQuizResponseRepo, QuizResponseRepo},
    quiz_session_repo::{DynQuizSessionRepo, QuizSessionRepo},
    user_repo::{DynUserRepo, UserRepo},
};

#[derive(Clone)]
pub struct AppRepo {
    pub db: PgPool,
    pub user: DynUserRepo,
    pub quiz: DynQuizRepo,
    pub question: DynQuestionRepo,
    pub quiz_asset: DynQuizAssetRepo,
    pub answer: DynAnswerRepo,
    pub quiz_session: DynQuizSessionRepo,
    pub participant: DynParticipantRepo,
    pub quiz_response: DynQuizResponseRepo,
}

pub async fn start_transaction(db: &PgPool) -> Result<Transaction<'_, Postgres>, DbError> {
    let transaction = db.begin().await.map_err(|e| DbError::SqlxError(e))?;
    Ok(transaction)
}

impl AppRepo {
    pub async fn new(db_url: &str, db_name: &str) -> Result<Self, DbError> {
        let db_url = format!("{}{}", db_url, db_name);
        let db = app_db_connect(&db_url, db_name).await?;

        Ok(AppRepo {
            db: db.clone(),
            user: Arc::new(UserRepo { db: db.clone() }) as DynUserRepo,
            quiz: Arc::new(QuizRepo { db: db.clone() }) as DynQuizRepo,
            question: Arc::new(QuestionRepo { db: db.clone() }) as DynQuestionRepo,
            quiz_asset: Arc::new(QuizAssetRepo { db: db.clone() }) as DynQuizAssetRepo,
            answer: Arc::new(AnswerRepo { db: db.clone() }) as DynAnswerRepo,
            quiz_session: Arc::new(QuizSessionRepo { db: db.clone() }) as DynQuizSessionRepo,
            quiz_response: Arc::new(QuizResponseRepo { db: db.clone() }) as DynQuizResponseRepo,
            participant: Arc::new(ParticipantRepo { db: db.clone() }) as DynParticipantRepo,
        })
    }

    pub async fn start_transaction(&self) -> Result<Transaction<'_, Postgres>, ApiError> {
        start_transaction(&self.db)
            .await
            .map_err(|e| ApiError::internal_error().message(e))
    }
}
