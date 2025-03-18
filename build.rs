use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::File;
use std::io::Write;
use std::fs;

fn main() {
const FAVICON_PATH: &str = "https://raw.githubusercontent.com/Geothelphusa/geothelphusa.github.io/refs/heads/main/static/Geothelphusa.jpeg";
    let favicon_path = FAVICON_PATH;
    
    let html = html! {
        : doctype::HTML;
        html(lang="ja", data-theme="dim") {
            head {
                meta(charset="utf-8");
                title : "Geothelphusa";
                link(data-trunk, rel="rust", href="Cargo.toml");
                link(rel="icon", href=favicon_path);
                meta(name="viewport", content="width=device-width, initial-scale=1"); // viewportメタタグを追加
                style {
                    : "body { margin: 0; padding: 0; } "; // Chromeの余白を消すためのスタイル
                }
            }
            body { }
        }
    };

    // Create index.html in the root directory
    let mut index_html = File::create("index.html").expect("Could not create `index.html`.");
    writeln!(index_html, "{html}").expect("Could not write to `index.html`.");

    // Create dist directory if it doesn't exist
    fs::create_dir_all("dist").expect("Could not create `dist` directory.");

    // Create index.html in the dist directory
    let mut dist_index_html = File::create("dist/index.html").expect("Could not create `dist/index.html`.");
    writeln!(dist_index_html, "{html}").expect("Could not write to `dist/index.html`.");
    
}

// This build main function is generating HTML templates.
// trunk command use HTML in root directory.
