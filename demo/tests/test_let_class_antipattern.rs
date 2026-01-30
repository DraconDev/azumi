// This should fail to compile because it uses @let to define a class name

use azumi::html;

fn test_let_class_antipattern() {
    html! {
        // This is the anti-pattern we want to catch
        @let my_class = "my_class";
        <div class={my_class}>"Hello"</div>
        <style>
            .my_class { color: "red"; }
        </style>
    };
}

fn main() {}
