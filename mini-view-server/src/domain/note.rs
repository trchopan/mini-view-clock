use lazy_static::lazy_static;
use org::Org;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub content: String,
}

impl Note {
    fn clean_id_str(content: &String) -> String {
        lazy_static! {
            static ref ORG_LINK_REGEX: Regex =
                Regex::new(r"\[\[(?P<link>.*?)\]\[(?P<text>.*?)\]\]").unwrap();
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
                        "<h{depth} id=\"org-h{content}\" class=\"org-h{depth}\">{content}</h{depth}>",
                        depth = sub.depth() + 1,
                        content = sub.heading(),
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
                        .map(|s| Note::clean_id_str(s))
                        .collect();
                    [output, Note::org_to_html(sub.subtrees_as_ref())].concat()
                })
                .flatten()
                .collect()
        }
    }

    pub fn from_org_to_html(org_str: String) -> Option<Self> {
        let contents: Vec<String> = org_str.split('\n').map(|s| s.to_string()).collect();
        let org = Org::from_vec(&contents).map_or(None, |v| Some(v))?;
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
        let content = Note::org_to_html(org.subtrees_as_ref());

        Some(Self {
            content: title + &content.join(""),
        })
    }

    pub fn from_headers_to_html(strs: Vec<String>) -> Self {
        let content = strs
            .iter()
            .enumerate()
            .map(|(index, s)| {
                format!(
                    "<h{depth} id=\"org-h{content}\" class=\"org-h{depth}\">{content}</h{depth}>",
                    depth = index + 1,
                    content = s,
                )
            })
            .collect::<Vec<String>>()
            .join("");
        Self { content }
    }
}

#[cfg(test)]
mod tests {
    use super::Note;

    #[test]
    fn can_clean_id_str() {
        let sample = "aaa [[id:123456-78abc][My String]]ccc";
        assert_eq!(
            Note::clean_id_str(&sample.to_string()),
            "aaa <b class=\"org-bold\">My String</b>ccc"
        );

        let sample = "bbb [[https://mini-view.web.app/][mini-view.web.app]] ddd aaa [[id:123456-78abc][My String]]ccc";
        assert_eq!(
            Note::clean_id_str(&sample.to_string()),
            "bbb <b class=\"org-bold\">mini-view.web.app</b> ddd aaa <b class=\"org-bold\">My String</b>ccc"
        );
    }
}
