use azumi::html;

fn main() {
    html! {
        // <script>
        //     // Script at top is OK
        //     console.log("ok");
        // </script>

        <div>"Content in middle is OK"</div>

        <style>
            /* Style at bottom is OK */
            .foo { color: "green"; }
        </style>

    };
}
