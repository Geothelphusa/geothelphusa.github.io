// use std::fs;
// use regex::Regex;

// fn main() {
//     // dist フォルダから `.js` ファイルを探す
//     let js_file = fs::read_dir("dist")
//         .unwrap()
//         .filter_map(|entry| entry.ok())
//         .find(|entry| entry.file_name().to_string_lossy().ends_with(".js"))
//         .expect("JavaScript file not found in dist directory");

//     // JavaScript のファイル名
//     let js_filename = js_file.file_name().to_string_lossy().into_owned();

//     // `index.html` を読み込む
//     let html_file = "dist/index.html";
//     let content = fs::read_to_string(html_file).expect("Could not read `index.html`");

//     // `<script src="古いJSファイル名"></script>` を更新
//     let re = Regex::new(r#"<script src=".*\.js"></script>"#).unwrap();
//     let new_script_tag = format!(r#"<script src="{}"></script>"#, js_filename);
//     let updated_content = re.replace_all(&content, new_script_tag);

//     // `index.html` を更新
//     fs::write(html_file, updated_content.as_bytes()).expect("Could not write to `index.html`");
// }

use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dist_dir = Path::new("dist");

    // 1️⃣ Trunk によって生成された JS ファイル名を取得
    let old_js_name = fs::read_dir(dist_dir)?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.file_name().to_string_lossy().ends_with(".js"))
        .map(|entry| entry.file_name().to_string_lossy().to_string());

    if let Some(old_js_name) = old_js_name {
        // let old_js_path = dist_dir.join(&old_js_name);
        // let new_js_path = dist_dir.join("app.js");

        // 2️⃣ JS ファイルを `app.js` にリネーム
        // fs::rename(&old_js_path, &new_js_path)?;

        // 3️⃣ `index.html` の修正
        let index_path = dist_dir.join("index.html");
        let mut content = fs::read_to_string(&index_path)?;

        // 変更 1: import 文の修正
        content = content.replace(
            &format!(r#"import init, * as bindings from '/geothelphusa.github.io/{}';"#, old_js_name),
            r#"import init, * as bindings from '/app.js';"#,
        );

        // 変更 2: `<script>` タグの `type="module"` 追加
        content = content.replace(
            r#"<script src="app.js"></script>"#,
            r#"<script type="module" src="app.js"></script>"#,
        );

        // 変更 3: `style.css` のパス修正
        content = content.replace(
            r#"<link rel="stylesheet" href="style.css">"#,
            r#"<link rel="stylesheet" href="/geothelphusa.github.io/style.css">"#,
        );

        // 修正した内容を `index.html` に書き戻す
        fs::write(&index_path, content)?;
    }

    Ok(())
}

