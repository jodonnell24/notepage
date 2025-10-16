use anyhow::Context;
use chrono::NaiveDate;
use serde::Deserialize;

// Define our Post struct - represents a single note
#[derive(Debug)]
pub struct Post {
    // URL-friendly version of the title
    pub slug: String,

    // Publication date
    pub date: NaiveDate,

    // Post title, like "My Notes"
    pub title: String,

    // Short description/summary of the post
    pub description: String,

    // The raw markdown content (everything after the front matter)
    pub raw_markdown: String,
}

// Implementation block for Post - methods for working with posts
impl Post {
    // Parse a markdown file into a Post struct

    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        // Read the entire file into a String
        let txt = std::fs::read_to_string(path)?;

        // Split the file into front matter and markdown content
        let (front, md) = txt
            .split_once("+++")
            .and_then(|(_, rest)| rest.split_once("+++"))
            .context("missing front-matter delimiters")?; 

        // Parse the front matter TOML into our FrontMatter struct
        let meta: FrontMatter = toml::from_str(front)?;

        // Extract the slug from the filename
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("bad filename")?
            .trim_start_matches(char::is_numeric)
            .trim_start_matches('-')
            .to_string();

        // Create and return the Post struct
        Ok(Post {
            // Use the slug we just extracted
            slug,

            // Parse the date from filename: "2025-10-15-my-note.md" -> 2025-10-15 (YYYY-MM-DD)
            date: NaiveDate::parse_from_str(
                &path.file_stem().and_then(|s| s.to_str()).unwrap()[..10],
                "%Y-%m-%d", 
            )?,

            // Copy title and description from front matter
            title: meta.title,
            description: meta.description,

            // Get the markdown content, trim whitespace
            raw_markdown: md.trim().to_string(),
        })
    }
}

// FrontMatter struct - represents the TOML metadata at top of markdown files
#[derive(Debug, Deserialize)]

struct FrontMatter { // Post title from front matter, like title = "My Post"
    title: String,

    // Post description from front matter, like description = "A great post"
    description: String,
}