use serde::Serialize;

#[derive(Serialize)]
pub struct QuizSessionLeader {
    pub name: String,
    pub avatar: String,
    pub points: i32,
}

#[derive(Serialize)]
pub struct GetQuizSessionLeadersResponse {
    pub leaders: Vec<QuizSessionLeader>,
}
