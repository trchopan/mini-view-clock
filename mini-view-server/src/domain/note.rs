use org::Org;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub content: String,
}

fn org_to_html(org: &Vec<Org>) -> Vec<String> {
    if org.is_empty() {
        vec![]
    } else {
        org.iter()
            .map(|sub| {
                let heading = format!(
                    "<h{depth} class=\"org-h{depth}\">{content}</h{depth}>",
                    depth = sub.depth() + 1,
                    content = sub.heading()
                );
                let content: Vec<String> = sub
                    .content_as_ref()
                    .iter()
                    .filter(|c| !c.is_empty())
                    .map(|c| {
                        format!(
                            "<p class=\"org-p{depth}\">{content}</p>",
                            depth = sub.depth() + 1,
                            content = c
                        )
                    })
                    .collect();
                let content = content.join("");
                [vec![heading, content], org_to_html(sub.subtrees_as_ref())].concat()
            })
            .flatten()
            .collect()
    }
}

impl Note {
    pub fn from_org_to_html(content: String) -> Self {
        let contents: Vec<String> = content.split('\n').map(|s| s.to_string()).collect();
        let content = match Org::from_vec(&contents) {
            Ok(org) => {
                let title_str = "#+title:";
                let title = org
                    .content_as_ref()
                    .iter()
                    .find(|&c| c.to_lowercase().starts_with(title_str))
                    .map(|title| {
                        let t = title.replace(title_str, "").trim().to_string();
                        format!("<h1 class=\"org-h1\">{}</h1>", t)
                    });
                let title = title.unwrap_or("".to_string());

                let content = org_to_html(org.subtrees_as_ref());

                title + &content.join("")
            }
            Err(_) => todo!(),
        };
        Self { content }
    }
}
