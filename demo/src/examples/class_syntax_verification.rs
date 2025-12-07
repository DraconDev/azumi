use azumi::prelude::*;

#[azumi::component]
pub fn ClassSyntaxVerification() -> impl Component {
    html! {
        <style>
            .dashed-class { color: "red"; }
            .snake_class { color: "blue"; }
        </style>

        // 1. Dashed class in quotes (should work and be scoped)
        <div class="dashed-class">"Dashed"</div>

        // 2. Snake class in quotes (should work and be scoped)
        <div class="snake_class">"Snake Quoted"</div>

        // 3. Snake class in brackets (variable)
        <div class={snake_class}>"Snake Bracket"</div>

        // 4. Multiple classes in quotes
        <div class="dashed-class snake_class">"Multiple"</div>
    }
}
