use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::File;
use std::io::Write;

fn main() {
  let html = html! {
    : doctype::HTML;
    html(lang="ja", data-theme="dim") {
        head {
            meta(name="viewport", content="width=device-width, initial-scale=1.0");
            title : "Geothelphusa.github.io";
            // link(data-trunk rel="rust" href="/Cargo.toml");
        }
        body(class="flex flex-col min-h-screen") {
          div(id="app")
        }
    }
  };

  let mut index_html = File::create("index.html").expect("Could not create `index.html`.");
  writeln!(index_html, "{html}").expect("Could not write to `index.html`.")
}