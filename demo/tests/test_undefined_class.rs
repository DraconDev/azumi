// This should fail because 'nice' is not defined in <style>

use azumi::html;

#[azumi::component]
pub fn basic_component() -> impl azumi::Component {
    let nice = "All CSS is automatically scoped to this component";
    html! {
        <div class={container}>
            <h1 class={title}>"Basic Azumi Component"</h1>
            <p class={desc}>"This demonstrates the basic component structure"</p>
            <p class={nice}>"All CSS is automatically scoped to this component"</p>
        </div>
        <style>
            .container {
                padding: "1.5rem";
                border: "1px solid rgba(255,255,255,0.1)";
                background: "rgba(15, 23, 42, 0.5)";
                border-radius: "12px";
            }
            .title { color: "#38bdf8"; margin-bottom: "0.5rem"; font-size: "1.25rem"; }
            .desc { color: "#94a3b8"; }
        </style>
    }
}

fn main() {}
