use clap::error::Result;
use tree_sitter::{Parser};
use walkdir::WalkDir;

pub fn analyze(path: &str) -> Vec<String> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_swift::LANGUAGE.into())
        .expect("Failed to set parser language to Swift");
    
    let mut symbols = vec![];

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "swift").unwrap_or(false)) {
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            if let Some(tree) = parser.parse(&content, None) {
                let root = tree.root_node();
                collect_symbols(&content, root, &mut symbols);
            }
        }
    }

    symbols
}

fn collect_symbols(src: &str, node: tree_sitter::Node, output: &mut Vec<String>) {
    if node.kind() == "identifier" {
        let txt = &src[node.byte_range()];
        output.push(txt.to_string());
    }
    
    for child in node.children(&mut node.walk()) {
        collect_symbols(src, child, output);
    }
}