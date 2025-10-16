use crate::{config::Config, post::Post};

// Wrap post content in a complete HTML webpage template
pub fn wrap(post: &Post, body_html: &str, cfg: &Config) -> String {
    // Pass the post data, HTML body, and site config to the template
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>{title} – {site}</title>
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <style>
    body{{max-width:650px;margin:0 auto;font-family:system-ui,line-height:1.6}}
  </style>
</head>
<body>
  <h1>{title}</h1>
  <p><em>{date}</em> — {desc}</p>
  {content}
</body>
</html>"#,
        title = post.title,      // Post title from front matter
        site = cfg.title,        // Site title from config.toml
        date = post.date,        // Post date from filename
        desc = post.description, // Post description from front matter
        content = body_html,     // HTML content (converted from markdown)
    )
}