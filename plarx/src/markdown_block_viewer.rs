use pulldown_cmark::{HeadingLevel, Parser, Event, Tag, TagEnd};
use egui::{RichText, Ui};

use crate::markdown_viewer;

pub fn view_markdown(ui: &mut Ui, source: &mut String, active_line: &mut Option<usize>){
    let mut line_source: Vec<&str>  = source.split("\n").collect();

    for (i,line) in line_source.iter_mut().enumerate(){
        if *active_line == Some(i){
            let resp = ui.add(egui::TextEdit::multiline(line));
            if resp.lost_focus(){
                *active_line = None
            }
        }
        else
        {
            if line.is_empty(){
                ui.add_space(18.0);
            }
            else{
                markdown_viewer::view_singular_markdown(ui, line, i);
            }
        }
    }
}