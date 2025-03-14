use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use regex::Regex;
use log::debug;

pub fn update_integrity_attributes() -> Result<(), Box<dyn std::error::Error>> {
    // JavaScript ファイルを探す
    let js_file = fs::read_dir("dist")?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.file_name().to_string_lossy().ends_with(".js"))
        .ok_or("JavaScript file not found")?;

    // WASM ファイルを探す
    let wasm_file = fs::read_dir("dist")?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.file_name().to_string_lossy().ends_with("_bg.wasm"))
        .ok_or("WASM file not found")?;

    // ハッシュを計算する関数
    fn compute_sha384_hash(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("openssl")
            .args(&["dgst", "-sha384", "-binary", file_path])
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;        
        
        let mut hash_base64 = Command::new("base64")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = hash_base64.stdin.as_mut().ok_or("Failed to open stdin for base64")?;
        stdin.write_all(&output.stdout)?;
        // drop(stdin); // No need to explicitly drop stdin here
        // stdin is dropped when hash_base64 goes out of scope

        let base64_output = hash_base64.wait_with_output()?;
        let hash_str = String::from_utf8(base64_output.stdout)?.trim().to_string();

        Ok(hash_str)
    }

    // JS, WASM の SRI ハッシュを取得
    let js_hash = compute_sha384_hash(js_file.path().to_str().unwrap())?;
    let wasm_hash = compute_sha384_hash(wasm_file.path().to_str().unwrap())?;

    // `index.html` を読み込む
    let html_file = "dist/index.html";
    let content = fs::read_to_string(html_file)?;

    // `<script>` タグの integrity 属性を更新
    let js_re = Regex::new(r#"(<script .*?src=".*?\.js".*?integrity=")sha384-.*?(")"#)?;
    let new_content = js_re.replace_all(&content, format!(r#"$1sha384-{}$2"#, js_hash));

    // `<link>` タグの integrity 属性を更新
    let wasm_re = Regex::new(r#"(<link .*?href=".*?_bg\.wasm".*?integrity=")sha384-.*?(")"#)?;
    let final_content = wasm_re.replace_all(&new_content, format!(r#"$1sha384-{}$2"#, wasm_hash));

    // `index.html` を更新
    fs::write(html_file, final_content.as_bytes())?;
    debug!("Hashes updated successfully!");

    Ok(())
}
