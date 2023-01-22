use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub content: String,
}

impl Note {
    pub fn from_headers_to_html(strs: Vec<(Option<char>, String)>) -> Self {
        let content = strs
            .iter()
            .map(|(emoji, s)| {
                let emoji = match emoji {
                    Some(c) => format!("{c} "),
                    None => "".to_owned(),
                };
                format!(
                    "<h{depth} id=\"org-h{content}\" class=\"org-h{depth}\">\
                        <span class=\"emoji\">{emoji}</span>{content}\
                    </h{depth}>",
                    depth = 2,
                    content = s,
                )
            })
            .collect::<Vec<String>>()
            .join("");
        Self { content }
    }
}
