use azumi::html;

fn main() {
    html! {
        <div>"Script below content should fail"</div>
        <script>
            console.log("fail");
        </script>
    };
}
