mod document;

mod markdown_block_viewer;
mod markdown_viewer;

mod drawing_view;

use document::{Document, Block, Stroke};

use markdown_block_viewer::view_markdown;

use drawing_view::view_drawing;

struct PlarxApp{
    doc: Document,
    active_stroke: Option<Stroke>,
    active_line: Option<usize>,
}

impl Default for PlarxApp {
    fn default() -> Self {
        let mut doc: Document = Document::new("Untitled Document");
        doc.blocks.push(Block::new_markdown("# Hello\n**This** *is* ***a test block.***\nThis should still work in a new line"));
        doc.blocks.push(Block::new_drawing(560.0, 240.0));

        Self { doc, active_stroke: None, active_line: None }
    }
}

impl eframe::App for PlarxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.doc.title);
            ui.separator();

            for block in &mut self.doc.blocks {
                match block{
                    Block::Markdown(text) => view_markdown(ui, text, &mut self.active_line),
                    Block::Drawing { width, height, strokes } => view_drawing(ui, *width, *height, strokes, &mut self.active_stroke),
                    _ => {
                        ui.label("Block not implemented yet! Sorry for the inconvenience");
                    }
                }
                ui.add_space(8.0);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Plarx",
        options,
        Box::new(|_cc| Box::new(PlarxApp::default())),
    )
}