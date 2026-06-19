mod document;

mod markdown_viewer;

use document::{Document, Block};

use markdown_viewer::view_markdown;

struct PlarxApp{
    doc: Document,
}

impl Default for PlarxApp {
    fn default() -> Self {
        let mut doc = Document::new("Untitled Document");
        doc.blocks.push(Block::new_markdown("# Hello\n**This** *is* ***a test block.***"));
        doc.blocks.push(Block::new_drawing(560.0, 240.0));

        Self { doc }
    }
}

impl eframe::App for PlarxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.doc.title);
            ui.separator();

            for block in &self.doc.blocks {
                match block{
                    Block::Markdown(text) => view_markdown(ui, text),
                    _ => {
                        ui.label("Drawing block (not implemented yet)");
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