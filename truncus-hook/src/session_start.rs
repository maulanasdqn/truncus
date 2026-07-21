use serde_json::{json, Value};
use truncus_core::client::ApiClient;
use truncus_core::config::Config;
use truncus_core::dto::ContextBundle;
use truncus_core::project::project_from_cwd;
use truncus_core::timefmt::date;

pub async fn run(payload: &Value) -> anyhow::Result<()> {
    let cwd = payload["cwd"].as_str().unwrap_or(".");
    let project = project_from_cwd(cwd);
    let client = ApiClient::new(&Config::load()?);
    let bundle = client.context(&project).await?;
    if bundle.project_sessions.is_empty()
        && bundle.other_sessions.is_empty()
        && bundle.lessons.is_empty()
        && bundle.note_count == 0
    {
        return Ok(());
    }
    let output = json!({
        "hookSpecificOutput": {
            "hookEventName": "SessionStart",
            "additionalContext": render(&project, &bundle)
        }
    });
    println!("{output}");
    Ok(())
}

fn render(project: &str, bundle: &ContextBundle) -> String {
    let mut text = String::from(
        "Truncus memory — distilled summaries of your past Claude Code sessions:\n",
    );
    if !bundle.project_sessions.is_empty() {
        text.push_str(&format!("\n## Recent sessions in {project}\n"));
        for brief in &bundle.project_sessions {
            text.push_str(&format!(
                "\n### {} (session {})\n{}\n",
                date(brief.ended_at),
                brief.id,
                brief.summary
            ));
        }
    }
    if !bundle.other_sessions.is_empty() {
        text.push_str("\n## Recent sessions in other projects\n");
        for brief in &bundle.other_sessions {
            text.push_str(&format!(
                "\n### {} — {} (session {})\n{}\n",
                date(brief.ended_at),
                brief.project,
                brief.id,
                brief.summary
            ));
        }
    }
    if !bundle.lessons.is_empty() {
        text.push_str(&format!(
            "\n## Lessons learned in {project} — apply these\n"
        ));
        for lesson in &bundle.lessons {
            text.push_str(&format!(
                "\n- **[{}] {}** — {}\n",
                lesson.category, lesson.title, lesson.insight
            ));
        }
    }
    if bundle.note_count > 0 {
        text.push_str(&format!(
            "\n## Knowledge base\nThis project has a knowledge base of {} notes synced from your vault. Use the `knowledge_search` MCP tool to pull relevant notes on demand instead of loading everything.\n",
            bundle.note_count
        ));
    }
    text.push_str(
        "\n## Grounding — say \"I don't know\" when unsure\nAnswer from this memory, the loaded lessons, the knowledge base (knowledge_search), and the actual code. If none of those support a specific claim — a file, API, name, number, or past decision — say \"I don't know\" or \"I'm not sure\" plainly and stop. A wrong confident answer is worse than admitting uncertainty; never invent specifics to fill a gap, and don't dress up a low-confidence recall as fact.\n",
    );
    text.push_str(
        "\nUse the truncus MCP tools (memory_search, recent_sessions, get_session, lessons, knowledge_search) for deeper recall.\n",
    );
    text
}
