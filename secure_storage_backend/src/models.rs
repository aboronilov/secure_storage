use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
#[sqlx(type_name = "user")]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub public_key: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
#[sqlx(type_name = "file")]
pub struct File {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub file_name: String,
    pub file_size: i64,
    pub encrypted_aes_key: Vec<u8>,
    pub encrypted_file: Vec<u8>,
    pub iv: Vec<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
#[sqlx(type_name = "shared_link")]
pub struct SharedLink {
    pub id: uuid::Uuid,
    pub file_id: Option<uuid::Uuid>,
    pub recipient_user_id: uuid::Uuid,
    pub password: String,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
pub struct SentFileDetails {
    pub file_id: uuid::Uuid,
    pub file_name: String,
    pub file_size: i64,
    pub recipient_email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
pub struct ReceiveFileDetails {
    pub file_id: uuid::Uuid,
    pub file_name: String,
    pub file_size: i64,
    pub sender_email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

