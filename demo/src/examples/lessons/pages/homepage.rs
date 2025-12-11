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
        <div class={page_wrapper}>
            // Hero Section
            <header class={hero}>
                <div class={hero_badge}>"🚀 The Future of Rust Web Development"</div>
                <h1 class={hero_title}>"Master Azumi"</h1>
                <p class={hero_subtitle}>
                    "A comprehensive, interactive journey through the Azumi framework.
                    From basic components to production-ready applications."
                </p>
                <div class={hero_stats}>
                    <div class={stat}>
                        <span class={stat_number}>"20"</span>
                        <span class={stat_label}>"Lessons"</span>
                    </div>
                    <div class={stat}>
                        <span class={stat_number}>"∞"</span>
                        <span class={stat_label}>"Possibilities"</span>
                    </div>
                    <div class={stat}>
                        <span class={stat_number}>"0"</span>
                        <span class={stat_label}>"JS Required"</span>
                    </div>
                </div>
            </header>

            // Course Sections
            <section class={section}>
                <h2 class={section_title}>"🌱 Foundations"</h2>
                <div class={grid}>
                    @LessonCard(num="00", title="Introduction", desc="Understanding the component structure.", link="/lesson-0", accent="#10b981")
                    @LessonCard(num="01", title="Components", desc="Building your first reusable blocks.", link="/lesson-1", accent="#10b981")
                    @LessonCard(num="02", title="CSS Scoping", desc="How styles are isolated safely.", link="/lesson-2", accent="#10b981")
                    @LessonCard(num="03", title="Global Styles", desc="Managing global vs local CSS.", link="/lesson-3", accent="#10b981")
                </div>
            </section>

            <section class={section}>
                <h2 class={section_title}>"🏗️ Structure & Forms"</h2>
                <div class={grid}>
                    @LessonCard(num="04", title="HTML Structure", desc="Compile-time HTML validation.", link="/lesson-4", accent="#3b82f6")
                    @LessonCard(num="05", title="Accessibility", desc="Building inclusive interfaces.", link="/lesson-5", accent="#3b82f6")
                    @LessonCard(num="06", title="Basic Forms", desc="Standard form handling patterns.", link="/lesson-6", accent="#3b82f6")
                    @LessonCard(num="07", title="Event Handling", desc="Interactivity with event listeners.", link="/lesson-7", accent="#3b82f6")
                </div>
            </section>

            <section class={section}>
                <h2 class={section_title}>"⚡ Advanced Patterns"</h2>
                <div class={grid}>
                    @LessonCard(num="08", title="State Management", desc="Managing complex component state.", link="/lesson-8", accent="#8b5cf6")
                    @LessonCard(num="09", title="Advanced Patterns", desc="Composition and slots.", link="/lesson-9", accent="#8b5cf6")
                    @LessonCard(num="10", title="Performance", desc="Optimizing rendering and updates.", link="/lesson-10", accent="#8b5cf6")
                    @LessonCard(num="11", title="Async Patterns", desc="Loading states and error handling.", link="/lesson-11", accent="#8b5cf6")
                </div>
            </section>

            <section class={section}>
                <h2 class={section_title}>"🔥 Production Ready"</h2>
                <div class={grid}>
                    @LessonCard(num="12", title="Image Optimization", desc="Automatic lazy loading & attributes.", link="/lesson-12", accent="#f59e0b")
                    @LessonCard(num="13", title="Live Forms", desc="Real-time validation and feedback.", link="/lesson-13", accent="#f59e0b")
                    @LessonCard(num="14", title="Composition", desc="Building complex Live UIs.", link="/lesson-14", accent="#f59e0b")
                    @LessonCard(num="15", title="SQL Basics", desc="Direct database queries.", link="/lesson-15-sql-basics", accent="#f59e0b")
                </div>
            </section>

            <section class={section}>
                <h2 class={section_title}>"🛡️ Enterprise Features"</h2>
                <div class={grid}>
                    @LessonCard(num="16", title="Async Database", desc="SQLite integration with optimistic UI.", link="/lesson-16-async-db", accent="#ef4444")
                    @LessonCard(num="17", title="Testing", desc="Unit and Integration testing strategies.", link="/lesson-17-testing", accent="#ef4444")
                    @LessonCard(num="18", title="Security", desc="XSS prevention and safe practices.", link="/lesson-18-security", accent="#ef4444")
                    @LessonCard(num="19", title="Authentication", desc="User sessions and login flows.", link="/lesson-19-auth", accent="#ef4444")
                </div>
            </section>
        </div>

        <style>
            .page_wrapper {
                padding-bottom: "6rem";
            }

            // Hero
            .hero {
                text-align: "center";
                padding: "5rem 1rem 4rem";
                position: "relative";
            }
            .hero_badge {
                display: "inline-block";
                padding: "0.5rem 1rem";
                background: "rgba(99, 102, 241, 0.15)";
                border: "1px solid rgba(99, 102, 241, 0.3)";
                border-radius: "9999px";
                font-size: "0.875rem";
                font-weight: "500";
                color: "#a5b4fc";
                margin-bottom: "1.5rem";
                animation: "fadeInDown 0.6s ease-out";
            }
            .hero_title {
                font-size: "clamp(2.5rem, 8vw, 5rem)";
                font-weight: "800";
                margin: "0 0 1.5rem";
                background: "linear-gradient(135deg, #fff 0%, #60a5fa 50%, #a78bfa 100%)";
                -webkit-background-clip: "text";
                -webkit-text-fill-color: "transparent";
                animation: "fadeInUp 0.6s ease-out 0.1s both";
                letter-spacing: "-0.03em";
            }
            .hero_subtitle {
                font-size: "1.25rem";
                color: "#94a3b8";
                max-width: "600px";
                margin: "0 auto 2.5rem";
                line-height: "1.7";
                animation: "fadeInUp 0.6s ease-out 0.2s both";
            }
            .hero_stats {
                display: "flex";
                justify-content: "center";
                gap: "3rem";
                animation: "fadeInUp 0.6s ease-out 0.3s both";
            }
            .stat {
                display: "flex";
                flex-direction: "column";
                align-items: "center";
            }
            .stat_number {
                font-size: "2.5rem";
                font-weight: "700";
                color: "#f1f5f9";
            }
            .stat_label {
                font-size: "0.875rem";
                color: "#64748b";
                text-transform: "uppercase";
                letter-spacing: "0.05em";
            }

            // Sections
            .section {
                max-width: "1200px";
                margin: "0 auto 3rem";
                padding: "0 1.5rem";
            }
            .section_title {
                font-size: "1.5rem";
                font-weight: "600";
                color: "#e2e8f0";
                margin-bottom: "1.5rem";
                padding-bottom: "0.75rem";
                border-bottom: "1px solid rgba(255,255,255,0.1)";
            }

            // Grid
            .grid {
                display: "grid";
                grid-template-columns: "repeat(auto-fill, minmax(280px, 1fr))";
                gap: "1.5rem";
            }

            // Animations
            @keyframes fadeInUp {
                from { opacity: "0"; transform: "translateY(20px)"; }
                to { opacity: "1"; transform: "translateY(0)"; }
            }
            @keyframes fadeInDown {
                from { opacity: "0"; transform: "translateY(-10px)"; }
                to { opacity: "1"; transform: "translateY(0)"; }
            }
        </style>

    }
}

#[azumi::component]
#[allow(non_snake_case)]
fn LessonCard<'a>(
    num: &'a str,
    title: &'a str,
    desc: &'a str,
    link: &'a str,
    accent: &'a str,
) -> impl Component + 'a {
    html! {
        <div class={lesson_card} style={format!("--accent: {}", accent)}>
            <div class={card_accent}></div>
            <div class={card_number}>"LESSON " {num}</div>
            <h3 class={card_title}>{title}</h3>
            <p class={card_desc}>{desc}</p>
            <a href={link} class={card_link}>"Start Lesson →"</a>
        </div>
        <style>
            .lesson_card {
                background: "rgba(30, 41, 59, 0.6)";
                border: "1px solid rgba(255, 255, 255, 0.08)";
                border-radius: "12px";
                padding: "1.5rem";
                display: "flex";
                flex-direction: "column";
                position: "relative";
                overflow: "hidden";
                transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)";
                backdrop-filter: "blur(8px)";
            }
            .lesson_card:hover {
                transform: "translateY(-4px)";
                background: "rgba(30, 41, 59, 0.85)";
                border-color: "var(--accent, #60a5fa)";
                box-shadow: "0 20px 40px -12px rgba(0, 0, 0, 0.35)";
            }
            .card_accent {
                position: "absolute";
                top: "0";
                left: "0";
                right: "0";
                height: "3px";
                background: "var(--accent, #60a5fa)";
                opacity: "0.7";
                transition: "opacity 0.3s";
            }
            .lesson_card:hover .card_accent {
                opacity: "1";
            }
            .card_number {
                font-size: "0.75rem";
                font-weight: "600";
                color: "var(--accent, #60a5fa)";
                margin-bottom: "0.5rem";
                text-transform: "uppercase";
                letter-spacing: "0.08em";
            }
            .card_title {
                font-size: "1.25rem";
                font-weight: "600";
                color: "#f1f5f9";
                margin: "0 0 0.75rem";
                line-height: "1.3";
            }
            .card_desc {
                color: "#94a3b8";
                font-size: "0.9rem";
                margin-bottom: "1.25rem";
                flex-grow: "1";
                line-height: "1.5";
            }
            .card_link {
                display: "inline-flex";
                align-items: "center";
                color: "#fff";
                text-decoration: "none";
                font-weight: "500";
                font-size: "0.875rem";
                padding: "0.6rem 1.25rem";
                background: "var(--accent, #3b82f6)";
                border-radius: "6px";
                transition: "all 0.2s";
                align-self: "start";
            }
            .card_link:hover {
                filter: "brightness(1.1)";
                transform: "translateX(2px)";
            }
        </style>
    }
}

pub async fn homepage_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&homepage()))
}
