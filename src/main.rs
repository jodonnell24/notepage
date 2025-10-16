use anyhow::Context;
use notepage::{config::Config, fs_utils, post::Post, render, template};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Load our website configuration from config.toml
    let cfg = Config::from_file("config.toml")?;

    // Define where we'll put our generated HTML files
    let out_dir = "public";


    fs_utils::ensure_dir(out_dir)?;

    // Create a Path object pointing to our content directory
    let posts_dir = Path::new("content");

    // Read all entries (files and directories) in the content folder
    let entries = std::fs::read_dir(posts_dir).context("read content/")?;

    // Loop through each entry we found
    for entry in entries {
        let path = entry?.path();

        // Check if this file has .md extension
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        // Parse this markdown file into a Post struct
        let post = Post::from_file(&path)?;

        // Convert the markdown content to HTML
        let body_html = render::markdown_to_html(&post.raw_markdown);

        // Pass the post data, HTML body, and site config to the template
        let full_html = template::wrap(&post, &body_html, &cfg);


        let out_path = Path::new(out_dir).join(format!("{}.html", post.slug));

        // Write the complete HTML to the output file
        std::fs::write(out_path, full_html)?;
    }

    // Print success message showing where files were generated
    println!("✔ Generated site in {}/", out_dir);

    Ok(())
}