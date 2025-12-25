use azumi::html;

/// Example: Valid form binding
struct UserRegistration {
    username: String,
    email: String,
    password: String,
}

#[allow(dead_code)]
pub fn valid_registration_form() -> impl azumi::Component {
    html! {
        <form bind={UserRegistration}>
            <input name="username" type="text" />
            <input name="email" type="email" />
            <input name="password" type="password" />
            <button type="submit">"Register"</button>
        </form>
    }
}

/// Example: Nested field binding
struct UserProfile {
    name: String,
    address: Address,
}

struct Address {
    street: String,
    city: String,
}

#[allow(dead_code)]
pub fn nested_binding_form() -> impl azumi::Component {
    html! {
        <form bind={UserProfile}>
            <input name="name" type="text" />
            <input name="address.street" type="text" />
            <input name="address.city" type="text" />
        </form>
    }
}

// NEGATIVE EXAMPLE: This would fail to compile with a helpful error
// Uncomment to test:
/*
#[allow(dead_code)]
pub fn invalid_form_with_typo() -> impl azumi::Component {
    html! {
        <form bind={UserRegistration}>
            <input name="username" type="text" />
            <input name="emal" type="email" />  // TYPO: should be "email"
            <input name="password" type="password" />
        </form>
    }
}
*/

#[test]
fn test_valid_forms_compile() {
    // If these compile, the test passes
    let _form1 = valid_registration_form();
    let _form2 = nested_binding_form();
}
