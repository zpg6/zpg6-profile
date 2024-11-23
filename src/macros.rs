/// Includes markdown files and converts them to HTML.
///
/// Since SimpleCSS is injected into the `head` the final HTML, this just renders
/// nicely formatted markdown without us having to add any inline styles.
///
/// # Example
///
/// ```rust
/// let html_string = include_markdown!("path/to/file.md");
/// ```
#[macro_export]
macro_rules! include_markdown {
    ($path:expr) => {{
        use pulldown_cmark::{html, Parser};
        const CONTENT: &str = include_str!($path);
        {
            let parser = Parser::new(CONTENT);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            html_output
        }
    }};
}
