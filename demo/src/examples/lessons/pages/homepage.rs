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
