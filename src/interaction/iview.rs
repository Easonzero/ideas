use crate::item::Item;
use crossterm::style::Color::*;
use minimad::{OwningTemplateExpander, TextTemplate};
use termimad::*;

static SUMMARY_TEMPLATE: &str = r#"
--------------------
[**${ty}**]: ${status}
--------------------
## Summary
> ${summary}
--------------------
"#;

static DETAIL_TEMPLATE: &str = r#"
## Detail
${detail}
--------------------
"#;

static URL_TEMPLATE: &str = r#"
## Reference
${url}
--------------------
"#;

pub struct View {
    summary: TextTemplate<'static>,
    detail: TextTemplate<'static>,
    url: TextTemplate<'static>,
    skin: MadSkin,
}

impl View {
    pub fn new() -> View {
        let mut skin = MadSkin::default();
        skin.bold.set_fg(DarkRed);
        skin.paragraph.set_fg(Blue);
        skin.italic
            .add_attr(crossterm::style::Attribute::Underlined);
        for header in skin.headers.iter_mut() {
            header.set_fg(Yellow);
        }
        View {
            summary: TextTemplate::from(SUMMARY_TEMPLATE),
            detail: TextTemplate::from(DETAIL_TEMPLATE),
            url: TextTemplate::from(URL_TEMPLATE),
            skin,
        }
    }

    pub fn run(&mut self, item: Item) {
        let (w, _) = terminal_size();
        let mut expander = OwningTemplateExpander::new();
        let mut expander_d = OwningTemplateExpander::new();
        let mut expander_u = OwningTemplateExpander::new();
        expander
            .set("summary", item.summary)
            .set("ty", item.ty.desc())
            .set("status", item.ty.status());
        let summary_str =
            FmtText::from_text(&self.skin, expander.expand(&self.summary), Some(w as usize));
        let detail_str = if let Some(detail) = item.detail {
            let lines: Vec<String> = detail
                .lines()
                .map(|x| {
                    if x.starts_with("> ") {
                        format!("{}  \n", x)
                    } else {
                        format!("> {}  \n", x)
                    }
                })
                .collect();
            let detail: String = lines.iter().flat_map(|s| s.chars()).collect();
            expander_d.set_lines_md("detail", detail);
            FmtText::from_text(
                &self.skin,
                expander_d.expand(&self.detail),
                Some(w as usize),
            )
        } else {
            FmtText::from(&self.skin, "", Some(w as usize))
        };
        let url_str = if let Some(url) = item.url {
            let lines: Vec<String> = url
                .lines()
                .map(|x| {
                    if x.starts_with("* ") {
                        format!("{}  \n", x)
                    } else {
                        format!("* *{}*  \n", x)
                    }
                })
                .collect();
            let url: String = lines.iter().flat_map(|s| s.chars()).collect();
            expander_u.set_lines_md("url", url);
            FmtText::from_text(&self.skin, expander_u.expand(&self.url), Some(w as usize))
        } else {
            FmtText::from(&self.skin, "", Some(w as usize))
        };
        println!("{}{}{}", summary_str, detail_str, url_str);
    }
}
