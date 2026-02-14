use crate::db::Database;
use crate::llm::{LLMClient, LLMConfig};
use serde_json::json;
use std::sync::Mutex;
use clipboard_win;

#[tauri::command]
pub async fn check_lm_studio_connection() -> Result<bool, String> {
    let config = LLMConfig::default();
    let client = LLMClient::new(config);
    client.check_connection().await
}

#[tauri::command]
pub async fn send_message(
    state: tauri::State<'_, Mutex<Database>>,
    conversation_id: String,
    message: String,
) -> Result<serde_json::Value, String> {
    let db = state.lock().unwrap();
    
    // Save user message
    let user_msg = db.add_message(
        conversation_id.clone(),
        "user".to_string(),
        message.clone(),
        None,
        "text".to_string(),
    ).map_err(|e| e.to_string())?;

    // Get response from LM Studio
    let config = LLMConfig::default();
    let llm_client = LLMClient::new(config);

    let response = llm_client.chat(vec![
        crate::llm::ChatMessage {
            role: "user".to_string(),
            content: message,
        },
    ]).await?;

    // Save assistant message
    let assistant_msg = db.add_message(
        conversation_id,
        "assistant".to_string(),
        response,
        None,
        "text".to_string(),
    ).map_err(|e| e.to_string())?;

    Ok(json!({
        "user_message": user_msg,
        "assistant_message": assistant_msg,
    }))
}

#[tauri::command]
pub async fn generate_code(
    state: tauri::State<'_, Mutex<Database>>,
    conversation_id: String,
    description: String,
    language: String,
) -> Result<serde_json::Value, String> {
    let db = state.lock().unwrap();

    let user_msg = db.add_message(
        conversation_id.clone(),
        "user".to_string(),
        format!("Generate {} code: {}", language, description),
        Some(language.clone()),
        "code".to_string(),
    ).map_err(|e| e.to_string())?;

    let config = LLMConfig::default();
    let llm_client = LLMClient::new(config);

    let code = llm_client.generate_code(&description, &language).await?;

    let assistant_msg = db.add_message(
        conversation_id,
        "assistant".to_string(),
        code,
        Some(language),
        "code".to_string(),
    ).map_err(|e| e.to_string())?;

    Ok(json!({
        "user_message": user_msg,
        "assistant_message": assistant_msg,
    }))
}

#[tauri::command]
pub async fn explain_code(
    state: tauri::State<'_, Mutex<Database>>,
    conversation_id: String,
    code: String,
    language: String,
) -> Result<serde_json::Value, String> {
    let db = state.lock().unwrap();

    let user_msg = db.add_message(
        conversation_id.clone(),
        "user".to_string(),
        format!("Explain this {} code:\n\n```{}\n{}\n```", language, language, code),
        Some(language.clone()),
        "explanation".to_string(),
    ).map_err(|e| e.to_string())?;

    let config = LLMConfig::default();
    let llm_client = LLMClient::new(config);

    let explanation = llm_client.explain_code(&code, &language).await?;

    let assistant_msg = db.add_message(
        conversation_id,
        "assistant".to_string(),
        explanation,
        Some(language),
        "explanation".to_string(),
    ).map_err(|e| e.to_string())?;

    Ok(json!({
        "user_message": user_msg,
        "assistant_message": assistant_msg,
    }))
}

#[tauri::command]
pub async fn debug_code(
    state: tauri::State<'_, Mutex<Database>>,
    conversation_id: String,
    code: String,
    language: String,
) -> Result<serde_json::Value, String> {
    let db = state.lock().unwrap();

    let user_msg = db.add_message(
        conversation_id.clone(),
        "user".to_string(),
        format!("Debug this {} code:\n\n```{}\n{}\n```", language, language, code),
        Some(language.clone()),
        "debug".to_string(),
    ).map_err(|e| e.to_string())?;

    let config = LLMConfig::default();
    let llm_client = LLMClient::new(config);

    let debug_result = llm_client.debug_code(&code, &language).await?;

    let assistant_msg = db.add_message(
        conversation_id,
        "assistant".to_string(),
        debug_result,
        Some(language),
        "debug".to_string(),
    ).map_err(|e| e.to_string())?;

    Ok(json!({
        "user_message": user_msg,
        "assistant_message": assistant_msg,
    }))
}

#[tauri::command]
pub async fn optimize_code(
    state: tauri::State<'_, Mutex<Database>>,
    conversation_id: String,
    code: String,
    language: String,
) -> Result<serde_json::Value, String> {
    let db = state.lock().unwrap();

    let user_msg = db.add_message(
        conversation_id.clone(),
        "user".to_string(),
        format!("Optimize this {} code:\n\n```{}\n{}\n```", language, language, code),
        Some(language.clone()),
        "optimization".to_string(),
    ).map_err(|e| e.to_string())?;

    let config = LLMConfig::default();
    let llm_client = LLMClient::new(config);

    let optimized = llm_client.optimize_code(&code, &language).await?;

    let assistant_msg = db.add_message(
        conversation_id,
        "assistant".to_string(),
        optimized,
        Some(language),
        "optimization".to_string(),
    ).map_err(|e| e.to_string())?;

    Ok(json!({
        "user_message": user_msg,
        "assistant_message": assistant_msg,
    }))
}

#[tauri::command]
pub async fn upload_and_analyze_file(
    state: tauri::State<'_, Mutex<Database>>,
    conversation_id: String,
    file_name: String,
    file_type: String,
    file_content: String,
    question: Option<String>,
) -> Result<serde_json::Value, String> {
    let db = state.lock().unwrap();

    let file_path = format!("file://{}", file_name);
    let uploaded_file = db.save_uploaded_file(
        conversation_id.clone(),
        file_name.clone(),
        file_type.clone(),
        file_path,
    ).map_err(|e| e.to_string())?;

    let user_msg = db.add_message(
        conversation_id.clone(),
        "user".to_string(),
        format!("Uploaded file: {}\n\n{}", file_name, question.as_deref().unwrap_or("Please analyze this code file.")),
        Some(file_type.clone()),
        "file-analysis".to_string(),
    ).map_err(|e| e.to_string())?;

    let config = LLMConfig::default();
    let llm_client = LLMClient::new(config);

    let analysis = llm_client.analyze_file(&file_content, &file_type, question.as_deref()).await?;

    let assistant_msg = db.add_message(
        conversation_id,
        "assistant".to_string(),
        analysis,
        Some(file_type),
        "file-analysis".to_string(),
    ).map_err(|e| e.to_string())?;

    Ok(json!({
        "uploaded_file": uploaded_file,
        "user_message": user_msg,
        "assistant_message": assistant_msg,
    }))
}

#[tauri::command]
pub fn get_conversations(state: tauri::State<'_, Mutex<Database>>) -> Result<Vec<serde_json::Value>, String> {
    let db = state.lock().unwrap();
    let conversations = db.get_conversations().map_err(|e| e.to_string())?;
    
    Ok(conversations.iter().map(|c| json!(c)).collect())
}

#[tauri::command]
pub fn create_conversation(
    state: tauri::State<'_, Mutex<Database>>,
    title: String,
) -> Result<serde_json::Value, String> {
    let db = state.lock().unwrap();
    let conversation = db.create_conversation(title).map_err(|e| e.to_string())?;
    
    Ok(json!(conversation))
}

#[tauri::command]
pub fn get_messages(
    state: tauri::State<'_, Mutex<Database>>,
    conversation_id: String,
) -> Result<Vec<serde_json::Value>, String> {
    let db = state.lock().unwrap();
    let messages = db.get_messages(conversation_id).map_err(|e| e.to_string())?;
    
    Ok(messages.iter().map(|m| json!(m)).collect())
}

#[tauri::command]
pub fn get_clipboard() -> Result<String, String> {
    clipboard_win::get_clipboard_string().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_clipboard(content: String) -> Result<(), String> {
    clipboard_win::set_clipboard_string(&content).map_err(|e| e.to_string())
}
