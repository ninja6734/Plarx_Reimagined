mod document;

use document::{Document, Block};

fn main() {
    let mut doc = Document::new("Test Document");
    doc.blocks.push(Block::new_markdown("Hello, World!"));
    doc.blocks.push(Block::new_drawing(560.0, 420.0));

    println!("Before Saving: {:#?}", doc);
    doc.saveToFile("test_document.json").expect("Failed to save document");
    println!("Document saved to test_document.json");

    let loaded = Document::loadFromFile("test_document.json").expect("Failed to load document");
    println!("After Loading: {:#?}", loaded);
}