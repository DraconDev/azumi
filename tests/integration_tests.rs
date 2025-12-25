//! Integration Tests
//!
//! Tests for complex component compositions and real-world patterns
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Component Composition (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_simple_component() {
    fn card() -> impl azumi::Component {
        html! { <div>"Card content"</div> }
    }
    let component = html! {
        <div>
            {card()}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Card content"));
}

#[test]
fn test_nested_components() {
    fn inner() -> impl azumi::Component {
        html! { <span>"Inner"</span> }
    }
    fn outer() -> impl azumi::Component {
        html! { <div>{inner()}</div> }
    }
    let component = html! { <section>{outer()}</section> };
    let html = test::render(&component);
    assert!(html.contains("Inner") && html.contains("<section>"));
}

#[test]
fn test_conditional_component() {
    fn alert() -> impl azumi::Component {
        html! { <div role="alert">"Warning!"</div> }
    }
    let show_alert = true;
    let component = html! {
        <div>
            @if show_alert {
                {alert()}
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Warning!"));
}

#[test]
fn test_header_component() {
    fn header() -> impl azumi::Component {
        html! {
            <header>
                <h1>"Welcome"</h1>
            </header>
        }
    }
    let component = html! { <div>{header()}</div> };
    let html = test::render(&component);
    assert!(html.contains("<header>") && html.contains("Welcome"));
}

#[test]
fn test_footer_component() {
    fn footer() -> impl azumi::Component {
        html! {
            <footer>
                <p>"© 2024"</p>
            </footer>
        }
    }
    let component = html! { <div>{footer()}</div> };
    let html = test::render(&component);
    assert!(html.contains("<footer>") && html.contains("2024"));
}

#[test]
fn test_badge_component() {
    fn badge() -> impl azumi::Component {
        html! { <span>"5"</span> }
    }
    let component = html! { <div>"Notifications "{badge()}</div> };
    let html = test::render(&component);
    assert!(html.contains("Notifications") && html.contains("5"));
}

#[test]
fn test_loading_spinner() {
    fn spinner() -> impl azumi::Component {
        html! { <div aria-busy="true">"Loading..."</div> }
    }
    let loading = true;
    let component = html! {
        <div>
            @if loading {
                {spinner()}
            } else {
                <p>"Content loaded"</p>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Loading...") && html.contains("aria-busy"));
}

#[test]
fn test_error_message_component() {
    fn error_msg() -> impl azumi::Component {
        html! { <div role="alert">"Invalid input"</div> }
    }
    let component = html! { <form>{error_msg()}</form> };
    let html = test::render(&component);
    assert!(html.contains("Invalid input") && html.contains("role=\"alert\""));
}

#[test]
fn test_icon_component() {
    fn icon() -> impl azumi::Component {
        html! { <span data-icon="star">"★"</span> }
    }
    let component = html! { <button>{icon()}"Favorite"</button> };
    let html = test::render(&component);
    assert!(html.contains("data-icon") && html.contains("Favorite"));
}

#[test]
fn test_card_structure() {
    fn card_header() -> impl azumi::Component {
        html! { <div>"Title"</div> }
    }
    fn card_body() -> impl azumi::Component {
        html! { <div>"Body content"</div> }
    }
    fn card_footer() -> impl azumi::Component {
        html! { <div>"Actions"</div> }
    }
    let component = html! {
        <div>
            {card_header()}
            {card_body()}
            {card_footer()}
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Title") && html.contains("Body") && html.contains("Actions"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Real-World UI Patterns (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_login_form() {
    let component = html! {
        <form action="/login" method="post">
            <div>
                <label for="email">"Email"</label>
                <input type="email" name="email" />
            </div>
            <div>
                <label for="password">"Password"</label>
                <input type="password" name="password" />
            </div>
            <button type="submit">"Log In"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("Email") && html.contains("password") && html.contains("Log In"));
}

#[test]
fn test_signup_form() {
    let component = html! {
        <form action="/signup" method="post">
            <input type="text" name="name" placeholder="Full Name" />
            <input type="email" name="email" placeholder="Email" />
            <input type="password" name="password" placeholder="Password" />
            <button type="submit">"Sign Up"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("signup") && html.contains("Sign Up"));
}

#[test]
fn test_search_form() {
    let component = html! {
        <form role="search" action="/search">
            <input type="search" name="q" placeholder="Search..." />
            <button type="submit">"Search"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"search\"") && html.contains("Search"));
}

#[test]
fn test_contact_form() {
    let component = html! {
        <form action="/contact" method="post">
            <input type="text" name="name" />
            <input type="email" name="email" />
            <textarea name="message">"Message"</textarea>
            <button type="submit">"Send"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("contact") && html.contains("textarea"));
}

#[test]
fn test_product_card() {
    let component = html! {
        <article>
            <img src="/product.jpg" alt="Product" />
            <h3>"Product Name"</h3>
            <p>"$99.99"</p>
            <button>"Add to Cart"</button>
        </article>
    };
    let html = test::render(&component);
    assert!(html.contains("Product Name") && html.contains("$99.99"));
}

#[test]
fn test_user_profile_card() {
    let component = html! {
        <div>
            <img src="/avatar.jpg" alt="User avatar" />
            <h2>"John Doe"</h2>
            <p>"john@example.com"</p>
            <button>"Edit Profile"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("John Doe") && html.contains("Edit Profile"));
}

#[test]
fn test_notification_list() {
    let notifications = vec!["New message", "Order shipped"];
    let component = html! {
        <ul role="list">
            @for msg in &notifications {
                <li>{msg}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("New message") && html.contains("Order shipped"));
}

#[test]
fn test_data_table() {
    let users = vec![("Alice", "alice@example.com"), ("Bob", "bob@example.com")];
    let component = html! {
        <table>
            <thead>
                <tr><th>"Name"</th><th>"Email"</th></tr>
            </thead>
            <tbody>
                @for (name, email) in &users {
                    <tr><td>{name}</td><td>{email}</td></tr>
                }
            </tbody>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice") && html.contains("bob@example.com"));
}

#[test]
fn test_modal_dialog() {
    let modal_title = "modal-title";
    let component = html! {
        <div role="dialog" aria-modal="true" aria-labelledby={modal_title}>
            <h2>"Confirm Action"</h2>
            <p>"Are you sure?"</p>
            <button>"Cancel"</button>
            <button>"Confirm"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Confirm Action") && html.contains("aria-modal"));
}

#[test]
fn test_dropdown_menu() {
    let component = html! {
        <div>
            <button aria-haspopup="menu" aria-expanded="false">"Menu"</button>
            <ul role="menu">
                <li role="menuitem">"Option 1"</li>
                <li role="menuitem">"Option 2"</li>
            </ul>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("aria-haspopup") && html.contains("menuitem"));
}

#[test]
fn test_accordion() {
    let items = vec![("Section 1", "Content 1"), ("Section 2", "Content 2")];
    let component = html! {
        <div>
            @for (title, content) in &items {
                <details>
                    <summary>{title}</summary>
                    <p>{content}</p>
                </details>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Section 1") && html.contains("Content 2"));
}

#[test]
fn test_stepper() {
    let steps = vec![("Complete", true), ("Current", false), ("Pending", false)];
    let component = html! {
        <ol>
            @for (label, done) in &steps {
                <li data-completed={done.to_string()}>{label}</li>
            }
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("Complete") && html.contains("Pending"));
}

#[test]
fn test_rating_stars() {
    let rating = 4;
    let component = html! {
        <div role="img" aria-label="4 out of 5 stars">
            @for i in 1..=5 {
                <span>{if i <= rating { "★" } else { "☆" }}</span>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("★") && html.contains("☆"));
}

#[test]
fn test_tag_list() {
    let tags = vec!["rust", "web", "framework"];
    let component = html! {
        <ul role="list">
            @for tag in &tags {
                <li>{tag}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("rust") && html.contains("framework"));
}

#[test]
fn test_timeline() {
    let events = vec![
        ("2024-01-01", "Project started"),
        ("2024-06-01", "Beta release"),
    ];
    let component = html! {
        <ol>
            @for (date, event) in &events {
                <li><time datetime={*date}>{date}</time><p>{event}</p></li>
            }
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("Project started") && html.contains("Beta release"));
}

#[test]
fn test_pricing_card() {
    let component = html! {
        <div>
            <h3>"Pro Plan"</h3>
            <p>"$29/month"</p>
            <ul>
                <li>"Feature 1"</li>
                <li>"Feature 2"</li>
                <li>"Feature 3"</li>
            </ul>
            <button>"Subscribe"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Pro Plan") && html.contains("$29/month") && html.contains("Subscribe"));
}

#[test]
fn test_testimonial() {
    let component = html! {
        <blockquote>
            <p>"Great product!"</p>
            <footer>
                <cite>"John Doe"</cite>
            </footer>
        </blockquote>
    };
    let html = test::render(&component);
    assert!(html.contains("Great product") && html.contains("<cite>"));
}

#[test]
fn test_faq_section() {
    let faqs = vec![
        ("What is Azumi?", "A Rust web framework."),
        ("Is it fast?", "Yes!"),
    ];
    let component = html! {
        <section>
            <h2>"FAQ"</h2>
            <dl>
                @for (q, a) in &faqs {
                    <dt>{q}</dt>
                    <dd>{a}</dd>
                }
            </dl>
        </section>
    };
    let html = test::render(&component);
    assert!(html.contains("What is Azumi") && html.contains("Yes!"));
}

#[test]
fn test_empty_state() {
    let items: Vec<&str> = vec![];
    let component = html! {
        <div>
            @if items.is_empty() {
                <p>"No items found"</p>
            } else {
                <ul>
                    @for item in &items {
                        <li>{item}</li>
                    }
                </ul>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("No items found"));
}

#[test]
fn test_skeleton_loader() {
    let loading = true;
    let component = html! {
        <div>
            @if loading {
                <div aria-busy="true">
                    <div>"Loading..."</div>
                </div>
            } else {
                <p>"Actual content"</p>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Loading...") && html.contains("aria-busy"));
}
