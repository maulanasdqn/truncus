use serde::{Deserialize, Serialize};
use worker::{Ai, Env, Result};

const EMBED_BATCH: usize = 20;
const SUMMARY_PROMPT: &str = "You distill AI coding session transcripts into dense memory notes. \
Produce a digest with: what was worked on, key decisions and why, facts and constraints learned, \
problems solved, final outcomes, and open TODOs. Use terse bullet points, at most 300 words, \
no preamble and no headings other than the bullets.";

pub struct AiService {
    ai: Ai,
    embed_model: String,
    summary_model: String,
}

#[derive(Serialize)]
struct EmbeddingInput<'a> {
    text: &'a [String],
}

#[derive(Deserialize)]
struct EmbeddingOutput {
    data: Vec<Vec<f32>>,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct ChatInput<'a> {
    messages: Vec<ChatMessage<'a>>,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct ChatOutput {
    response: Option<String>,
}

impl AiService {
    pub fn new(env: &Env) -> Result<Self> {
        Ok(Self {
            ai: env.ai("AI")?,
            embed_model: env.var("EMBED_MODEL")?.to_string(),
            summary_model: env.var("SUMMARY_MODEL")?.to_string(),
        })
    }

    pub async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut vectors = Vec::with_capacity(texts.len());
        for batch in texts.chunks(EMBED_BATCH) {
            let output: EmbeddingOutput = self
                .ai
                .run(&self.embed_model, EmbeddingInput { text: batch })
                .await?;
            vectors.extend(output.data);
        }
        if vectors.len() != texts.len() {
            return Err(worker::Error::RustError(format!(
                "embedding count mismatch: {} texts, {} vectors",
                texts.len(),
                vectors.len()
            )));
        }
        Ok(vectors)
    }

    pub async fn summarize(&self, conversation: &str) -> Result<String> {
        let input = ChatInput {
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: SUMMARY_PROMPT,
                },
                ChatMessage {
                    role: "user",
                    content: conversation,
                },
            ],
            max_tokens: 640,
        };
        let output: ChatOutput = self.ai.run(&self.summary_model, input).await?;
        Ok(output.response.unwrap_or_default().trim().to_string())
    }
}
