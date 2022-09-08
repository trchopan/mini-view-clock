use lazy_static::lazy_static;
use org::Org;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub content: String,
}

fn clean_id_str(content: &String) -> String {
    lazy_static! {
        static ref ORG_LINK_REGEX: Regex =
            Regex::new(r"\[\[(?P<link>.*)\]\[(?P<text>.*)\]\]").unwrap();
    }
    ORG_LINK_REGEX
        .replace_all(content, "<b class=\"org-bold\">$text</b>")
        .to_string()
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
                let output: Vec<String> = vec![heading, content]
                    .iter()
                    .map(|s| clean_id_str(s))
                    .collect();
                [output, org_to_html(sub.subtrees_as_ref())].concat()
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

#[cfg(test)]
mod tests {
    use super::clean_id_str;

    #[test]
    fn can_clean_id_str() {
        let sample = "aaa [[id:123456-78abc][My String]]ccc";
        assert_eq!(
            clean_id_str(&sample.to_string()),
            "aaa <b class=\"org-bold\">My String</b>ccc"
        );

        let sample = "bbb [[https://www.example.com][Example String]] ddd";
        assert_eq!(
            clean_id_str(&sample.to_string()),
            "bbb <b class=\"org-bold\">Example String</b> ddd"
        );
    }
}
