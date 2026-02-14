use reqwest::Client;
use serde_json::{json, Value};

const LM_STUDIO_API_URL: &str = "http://localhost:1234/v1";

#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub api_url: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for LLMConfig {
    fn default() -> Self {
        LLMConfig {
            api_url: LM_STUDIO_API_URL.to_string(),
            model: "codellama".to_string(),
            temperature: 0.7,
            max_tokens: 2048,
        }
    }
}

pub struct LLMClient {
    client: Client,
    config: LLMConfig,
}

impl LLMClient {
    pub fn new(config: LLMConfig) -> Self {
        LLMClient {
            client: Client::new(),
            config,
        }
    }

    pub async fn check_connection(&self) -> Result<bool, String> {
        match self.client
            .get(&format!("{}/models", self.config.api_url))
            .send()
            .await
        {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Err("Cannot connect to LM Studio. Make sure it's running on localhost:1234".to_string()),
        }
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, String> {
        let payload = json!({
            "model": self.config.model,
            "messages": messages.iter().map(|m| json!({
                "role": m.role,
                "content": m.content
            })).collect::<Vec<_>>(),
            "temperature": self.config.temperature,
            "max_tokens": self.config.max_tokens,
            "stream": false,
        });

        match self.client
            .post(&format!("{}/chat/completions", self.config.api_url))
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => {
                match resp.json::<Value>().await {
                    Ok(data) => {
                        if let Some(content) = data["choices"][0]["message"]["content"].as_str() {
                            Ok(content.to_string())
                        } else {
                            Err("Invalid response format from LM Studio".to_string())
                        }
                    }
                    Err(e) => Err(format!("Failed to parse LM Studio response: {}", e)),
                }
            }
            Err(e) => Err(format!("LM Studio request failed: {}", e)),
        }
    }

    pub async fn generate_code(&self, description: &str, language: &str) -> Result<String, String> {
        let system_prompt = format!(
            "You are an expert {} programmer. Generate clean, well-documented code based on the user's description.",
            language
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Generate {} code for: {}", language, description),
            },
        ];

        self.chat(messages).await
    }

    pub async fn explain_code(&self, code: &str, language: &str) -> Result<String, String> {
        let system_prompt = "You are an expert code analyst. Explain the provided code clearly and concisely.";

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Explain this {} code:\n\n```{}\n{}\n```", language, language, code),
            },
        ];

        self.chat(messages).await
    }

    pub async fn debug_code(&self, code: &str, language: &str) -> Result<String, String> {
        let system_prompt = "You are an expert debugger. Analyze the code for bugs and suggest fixes.";

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Debug this {} code:\n\n```{}\n{}\n```", language, language, code),
            },
        ];

        self.chat(messages).await
    }

    pub async fn optimize_code(&self, code: &str, language: &str) -> Result<String, String> {
        let system_prompt = "You are an expert code optimizer. Suggest improvements for performance, readability, and maintainability.";

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Optimize this {} code:\n\n```{}\n{}\n```", language, language, code),
            },
        ];

        self.chat(messages).await
    }

    pub async fn analyze_file(&self, file_content: &str, file_type: &str, question: Option<&str>) -> Result<String, String> {
        let system_prompt = format!(
            "You are an expert code analyst specializing in {}. Analyze the provided code file thoroughly.",
            file_type
        );

        let user_content = if let Some(q) = question {
            format!("File content:\n\n```{}\n{}\n```\n\nQuestion: {}", file_type, file_content, q)
        } else {
            format!("Please analyze this {} file:\n\n```{}\n{}\n```", file_type, file_type, file_content)
        };

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_content,
            },
        ];

        self.chat(messages).await
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}
