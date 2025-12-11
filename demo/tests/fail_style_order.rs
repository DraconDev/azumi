use azumi::html;

fn main() {
    html! {
        <div>"This should fail because style is at the top"</div>
        <style>
            .foo { color: "red"; }
        </style>
    };
}
