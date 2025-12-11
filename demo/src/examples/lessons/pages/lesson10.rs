use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::prelude::*;

/// Lesson 10: Client-Side State with `set`
///
/// Learn when to use client-side state vs server state.
#[azumi::component]
pub fn lesson10_page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 10: Client-Side State"</h1>
                    <p class={explanation}>
                        "Azumi is server-first, but sometimes you need pure client-side interactivity for "
                        "ephemeral UI state like tabs, accordions, and toggles. "
                        "For this, we use the "<span class={code}>"set"</span>" command."
                    </p>
                </header>

                // ==========================================
                // Example 1: Tabs
                // ==========================================
                <div class={card} az-scope="{ \"active_tab\": \"rust\" }">
                    <h2 class={title}>"Example 1: Tabs"</h2>
                    <p class={subtitle}>"State is local to the browser. Refreshing resets it."</p>

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
                        <h3 class={content_title}>"Rust"</h3>
                        <p>"Rust is blazingly fast and memory-efficient with no garbage collector."</p>
                    </div>
                    <div class={tab_content} az-bind:class:active="active_tab == 'python'">
                        <h3 class={content_title}>"Python"</h3>
                        <p>"Python is great for data science, AI, and scripting."</p>
                    </div>
                    <div class={tab_content} az-bind:class:active="active_tab == 'js'">
                        <h3 class={content_title}>"JavaScript"</h3>
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
                            <span az-bind:text="acc1 ? '−' : '+'" class={toggle_icon}>"+"</span>
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
                            <span az-bind:text="acc2 ? '−' : '+'" class={toggle_icon}>"+"</span>
                        </div>
                        <div class={accordion_body} az-bind:class:open="acc2">
                            <p>"It uses Rust macros to analyze your code and generate optimized HTML and minimal JS."</p>
                        </div>
                    </div>
                </div>

                <div class={card}>
                    <h2 class={title}>"When to use what?"</h2>
                    <ul class={info_list}>
                        <li class={info_item}><strong class={strong}>"Client 'set':"</strong>" UI state (tabs, modals, toggles). Data that can be lost on refresh."</li>
                        <li class={info_item}><strong class={strong}>"Server Actions:"</strong>" Business data (user profile, shopping cart, database records). Data that must persist."</li>
                    </ul>
                </div>
            </div>
            <style>
                .container { max-width: "800px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #facc15, #f59e0b)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .explanation { color: "#94a3b8"; line-height: "1.6"; margin-bottom: "2rem"; font-size: "1.1rem"; }
                .code { background: "rgba(255,255,255,0.1)"; padding: "0.2rem 0.4rem"; border-radius: "4px"; font-family: "monospace"; color: "#facc15"; }

                .card {
                    border: "1px solid rgba(255,255,255,0.05)";
                    border-radius: "16px";
                    padding: "2rem";
                    margin-bottom: "2rem";
                    background: "rgba(30, 41, 59, 0.6)";
                    backdrop-filter: "blur(10px)";
                    color: "#cbd5e1";
                }
                .title { color: "#e2e8f0"; margin-bottom: "0.5rem"; font-size: "1.5rem"; }
                .subtitle { color: "#94a3b8"; margin-bottom: "1.5rem"; font-size: "0.9rem"; }

                /* Tabs Styling */
                .tabs { display: "flex"; border-bottom: "1px solid rgba(255,255,255,0.1)"; margin-bottom: "1.5rem"; gap: "0.5rem"; }
                .tab_btn {
                    padding: "0.75rem 1.5rem";
                    border: "none";
                    background: "transparent";
                    cursor: "pointer";
                    font-weight: "600";
                    color: "#94a3b8";
                    border-bottom: "2px solid transparent";
                    margin-bottom: "-1px";
                    transition: "all 0.2s";
                }
                .tab_btn:hover { color: "#e2e8f0"; }
                .tab_btn.active { color: "#facc15"; border-bottom-color: "#facc15"; }
                .tab_content { display: "none"; padding: "1rem 0"; animation: "fadeIn 0.3s ease-out"; }
                .tab_content.active { display: "block"; }
                .content_title { color: "#fde047"; margin-bottom: "0.5rem"; font-size: "1.25rem"; }

                @keyframes fadeIn { from { opacity: "0"; transform: "translateY(5px)"; } to { opacity: "1"; transform: "translateY(0)"; } }

                /* Accordion Styling */
                .accordion_item {
                    border: "1px solid rgba(255,255,255,0.1)";
                    border-radius: "8px";
                    margin-bottom: "1rem";
                    overflow: "hidden";
                    background: "rgba(15, 23, 42, 0.3)";
                }
                .accordion_header {
                    padding: "1rem";
                    background: "rgba(255,255,255,0.02)";
                    cursor: "pointer";
                    font-weight: "600";
                    display: "flex";
                    justify-content: "space-between";
                    align-items: "center";
                    color: "#e2e8f0";
                    transition: "background 0.2s";
                }
                .accordion_header:hover { background: "rgba(255,255,255,0.05)"; }
                .toggle_icon { color: "#facc15"; font-weight: "bold"; font-size: "1.2rem"; }
                .accordion_body { display: "none"; padding: "1.5rem"; border-top: "1px solid rgba(255,255,255,0.05)"; color: "#cbd5e1"; }
                .accordion_body.open { display: "block"; }

                .info_list { list-style: "none"; padding: "0"; display: "grid"; gap: "1rem"; }
                .info_item { padding: "1rem"; background: "rgba(0,0,0,0.2)"; border-radius: "8px"; color: "#cbd5e1"; }
                .strong { color: "#facc15"; margin-right: "0.5rem"; }
            </style>
        }
    }
}

pub async fn lesson10_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson10_page()))
}
