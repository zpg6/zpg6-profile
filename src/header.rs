/// Wraps the given HTML in a full HTML document with a header and pre-included CSS.
pub fn wrap_html_with_header(body_html: String) -> axum::http::Response<axum::body::Body> {
    let html = format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
       <meta charset="UTF-8"> 
       <title>Zach Grimaldi</title>
       <style>{}</style>
    </head>
    <body>
        <header>
            <h1>Zach Grimaldi</h1>

            <nav>
                <a href="/">Home</a>
                <a href="/projects">Projects</a>
                <a href="https://github.com/zpg6">GitHub</a>
                <a href="https://www.linkedin.com/in/zpg6/">LinkedIn</a>
            </nav>
        </header>
        <main>
            {}
        </main>
        <footer>
            <p>&copy; <script>document.write(new Date().getFullYear())</script> Zach Grimaldi</p>
        </footer>
    </body>
</html>
"#,
        include_str!("../static/simple.min.css"),
        body_html
    );
    axum::http::Response::builder()
        .header("Content-Type", "text/html")
        .body(axum::body::Body::from(html))
        .unwrap()
}
