use std::{fs, path::PathBuf};
use axum::response::Html;

pub async fn listfiles(dir: &str, relative_path: &str) -> Html<String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Html("<h1>Failure reading the directory</h1>".to_string()),
    };

    let mut html = String::from(r#"
        <!DOCTYPE html>
        <html lang="de">
        <head>
            <meta charset="UTF-8">
            <title>FileServer</title>
            <link rel="stylesheet" href="/static/style.css">
        </head>
        <body>
            <h1>Dateien:</h1>
            <ul>
    "#);

    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let mut new_relative_path = PathBuf::from(relative_path);
            new_relative_path.push(name);
            let encoded_relative_path = new_relative_path.to_str().unwrap_or(name);

            if path.is_file() {
                let download_link = format!(
                    "<span class=\"download-link\"><a href=\"/staticfiles/{}\" download>⬇️ Download</a></span>",
                    encoded_relative_path
                );
                let play_link = format!(
                    "<span class=\"file-content\"><a href=\"/staticfiles/{}\">▶️ {}</a></span>",
                    encoded_relative_path, name
                );

                html.push_str(&format!(
                    "<li>{} {}</li>",
                    download_link, play_link
                ));
            } else if path.is_dir() {
                html.push_str(&format!(
                    "<li><span class=\"file-content\">📁 <a href=\"/browse/{}/\">{}/</a></span></li>",
                    encoded_relative_path, name
                ));
            }
        }
    }

    html.push_str("</ul></body></html>");
    Html(html)
}
