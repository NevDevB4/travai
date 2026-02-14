use rusqlite::{Connection, Result as SqliteResult, params};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub code_language: Option<String>,
    pub message_type: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedFile {
    pub id: String,
    pub conversation_id: String,
    pub file_name: String,
    pub file_type: String,
    pub file_path: String,
    pub created_at: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Self {
        let db_path = Self::get_db_path();
        std::fs::create_dir_all(db_path.parent().unwrap()).ok();
        
        let conn = Connection::open(&db_path).expect("Failed to open database");
        let db = Database { conn };
        db.init_tables().expect("Failed to initialize database");
        db
    }

    fn get_db_path() -> PathBuf {
        let app_data = dirs::data_local_dir().expect("Failed to get app data directory");
        app_data.join("Travai").join("travai.db")
    }

    fn init_tables(&self) -> SqliteResult<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                code_language TEXT,
                message_type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            );

            CREATE TABLE IF NOT EXISTS uploaded_files (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                file_name TEXT NOT NULL,
                file_type TEXT NOT NULL,
                file_path TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            );"
        )?;
        Ok(())
    }

    pub fn create_conversation(&self, title: String) -> SqliteResult<Conversation> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO conversations (id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![&id, &title, &now, &now],
        )?;

        Ok(Conversation {
            id,
            title,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_conversations(&self) -> SqliteResult<Vec<Conversation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, created_at, updated_at FROM conversations ORDER BY updated_at DESC"
        )?;

        let conversations = stmt.query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?;

        conversations.collect()
    }

    pub fn add_message(
        &self,
        conversation_id: String,
        role: String,
        content: String,
        code_language: Option<String>,
        message_type: String,
    ) -> SqliteResult<Message> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO messages (id, conversation_id, role, content, code_language, message_type, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![&id, &conversation_id, &role, &content, &code_language, &message_type, &now],
        )?;

        Ok(Message {
            id,
            conversation_id,
            role,
            content,
            code_language,
            message_type,
            created_at: now,
        })
    }

    pub fn get_messages(&self, conversation_id: String) -> SqliteResult<Vec<Message>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, conversation_id, role, content, code_language, message_type, created_at
             FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC"
        )?;

        let messages = stmt.query_map(params![&conversation_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                code_language: row.get(4)?,
                message_type: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;

        messages.collect()
    }

    pub fn save_uploaded_file(
        &self,
        conversation_id: String,
        file_name: String,
        file_type: String,
        file_path: String,
    ) -> SqliteResult<UploadedFile> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO uploaded_files (id, conversation_id, file_name, file_type, file_path, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&id, &conversation_id, &file_name, &file_type, &file_path, &now],
        )?;

        Ok(UploadedFile {
            id,
            conversation_id,
            file_name,
            file_type,
            file_path,
            created_at: now,
        })
    }

    pub fn get_uploaded_files(&self, conversation_id: String) -> SqliteResult<Vec<UploadedFile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, conversation_id, file_name, file_type, file_path, created_at
             FROM uploaded_files WHERE conversation_id = ?1 ORDER BY created_at DESC"
        )?;

        let files = stmt.query_map(params![&conversation_id], |row| {
            Ok(UploadedFile {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                file_name: row.get(2)?,
                file_type: row.get(3)?,
                file_path: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;

        files.collect()
    }
}
