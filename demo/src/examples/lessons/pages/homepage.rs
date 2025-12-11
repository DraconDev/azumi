use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::prelude::*;

#[azumi::component]
pub fn homepage() -> impl Component {
    html! {
        @DarkModernLayout() {
            @Lessons()
        }
    }
}

#[azumi::component]
pub fn Lessons() -> impl Component {
    html! {
        <div>
            <div class={hero_section}>
                <h1 class={hero_title}>"Master Azumi"</h1>
                <p class={hero_subtitle}>
                    "A comprehensive, interactive journey through the Azumi framework.
                    From basic components to full-scale applications, learn how to build the future of web development."
                </p>
            </div>

            <div class={grid_container}>
                // Basics
                @LessonCard(num="00", title="Introduction", desc="Understanding the component structure.", link="/lesson-0")
                @LessonCard(num="01", title="Components", desc="Building your first reusable blocks.", link="/lesson-1")
                @LessonCard(num="02", title="CSS Scoping", desc="How styles are isolated safely.", link="/lesson-2")
                @LessonCard(num="03", title="Global Styles", desc="Managing global vs local CSS.", link="/lesson-3")

                // Structure & Forms
                @LessonCard(num="04", title="HTML Structure", desc="Compile-time HTML validation.", link="/lesson-4")
                @LessonCard(num="05", title="Accessibility", desc="Building inclusive interfaces.", link="/lesson-5")
                @LessonCard(num="06", title="Basic Forms", desc="Standard form handling patterns.", link="/lesson-6")
                @LessonCard(num="07", title="Event Handling", desc="Interactivity with event listeners.", link="/lesson-7")

                // Advanced
                @LessonCard(num="08", title="State Management", desc="Managing complex component state.", link="/lesson-8")
                @LessonCard(num="09", title="Advanced Patterns", desc="Composition and slots.", link="/lesson-9")
                @LessonCard(num="10", title="Performance", desc="Optimizing rendering and updates.", link="/lesson-10")
                @LessonCard(num="11", title="Async Patterns", desc="Loading states and error handling.", link="/lesson-11")

                // New Features
                @LessonCard(num="12", title="Image Optimization", desc="Automatic lazy loading & attributes.", link="/lesson-12")
                @LessonCard(num="13", title="Live Forms", desc="Real-time validation and feedback.", link="/lesson-13")
                @LessonCard(num="14", title="Composition", desc="Building complex Live UIs.", link="/lesson-14")
                @LessonCard(num="15", title="Full Application", desc="A complete Todo App demo.", link="/lesson-15")
            </div>
        </div>
        <style>
            .hero_section {
                text-align: "center";
                padding: "4rem 1rem";
                margin-bottom: "2rem";
                animation: "fadeIn 0.8s ease-out";
            }
            .hero_title {
                font-size: "4rem";
                font-weight: "800";
                margin-bottom: "1rem";
                background: "linear-gradient(135deg, #60a5fa, #a78bfa, #f472b6)";
                -webkit-background-clip: "text";
                -webkit-text-fill-color: "transparent";
                filter: "drop-shadow(0 2px 4px rgba(0,0,0,0.3))";
            }
            .hero_subtitle {
                font-size: "1.25rem";
                color: "#9ca3af";
                max-width: "600px";
                margin: "0 auto";
                line-height: "1.6";
            }

            .grid_container {
                max-width: "1200px";
                margin: "0 auto";
                padding: "0 1rem 4rem 1rem";
                display: "grid";
                grid-template-columns: "repeat(auto-fill, minmax(300px, 1fr))";
                gap: "2rem";
            }

            .lesson_card {
                background: "rgba(30, 41, 59, 0.7)";
                border: "1px solid rgba(255, 255, 255, 0.1)";
                border-radius: "16px";
                padding: "2rem";
                transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)";
                backdrop-filter: "blur(10px)";
                display: "flex";
                flex-direction: "column";
                height: "100%";
                position: "relative";
                overflow: "hidden";
            }
            .lesson_card:hover {
                transform: "translateY(-5px)";
                background: "rgba(30, 41, 59, 0.9)";
                border-color: "#60a5fa";
                box-shadow: "0 20px 25px -5px rgba(0, 0, 0, 0.2), 0 10px 10px -5px rgba(0, 0, 0, 0.1)";
            }
            // Glow effect on hover 
            .lesson_card::before {
                content: "''";
                position: "absolute";
                top: "0"; left: "0"; right: "0"; height: "2px";
                background: "linear-gradient(90deg, #60a5fa, #a78bfa)";
                opacity: "0";
                transition: "opacity 0.3s";
            }
            .lesson_card:hover::before {
                opacity: "1";
            }

            .card_number {
                font-size: "0.875rem";
                font-weight: "600";
                color: "#60a5fa";
                margin-bottom: "0.5rem";
                text-transform: "uppercase";
                letter-spacing: "0.05em";
            }
            .card_title {
                font-size: "1.5rem";
                font-weight: "700";
                color: "#f3f4f6";
                margin-bottom: "1rem";
                line-height: "1.2";
            }
            .card_desc {
                color: "#9ca3af";
                margin-bottom: "1.5rem";
                flex-grow: "1";
                line-height: "1.5";
            }
            .card_link {
                display: "inline-flex";
                align-items: "center";
                color: "#fff";
                text-decoration: "none";
                font-weight: "600";
                padding: "0.75rem 1.5rem";
                background: "linear-gradient(135deg, #3b82f6, #2563eb)";
                border-radius: "8px";
                transition: "all 0.2s";
                align-self: "start";
            }
            .card_link:hover {
                background: "linear-gradient(135deg, #2563eb, #1d4ed8)";
                transform: "translateX(2px)";
            }

            @keyframes fadeIn {
                from { opacity: "0"; transform: "translateY(20px)"; }
                to { opacity: "1"; transform: "translateY(0)"; }
            }
        </style>
    }
}

#[azumi::component]
fn LessonCard<'a>(
    num: &'a str,
    title: &'a str,
    desc: &'a str,
    link: &'a str,
) -> impl Component + 'a {
    html! {
        <style>
            .lesson_card {
                display: "flex";
                flex-direction: "column";
                align-items: "center";
                justify-content: "center";
                width: "200px";
                height: "200px";
                border-radius: "8px";
                background: "#1e293b";
                color: "#f3f4f6";
                box-shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)";
                transition: "all 0.2s";
            }
            .card_number {
                font-size: "1.5rem";
                font-weight: "700";
                color: "#60a5fa";
                margin-bottom: "0.5rem";
                text-transform: "uppercase";
                letter-spacing: "0.05em";
            }
            .card_title {
                font-size: "1.5rem";
                font-weight: "700";
                color: "#f3f4f6";
                margin-bottom: "1rem";
                line-height: "1.2";
            }
            .card_desc {
                color: "#9ca3af";
                margin-bottom: "1rem";
            }
            .card_link {
                display: "inline-flex";
                align-items: "center";
                color: "#fff";
                text-decoration: "none";
                font-weight: "600";
                padding: "0.75rem 1.5rem";
                background: "linear-gradient(135deg, #3b82f6, #2563eb)";
                border-radius: "8px";
                transition: "all 0.2s";
                align-self: "start";
            }

        </style>
        <div class={lesson_card}>
            <div class={card_number}>"LESSON " {num}</div>
            <h3 class={card_title}>{title}</h3>
            <p class={card_desc}>{desc}</p>
            <a href={link} class={card_link}>"Start Lesson →"</a>
        </div>
    }
}

pub async fn homepage_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&homepage()))
}
