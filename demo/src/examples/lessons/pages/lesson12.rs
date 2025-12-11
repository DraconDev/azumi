// use crate::components::{Image, ResponsiveImage};
use azumi::prelude::*;

/// Lesson 12: Image Optimization
///
/// Demonstrates the `@Image` component for performance.
#[azumi::component]
pub fn lesson12_page() -> impl Component {
    html! {


        <div class={container}>
            <h1>"Lesson 12: Image Optimization"</h1>
            <p>"Azumi automatically optimizes standard <img> tags with lazy loading and async decoding."</p>

            <div class={section}>
                <h2>"1. Basic Optimized Image"</h2>
                <div class={code}>
                    "<img src=\"/static/photo.jpg\" width=\"800\" height=\"600\" alt=\"Photo\" />"
                </div>
                <div class={card}>
                    <div class={label}>"Output HTML"</div>
                    <div class={code_comment}>
                        "&lt;img src=\"...\" loading=\"lazy\" decoding=\"async\" width=\"800\" ...&gt;"
                    </div>

                    // Usage of the component
                    <img
                        src="/static/photo.jpg"
                        width="800"
                        height="600"
                        alt="Photo"
                    />
                </div>
            </div>

            <div class={section}>
                <h2>"2. Eager Loading (Above the Fold)"</h2>
                <p>"For hero images at the top of the page, use eager loading."</p>
                <div class={code}>
                    "<img ... loading=\"eager\" />"
                </div>
                <div class={card}>
                    <img
                        src="https://images.unsplash.com/photo-1682687221038-404670e01d46?w=800&q=80"
                        alt="Hero mountain"
                        width="800"
                        height="600"
                        loading="eager"
                    />
                </div>
            </div>

            <div class={section}>
                <h2>"3. Responsive Images (srcset)"</h2>
                <p>"Automatically serve the right size for the device."</p>
                <div class={code}>
                    "<img srcset=\"...\" sizes=\"...\" />"
                </div>
                <div class={card}>
                    <div class={label}>"Try resizing window"</div>

                    // We use a placeholder service that supports width parameter
                    // In real app, you'd have photo-400.jpg, photo-800.jpg on disk
                    <img
                        src="https://images.unsplash.com/photo-1469474968028-56623f02e42e?w=800"
                        srcset="https://images.unsplash.com/photo-1469474968028-56623f02e42e?w=600 600w,
                                https://images.unsplash.com/photo-1469474968028-56623f02e42e?w=1200 1200w"
                        sizes="(max-width: 600px) 100vw, 800px"
                        alt="Responsive nature"
                        class={responsive_img}
                    />
                </div>
            </div>
        </div>
        <style>
            .container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
            .section { margin-bottom: "3rem"; }
            .grid { display: "grid"; grid-template-columns: "repeat(auto-fit, minmax(300px, 1fr))"; gap: "1rem"; }
            .card {
                border: "1px solid #eee"; padding: "1rem"; border-radius: "8px";
                background: "white";
            }
            .code {
                background: "#1e1e1e"; color: "#d4d4d4";
                padding: "1rem"; border-radius: "6px";
                overflow-x: "auto"; font-family: "monospace"; margin: "1rem 0";
            }
            // img { max-width: "100%"; height: "auto"; border-radius: "4px"; }
            .label {
                display: "inline-block"; background: "#e0e7ff"; color: "#4338ca";
                padding: "0.25rem 0.5rem"; border-radius: "4px"; font-size: "0.8rem";
                margin-bottom: "0.5rem"; font-weight: "bold";
            }
            .code_comment { color: "#666"; font-family: "monospace"; font-size: "0.9rem"; margin-bottom: "1rem"; }
            .responsive_img { width: "100%"; height: "auto"; }
        </style>
    }
}

pub async fn lesson12_handler() -> axum::response::Html<String> {
    let component_html = azumi::render_to_string(&lesson12_page());
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Lesson 12: Image Optimization</title>
    <style>body {{ font-family: system-ui; background: #f3f4f6; margin: 0; }}</style>
</head>
<body>
    {}
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        component_html
    );
    axum::response::Html(html)
}
