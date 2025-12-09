use azumi::html;

#[allow(dead_code)]
pub fn form() -> impl azumi::Component {
    html! {
        <form>
            <input type="text" name="name" />
            <input type="email" name="email" />
            <button type="submit">"Submit"</button>
        </form>
    }
}
