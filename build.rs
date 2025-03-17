use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::File;
use std::io::Write;
use std::fs;
use std::env;
use std::path::Path;

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
    
    println!("cargo:rerun-if-env-changed=CARGO_WEBHOOK_URL");

    // `CARGO_WEBHOOK_URL` 環境変数を取得
    let webhook_url = env::var("CARGO_WEBHOOK_URL").expect("CARGO_WEBHOOK_URL not set");

    // `WEBHOOK_URL` をコンパイル時に埋め込む
    println!("cargo:rustc-env=WEBHOOK_URL={}", webhook_url);

    // `OUT_DIR` のパスを取得
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("webhook.rs");

    // `webhook.rs` の内容を生成
    let content = format!(r#"pub const WEBHOOK_URL: &str = "{}";"#, webhook_url);

    // `webhook.rs` を作成
    fs::write(dest_path, content).expect("Failed to write webhook.rs");
}

// This build main function is generating HTML templates.
// trunk command use HTML in root directory.
