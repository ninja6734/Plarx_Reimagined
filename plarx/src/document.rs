
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StrokePoint {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stroke {
    pub points: Vec<StrokePoint>,
    pub color: (u8, u8, u8),
    pub width: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Block {
    Markdown(String),
    Drawing {
        width: f32,
        height: f32,
        strokes: Vec<Stroke>,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Document {
    pub title: String,
    pub blocks: Vec<Block>,
}

impl Block {
    pub fn new_markdown(text: &str) -> Self {
        Block::Markdown(text.to_string())
    }

    pub fn new_drawing(width: f32, height: f32) -> Self {
        Block::Drawing { width, height, strokes: Vec::new() }
    }
}

impl Document {
    pub fn new(title: &str) -> Self {
        Document { title: title.to_string(), blocks: Vec::new() }
    }

    pub fn saveToFile(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, json)
    }

    pub fn loadFromFile(path: &str) -> std::io::Result<Document> {
        let json = std::fs::read_to_string(path)?;
        let doc: Document = serde_json::from_str(&json).unwrap_or_default();
        Ok(doc)
    }
}