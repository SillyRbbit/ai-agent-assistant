//! Local wikilinks only. No filesystem or execution authority.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Link {
    pub title: String,
    pub label: String,
    pub context: String,
    pub target_id: Option<i64>,
    pub status: String,
}
// Conservative Markdown code exclusion: fenced blocks (including nested/indented
// fences) and matching backtick spans are not link sources. Limits inherit 16 KiB.
pub(crate) fn parse(text: &str) -> Vec<Link> {
    let mut result = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    let mut ticks = 0;
    for line in text.lines() {
        let s = line.trim_start_matches([' ', '\t', '>']);
        let c = s.chars().next().unwrap_or(' ');
        let count = s.chars().take_while(|x| *x == c).count();
        if matches!(c, '`' | '~') && count >= 3 {
            match fence {
                None => fence = Some((c, count)),
                Some((f, n)) if f == c && count >= n => fence = None,
                _ => {}
            }
            continue;
        }
        if fence.is_some() || line.starts_with("    ") || line.starts_with('\t') {
            continue;
        }
        let mut tail = line;
        while !tail.is_empty() {
            if tail.starts_with('\\') {
                tail = &tail[1..];
                if let Some(c) = tail.chars().next() {
                    tail = &tail[c.len_utf8()..];
                }
                continue;
            }
            if tail.starts_with('`') {
                let n = tail.bytes().take_while(|c| *c == b'`').count();
                if ticks == 0 {
                    ticks = n;
                } else if ticks == n {
                    ticks = 0;
                }
                tail = &tail[n..];
                continue;
            }
            if ticks == 0 && tail.starts_with("[[") {
                if let Some(end) = tail[2..].find("]]") {
                    let value = &tail[2..end + 2];
                    let (title, label) = value.split_once('|').unwrap_or((value, value));
                    let title = title.trim();
                    if !title.is_empty()
                        && title.chars().count() <= 120
                        && !title.contains(['[', ']'])
                    {
                        result.push(Link {
                            title: title.into(),
                            label: label.trim().into(),
                            context: line.chars().take(220).collect(),
                            target_id: None,
                            status: "missing".into(),
                        });
                    }
                    tail = &tail[end + 4..];
                    continue;
                }
            }
            if let Some(c) = tail.chars().next() {
                tail = &tail[c.len_utf8()..];
            }
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn excludes_code_and_escapes_and_keeps_unicode_aliases() {
        let text = "[[Runbook|Read it]] [[界]] `[[inline]]`\n```md\n[[fenced]]\n```\n~~~\n[[tilde]]\n~~~\n    [[indented]]\n\\[[escaped]]";
        let links = parse(text);
        assert_eq!(
            links.iter().map(|l| l.title.as_str()).collect::<Vec<_>>(),
            vec!["Runbook", "界"]
        );
        assert_eq!(links[0].label, "Read it");
    }
}
