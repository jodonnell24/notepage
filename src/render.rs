use pulldown_cmark::{Options, Parser, html};

// Convert markdown text to HTML
pub fn markdown_to_html(md: &str) -> String {
    
    // Create empty options (no special features enabled by default)
    let mut opts = Options::empty();

    // Enable table support so | Header | works in markdown
    opts.insert(Options::ENABLE_TABLES);

    // Create a parser that will process the markdown text
    let parser = Parser::new_ext(md, opts);

    // Create an empty String to build our HTML output
    let mut html_buf = String::new();

    // Convert the parser events into HTML and append to our buffer
    html::push_html(&mut html_buf, parser);

    html_buf
}