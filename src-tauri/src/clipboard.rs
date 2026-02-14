use clipboard_win::{get_clipboard_string, set_clipboard_string};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ClipboardMonitor {
    last_content: Arc<Mutex<String>>,
    is_running: Arc<Mutex<bool>>,
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        ClipboardMonitor {
            last_content: Arc::new(Mutex::new(String::new())),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start<F>(&self, callback: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let last_content = Arc::clone(&self.last_content);
        let is_running = Arc::clone(&self.is_running);

        *self.is_running.lock().unwrap() = true;

        thread::spawn(move || {
            while *is_running.lock().unwrap() {
                if let Ok(current) = get_clipboard_string() {
                    let mut last = last_content.lock().unwrap();
                    if current != *last && is_code_like(&current) {
                        callback(current.clone());
                        *last = current;
                    }
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    pub fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
    }

    pub fn get_clipboard(&self) -> Result<String, String> {
        get_clipboard_string().map_err(|e| e.to_string())
    }

    pub fn set_clipboard(&self, content: String) -> Result<(), String> {
        set_clipboard_string(&content).map_err(|e| e.to_string())
    }
}

fn is_code_like(text: &str) -> bool {
    // Check if text looks like code
    let code_indicators = [
        "function", "class", "def ", "const ", "let ", "var ", "fn ", "pub ",
        "{", "}", "[", "]", "(", ")", "=>", "->", "=", ";", ":",
        "import", "export", "return", "if ", "for ", "while ", "match ",
    ];

    let lower = text.to_lowercase();
    code_indicators.iter().any(|indicator| lower.contains(indicator))
        && text.len() > 10
        && text.lines().count() > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_code_like() {
        assert!(is_code_like("function hello() {\n  return 'world';\n}"));
        assert!(is_code_like("const x = 5;\nlet y = 10;"));
        assert!(is_code_like("def hello():\n    return 'world'"));
        assert!(!is_code_like("hello world"));
        assert!(!is_code_like("short"));
    }
}
