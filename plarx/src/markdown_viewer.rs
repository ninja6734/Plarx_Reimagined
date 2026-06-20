use pulldown_cmark::{HeadingLevel, Parser, Event, Tag, TagEnd};
use egui::{RichText, Ui};

pub fn view_singular_markdown(ui: &mut Ui, source: &str, lineNum: usize){

    let mut bold = false;
    let mut italic = false;

    let resp: Option<InnerResponse> = None

    let mut current_headings: Option<HeadingLevel> = None;
    let mut current_runs: Vec<RichText> = Vec::new();

    let parser = Parser::new(source);

    for event in parser{
        match event{
            Event::Start(tag) => {
                match tag{
                    Tag::Paragraph => current_runs = Vec::new(),
                    Tag::Heading {level, ..} => current_headings = Some(level),

                    Tag::Strong => bold = true,
                    Tag::Emphasis => italic = true,
                    _ => {}
                }
            },
            Event::Text(text) => {
                let mut rt = RichText::new(text.to_string());
                if bold {
                    rt = rt.strong();
                }
                if italic{
                    rt = rt.italics();
                }
                match current_headings{
                    Some(HeadingLevel::H1) => rt = rt.size(32.0),
                    Some(HeadingLevel::H2) => rt = rt.size(24.0),
                    Some(HeadingLevel::H3) => rt = rt.size(18.0),
                    _ => {}
                }
                current_runs.push(rt);
            }
            Event::End(tag) => {
                match tag{
                    TagEnd::Strong => bold = false,
                    TagEnd::Emphasis => italic = false,

                    TagEnd::Paragraph => {
                        ui.horizontal_wrapped(|ui| {
                            for rt in current_runs.drain(..) {
                                ui.label(rt);
                            }
                        });
                    }
                    TagEnd::Heading(_) => {
                        let resp = ui.scope(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            for rt in current_runs.drain(..) {
                                ui.label(rt);
                            }
                        });})
                        current_headings = None;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}