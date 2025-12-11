use crate::examples::lessons::components::layout::DarkModernLayout;

#[azumi::component]
pub fn container(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div class={content_box}>
            {children}
        </div>
        <style>
            .content_box {
                padding: "2rem";
                border: "1px solid rgba(255,255,255,0.1)";
                border-radius: "12px";
                background: "rgba(15, 23, 42, 0.4)";
                color: "#e2e8f0";
            }
        </style>
    }
}

/// Example: Layout with children
#[azumi::component]
pub fn layout_example() -> impl azumi::Component {
    html! {
        <div>
            <h2 class={layout_title}>"Container with Children"</h2>

            @container() {
                <p>"This content is passed as children"</p>
                <p class={dim_text}>"Children can be any valid Azumi components"</p>
            }
        </div>
        <style>
            .layout_title { font-size: "1.5rem"; color: "#38bdf8"; margin-bottom: "1rem"; font-weight: "600"; }
            .dim_text { color: "#94a3b8"; margin-top: "0.5rem"; }
        </style>
    }
}

/// Example: Nested children components
#[azumi::component]
pub fn nested_children() -> impl azumi::Component {
    html! {
        <div>
            <h3 class={title}>"Nested Children Example"</h3>
            @container() {
                <p class={outer_text}>"Outer content"</p>
                <div class={outer_container}>
                    <p class={inner_text}>"Inner nested content"</p>
                    @container() {
                        <p class={deep_text}>"Deeply nested content"</p>
                    }
                </div>
            }
        </div>
        <style>
            .title { font-size: "1.25rem"; color: "#e2e8f0"; margin-bottom: "1rem"; }
            .outer_text { color: "#94a3b8"; margin-bottom: "1rem"; }
            .outer_container {
                background: "rgba(0,0,0,0.2)";
                padding: "1.5rem";
                border-radius: "8px";
                border: "1px solid rgba(255,255,255,0.05)";
            }
            .inner_text { color: "#cbd5e1"; margin-bottom: "1rem"; }
            .deep_text { color: "#a5f3fc"; font-weight: "500"; }
        </style>
    }
}

/// Example: Children with multiple elements
#[azumi::component]
pub fn multiple_children_example() -> impl azumi::Component {
    html! {
        <div>
            @container() {
                <div class={children_demo}>
                    <div class={child_item}>
                        "Child 1"
                    </div>
                    <div class={child_item}>
                        "Child 2"
                    </div>
                    <div class={child_item}>
                        "Child 3"
                    </div>
                </div>
            }
            <div class={spacer}></div>
            @container() {
                <p>"Multiple children example"</p>
            }
        </div>
        <style>
            .children_demo { display: "grid"; gap: "1rem"; grid-template-columns: "repeat(3, 1fr)"; }
            .child_item {
                padding: "1rem";
                background: "rgba(56, 189, 248, 0.1)";
                border: "1px solid rgba(56, 189, 248, 0.2)";
                text-align: "center";
                border-radius: "8px";
                color: "#38bdf8";
                font-weight: "600";
            }
            .spacer { height: "1.5rem"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson4() -> impl azumi::Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 5: Children Pattern"</h1>
                    <p class={subtitle}>"Components with children parameter"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ Children parameter allows component composition"</li>
                        <li class={point}>"✅ Pass any Azumi component as children"</li>
                        <li class={point}>"✅ Children can contain multiple elements"</li>
                        <li class={point}>"✅ Enables flexible layout patterns"</li>
                        <li class={point}>"✅ Maintains clean component boundaries"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        @layout_example()
                    </div>
                    <div class={example_card}>
                        @nested_children()
                    </div>
                    <div class={example_card}>
                        @multiple_children_example()
                    </div>
                </section>
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #38bdf8, #818cf8)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .subtitle { font-size: "1.25rem"; color: "#94a3b8"; }

                .key_points {
                    background: "rgba(30, 41, 59, 0.5)";
                    padding: "2rem";
                    border-radius: "16px";
                    margin-bottom: "3rem";
                    border: "1px solid rgba(255,255,255,0.05)";
                    backdrop-filter: "blur(10px)";
                }
                .section_title {
                    font-size: "1.5rem";
                    color: "#f1f5f9";
                    margin-bottom: "1.5rem";
                    border-bottom: "1px solid rgba(255,255,255,0.1)";
                    padding-bottom: "0.5rem";
                }
                .points_list { list-style: "none"; padding: "0"; display: "grid"; gap: "1rem"; }
                .point {
                    color: "#e2e8f0";
                    padding: "0.75rem";
                    background: "rgba(255,255,255,0.03)";
                    border-radius: "8px";
                    font-size: "1.1rem";
                }

                .examples { display: "grid"; gap: "2rem"; }
                .example_card {
                    border: "1px solid rgba(255,255,255,0.1)";
                    padding: "2rem";
                    border-radius: "16px";
                    background: "rgba(15, 23, 42, 0.6)";
                }
            </style>
        }
    }
}

// Handler for Axum
pub async fn lesson4_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson4()))
}
