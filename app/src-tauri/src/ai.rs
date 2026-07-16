//! Program önerisi: kullanıcının günün akışına (Rust tarafında hazırlanmış
//! kısa bir özet) bakıp kısa, somut bir öneri döndürmesi için Anthropic
//! Messages API'sini çağırır. API anahtarı kullanıcı tarafından Settings'e
//! girilir ve yalnızca yerel SQLite'ta durur — hiçbir yere loglanmaz.

use serde::{Deserialize, Serialize};

const MODEL: &str = "claude-sonnet-5";

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct Request<'a> {
    model: &'a str,
    max_tokens: u32,
    system: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(default)]
    text: String,
}

#[derive(Deserialize)]
struct Response {
    content: Vec<ContentBlock>,
}

const SYSTEM_PROMPT: &str = "Sen kullanıcının kişisel gün programına bakan sakin bir asistansın. \
Sana günün saatli programı ve açık görevleri verilecek. Sadece somut, kısa öneriler ver: \
boş zaman aralıklarını nasıl değerlendirebileceğini, çakışan veya çok sıkışık bir program olup \
olmadığını söyle. En fazla 4-5 madde, süslemesiz, Türkçe. Şiirsel konuşma, sadece pratik öneri.";

pub async fn suggest(api_key: &str, day_summary: &str) -> Result<String, String> {
    let body = Request {
        model: MODEL,
        max_tokens: 500,
        system: SYSTEM_PROMPT,
        messages: vec![Message {
            role: "user",
            content: day_summary,
        }],
    };

    let resp = reqwest::Client::new()
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("AI isteği başarısız: {e}"))?
        .error_for_status()
        .map_err(|e| format!("AI isteği reddedildi: {e}"))?
        .json::<Response>()
        .await
        .map_err(|e| format!("AI yanıtı ayrıştırılamadı: {e}"))?;

    Ok(resp
        .content
        .into_iter()
        .map(|b| b.text)
        .collect::<Vec<_>>()
        .join(""))
}
