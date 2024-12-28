use axum::{
    Router,
    routing::{post, get},
    Json,
    extract::State,
};

pub fn router() -> Router {
    Router::new()
        .route("/start", post(start_interview))
        .route("/answer", post(process_answer))
        .route("/complete", post(complete_interview))
        .route("/questions/:country", get(get_country_questions))
}

async fn start_interview(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<StartInterviewRequest>,
) -> Result<Json<InterviewSession>> {
    // Initialize interview session
    let session = state.interview_service.start_interview(payload.country_code).await?;
    Ok(Json(session))
}

async fn process_answer(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AnswerRequest>,
) -> Result<Json<InterviewResponse>> {
    // Process answer and get next question
    let response = state.interview_service.process_answer(
        payload.session_id,
        payload.answer
    ).await?;
    Ok(Json(response))
}

async fn complete_interview(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CompleteInterviewRequest>,
) -> Result<Json<ListingCreationResult>> {
    // Finalize interview and trigger API key generation/email
    let result = state.interview_service.complete_interview(payload.session_id).await?;
    Ok(Json(result))
} 