pub fn markdown_code_block(lang: &str, content: &str) -> String {
    format!(
        "```{}
{}```",
        lang, content
    )
}
