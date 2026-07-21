use serde::{Deserialize, Serialize};
use worker::{Ai, Env, Result};

const EMBED_BATCH: usize = 20;
const SUMMARY_PROMPT: &str = "You distill AI coding session transcripts into dense memory notes. \
Produce a digest with: what was worked on, key decisions and why, facts and constraints learned, \
problems solved, final outcomes, and open TODOs. Write the digest in the dominant language of the \
transcript — mirror it exactly (English stays English, Bahasa Indonesia stays Bahasa Indonesia). \
Use terse bullet points, at most 300 words, no preamble and no headings other than the bullets.";

const REFLECT_PROMPT: &str = "You extract durable, transferable LESSONS from a summary of an AI \
coding session so that future sessions perform better. A lesson is a REUSABLE insight — a pitfall \
to avoid, a fix that worked, a user preference, a project convention, or an effective workflow — \
not a recap of what happened. Return ONLY a compact JSON array (no prose, no markdown fences) of at \
most 5 objects, each {\"category\": one of \"pitfall\",\"fix\",\"preference\",\"convention\",\
\"workflow\",\"insight\"; \"title\": a short imperative under 80 chars; \"insight\": one or two \
sentences, self-contained and actionable}. Prefer specific, transferable lessons over generic \
advice. If the session has no durable lesson, return []. Write titles and insights in the digest's \
dominant language.";

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

#[derive(Deserialize)]
struct ReflectOutput {
    response: Option<serde_json::Value>,
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

    pub async fn reflect(&self, project: &str, summary: &str) -> Result<String> {
        let user = format!("Project: {project}\n\nSession digest:\n{summary}");
        let input = ChatInput {
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: REFLECT_PROMPT,
                },
                ChatMessage {
                    role: "user",
                    content: &user,
                },
            ],
            max_tokens: 700,
        };
        let output: ReflectOutput = self.ai.run(&self.summary_model, input).await?;
        Ok(match output.response {
            Some(serde_json::Value::String(text)) => text,
            Some(other) => other.to_string(),
            None => String::new(),
        })
    }
}
