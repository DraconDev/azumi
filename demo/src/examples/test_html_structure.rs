use azumi::html;

// Test all 4 HTML structural validation rules

// ❌ Rule 1: Invalid table child
pub fn test_table_invalid_child() -> impl azumi::Component {
    html! {
        <table>
            <div>"This will be hoisted outside"</div>
        </table>
    }
}

// ✅ Rule 1: Valid table children
pub fn test_table_valid() -> impl azumi::Component {
    html! {
        <table>
            <thead>
                <tr>
                    <th>"Header"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"Data"</td>
                </tr>
            </tbody>
        </table>
    }
}

// ❌ Rule 2: Invalid list child
pub fn test_list_invalid_child() -> impl azumi::Component {
    html! {
        <ul>
            <div>"Breaks accessibility"</div>
        </ul>
    }
}

// ✅ Rule 2: Valid list children
pub fn test_list_valid() -> impl azumi::Component {
    html! {
        <ul>
            <li>"Item 1"</li>
            <li>"Item 2"</li>
        </ul>
    }
}

// ❌ Rule 3: Nested form
pub fn test_nested_forms() -> impl azumi::Component {
    html! {
        <form>
            <form>
                <input type="text" />
            </form>
        </form>
    }
}

// ✅ Rule 3: Valid single form
pub fn test_single_form() -> impl azumi::Component {
    html! {
        <form>
            <input type="text" />
            <button type="submit">"Submit"</button>
        </form>
    }
}

// ❌ Rule 4: Button with interactive child
pub fn test_button_with_link() -> impl azumi::Component {
    html! {
        <button>
            <a href="/test">"Click"</a>
        </button>
    }
}

// ✅ Rule 4: Valid button
pub fn test_button_valid() -> impl azumi::Component {
    html! {
        <button>
            <span>"Click me"</span>
        </button>
    }
}
