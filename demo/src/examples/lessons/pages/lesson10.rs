use azumi::prelude::*;

/// Lesson 10: Client-Side State with `set`
///
/// Learn when to use client-side state vs server state.
#[azumi::component]
pub fn lesson10_page() -> impl Component {
    html! {


        <div class={container}>
            <h1>"Lesson 10: Client-Side State"</h1>
            <p class={explanation}>
                "Azumi is server-first, but sometimes you need pure client-side interactivity for "
                "ephemeral UI state like tabs, accordions, and toggles. "
                "For this, we use the "<span class={code}>"set"</span>" command."
            </p>

            // ==========================================
            // Example 1: Tabs
            // ==========================================
            <div class={card} az-scope="{ \"active_tab\": \"rust\" }">
                <h2 class={title}>"Example 1: Tabs"</h2>
                <p>"State is local to the browser. Refreshing resets it."</p>

                <div class={tabs}>
                    <button
                        class={tab_btn}
                        // Bind class 'active' if active_tab == 'rust'
                        az-bind:class:active="active_tab == 'rust'"
                        // On click, set active_tab locally
                        az-on="click set active_tab = 'rust'"
                    >
                        "Rust"
                    </button>
                    <button
                        class={tab_btn}
                        az-bind:class:active="active_tab == 'python'"
                        az-on="click set active_tab = 'python'"
                    >
                        "Python"
                    </button>
                    <button
                        class={tab_btn}
                        az-bind:class:active="active_tab == 'js'"
                        az-on="click set active_tab = 'js'"
                    >
                        "JavaScript"
                    </button>
                </div>

                <div class={tab_content} az-bind:class:active="active_tab == 'rust'">
                    <h3>"Rust"</h3>
                    <p>"Rust is blazingly fast and memory-efficient with no garbage collector."</p>
                </div>
                <div class={tab_content} az-bind:class:active="active_tab == 'python'">
                    <h3>"Python"</h3>
                    <p>"Python is great for data science, AI, and scripting."</p>
                </div>
                <div class={tab_content} az-bind:class:active="active_tab == 'js'">
                    <h3>"JavaScript"</h3>
                    <p>"JavaScript powers the web... but Azumi helps you write less of it!"</p>
                </div>
            </div>

            // ==========================================
            // Example 2: Accordion
            // ==========================================
            <div class={card} az-scope="{ \"acc1\": false, \"acc2\": false }">
                <h2 class={title}>"Example 2: Accordion"</h2>

                <div class={accordion_item}>
                    <div
                        class={accordion_header}
                        az-on="click set acc1 = !acc1"
                    >
                        "Section 1: Why Azumi?"
                        <span az-bind:text="acc1 ? '−' : '+'">"+"</span>
                    </div>
                    // Show body if acc1 is true
                    <div class={accordion_body} az-bind:class:open="acc1">
                        <p>"Because it brings compile-time safety to your frontend code!"</p>
                    </div>
                </div>

                <div class={accordion_item}>
                    <div
                        class={accordion_header}
                        az-on="click set acc2 = !acc2"
                    >
                        "Section 2: How does it work?"
                        <span az-bind:text="acc2 ? '−' : '+'">"+"</span>
                    </div>
                    <div class={accordion_body} az-bind:class:open="acc2">
                        <p>"It uses Rust macros to analyze your code and generate optimized HTML and minimal JS."</p>
                    </div>
                </div>
            </div>

            <div class={card}>
                <h2 class={title}>"When to use what?"</h2>
                <ul>
                    <li><strong>"Client 'set':"</strong>" UI state (tabs, modals, toggles). Data that can be lost on refresh."</li>
                    <li><strong>"Server Actions:"</strong>" Business data (user profile, shopping cart, database records). Data that must persist."</li>
                </ul>
            </div>
        </div>
        <style>
            .container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
            .card {
                border: "1px solid #e0e0e0";
                border-radius: "8px";
                padding: "2rem";
                margin-bottom: "2rem";
                background: "white";
                box-shadow: "0 2px 4px rgba(0,0,0,0.05)";
            }
            .title { color: "#333"; margin-bottom: "1rem"; }
            .explanation { color: "#666"; line-height: "1.6"; margin-bottom: "2rem"; }

            /* Tabs Styling */
            .tabs { display: "flex"; border-bottom: "2px solid #eee"; margin-bottom: "1rem"; }
            .tab_btn {
                padding: "0.75rem 1.5rem";
                border: "none";
                background: "transparent";
                cursor: "pointer";
                font-weight: "bold";
                color: "#666";
                border-bottom: "2px solid transparent";
                margin-bottom: "-2px";
            }
            .tab_btn.active { color: "#667eea"; border-bottom-color: "#667eea"; }
            .tab_content { display: "none"; padding: "1rem 0"; }
            .tab_content.active { display: "block"; }

            /* Accordion Styling */
            .accordion_item { border: "1px solid #eee"; border-radius: "4px"; margin-bottom: "0.5rem"; }
            .accordion_header {
                padding: "1rem";
                background: "#f8f9fa";
                cursor: "pointer";
                font-weight: "bold";
                display: "flex";
                justify-content: "space-between";
            }
            .accordion_body { display: "none"; padding: "1rem"; border-top: "1px solid #eee"; }
            .accordion_body.open { display: "block"; }

            .code { background: "#f5f5f5"; padding: "0.2rem 0.4rem"; border-radius: "4px"; font-family: "monospace"; }
        </style>
    }
}

pub async fn lesson10_handler() -> axum::response::Html<String> {
    let component_html = azumi::render_to_string(&lesson10_page());
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Lesson 10: Client State</title>
    <style>body {{ font-family: system-ui; background: #fafafa; margin: 0; }}</style>
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
