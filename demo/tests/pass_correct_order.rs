use azumi::html;

fn main() {
    html! {
        // <script>
        //     // Script at top is OK
        //     console.log("ok");
        // </script>

        <style>
            /* Style at bottom is OK */
            .foo { color: green; }
        </style>

        <div>"Content in middle is OK"</div>

    };
}
