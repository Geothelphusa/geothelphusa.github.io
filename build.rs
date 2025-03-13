use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::File;
use std::io::Write;
use std::fs;

fn main() {
    let html = html! {
        : doctype::HTML;
        html(lang="ja", data-theme="dim") {
            head {
                meta(name="viewport", content="width=device-width, initial-scale=1.0");
                title : "Geothelphusa.github.io";
                
                // Do not apply integrity to preload
                link(rel="preload", href="app.js", as="script", crossorigin="anonymous");
                link(rel="preload", href="style.css", as="style", crossorigin="anonymous");

                // Integrity is applied to <script> and <link>
                script(src="app.js", integrity="sha384-XXXX", crossorigin="anonymous") {}
                link(rel="stylesheet", href="style.css", integrity="sha384-YYYY", crossorigin="anonymous");
            }
            body(class="flex flex-col min-h-screen") {
                div(id="app") {}
            }
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