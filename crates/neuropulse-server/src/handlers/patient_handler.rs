use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use neuropulse_core::domain::models::{CreatePatientPayload, GenericResponse, Patient};

// Endpoint para cadastrar paciente no banco PostgreSQL
pub async fn create_patient_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePatientPayload>,
) -> Result<Json<GenericResponse>, (StatusCode, String)> {
    let birth_date = chrono::NaiveDate::parse_from_str(&payload.birth_date, "%Y-%m-%d")
        .map_err(|_| (StatusCode::BAD_REQUEST, "Data de nascimento inválida (use o formato YYYY-MM-DD)".to_string()))?;

    sqlx::query!(
        r#"
        INSERT INTO patients (full_name, medical_record_id, birth_date, notes)
        VALUES ($1, $2, $3, $4)
        "#,
        payload.full_name,
        payload.medical_record_id,
        birth_date,
        payload.notes
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao salvar paciente: {}", e)))?;

    Ok(Json(GenericResponse {
        status: "success".to_string(),
        message: "Paciente cadastrado com sucesso!".to_string(),
    }))
}

// Endpoint para retornar todos os pacientes para o menu suspenso (select)
pub async fn list_patients_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Patient>>, (StatusCode, String)> {
    let patients = sqlx::query_as!(
        Patient,
        r#"
        SELECT id, full_name, medical_record_id, birth_date, notes, created_at
        FROM patients
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao buscar lista de pacientes: {}", e)))?;

    Ok(Json(patients))
}
