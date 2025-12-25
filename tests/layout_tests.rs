//! Layout Pattern Tests
//!
//! Tests for common layout patterns
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Page Layouts (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_basic_page_layout() {
    let component = html! {
        <div>
            <header>"Header"</header>
            <main>"Content"</main>
            <footer>"Footer"</footer>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("<header>") && html.contains("<main>") && html.contains("<footer>"));
}

#[test]
fn test_holy_grail_layout() {
    let component = html! {
        <div>
            <header>"Header"</header>
            <div>
                <nav>"Left Sidebar"</nav>
                <main>"Content"</main>
                <aside>"Right Sidebar"</aside>
            </div>
            <footer>"Footer"</footer>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("<nav>") && html.contains("<aside>"));
}

#[test]
fn test_sidebar_layout() {
    let component = html! {
        <div>
            <aside>"Sidebar"</aside>
            <main>"Main Content"</main>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Sidebar") && html.contains("Main Content"));
}

#[test]
fn test_full_width_layout() {
    let component = html! {
        <div>
            <div>"Hero Section"</div>
            <div>"Content"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Hero Section"));
}

#[test]
fn test_card_grid_layout() {
    let cards = vec!["Card 1", "Card 2", "Card 3", "Card 4"];
    let component = html! {
        <div>
            @for card in &cards {
                <article>{card}</article>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Card 1") && html.contains("Card 4"));
}

#[test]
fn test_masonry_items() {
    let items = vec!["Item 1", "Item 2", "Item 3"];
    let component = html! {
        <div>
            @for item in &items {
                <div>{item}</div>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Item 1") && html.contains("Item 3"));
}

#[test]
fn test_12_column_grid() {
    let component = html! {
        <div>
            <div data-cols="4">"Left"</div>
            <div data-cols="4">"Center"</div>
            <div data-cols="4">"Right"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("data-cols"));
}

#[test]
fn test_flex_container() {
    let component = html! {
        <div data-display="flex">
            <div>"Item 1"</div>
            <div>"Item 2"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("data-display="));
}

#[test]
fn test_centered_container() {
    let component = html! {
        <div data-container="centered">
            <p>"Centered content"</p>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Centered content"));
}

#[test]
fn test_sticky_header() {
    let component = html! {
        <header data-sticky="true">
            <nav>"Navigation"</nav>
        </header>
    };
    let html = test::render(&component);
    assert!(html.contains("data-sticky"));
}

#[test]
fn test_fixed_footer() {
    let component = html! {
        <footer data-fixed="true">
            <p>"Fixed footer"</p>
        </footer>
    };
    let html = test::render(&component);
    assert!(html.contains("data-fixed"));
}

#[test]
fn test_overlay_layout() {
    let component = html! {
        <div data-overlay="true">
            <div data-content="true">"Modal content"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("data-overlay"));
}

#[test]
fn test_split_screen() {
    let component = html! {
        <div>
            <div data-split="left">"Left Panel"</div>
            <div data-split="right">"Right Panel"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Left Panel") && html.contains("Right Panel"));
}

#[test]
fn test_dashboard_layout() {
    let component = html! {
        <div>
            <header>"Dashboard Header"</header>
            <nav>"Sidebar Nav"</nav>
            <main>
                <section>"Stats"</section>
                <section>"Charts"</section>
            </main>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Dashboard") && html.contains("Sidebar"));
}

#[test]
fn test_blog_layout() {
    let component = html! {
        <article>
            <header>
                <h1>"Blog Post Title"</h1>
                <time datetime="2024-01-01">"Jan 1"</time>
            </header>
            <div>"Post content"</div>
            <footer>"Author info"</footer>
        </article>
    };
    let html = test::render(&component);
    assert!(html.contains("Blog Post") && html.contains("Author"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Navigation Patterns (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_horizontal_nav() {
    let links = vec![("Home", "/"), ("About", "/about"), ("Contact", "/contact")];
    let component = html! {
        <nav>
            <ul>
                @for (label, href) in &links {
                    <li><a href={*href}>{label}</a></li>
                }
            </ul>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Home") && html.contains("Contact"));
}

#[test]
fn test_vertical_nav() {
    let component = html! {
        <nav data-direction="vertical">
            <a href="/">"Dashboard"</a>
            <a href="/users">"Users"</a>
            <a href="/settings">"Settings"</a>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Dashboard") && html.contains("Settings"));
}

#[test]
fn test_breadcrumbs() {
    let crumbs = vec!["Home", "Products", "Electronics", "Phones"];
    let component = html! {
        <nav aria-label="Breadcrumb">
            <ol>
                @for (i, crumb) in crumbs.iter().enumerate() {
                    <li aria-current={if i == crumbs.len() - 1 { "page" } else { "" }}>{crumb}</li>
                }
            </ol>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Home") && html.contains("Phones"));
}

#[test]
fn test_pagination() {
    let pages = vec![1, 2, 3, 4, 5];
    let current = 3;
    let component = html! {
        <nav aria-label="Pagination">
            <ul>
                <li><a href="?page=2">"Previous"</a></li>
                @for page in &pages {
                    <li aria-current={if *page == current { "page" } else { "" }}>
                        <a href={format!("?page={}", page)}>{page}</a>
                    </li>
                }
                <li><a href="?page=4">"Next"</a></li>
            </ul>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Previous") && html.contains("Next"));
}

#[test]
fn test_tab_navigation() {
    let tabs = vec!["Overview", "Features", "Pricing", "FAQ"];
    let component = html! {
        <div role="tablist">
            @for (i, tab) in tabs.iter().enumerate() {
                <button role="tab" aria-selected={if i == 0 { "true" } else { "false" }}>{tab}</button>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"tablist\"") && html.contains("Overview"));
}

#[test]
fn test_accordion_nav() {
    let component = html! {
        <nav>
            <details>
                <summary>"Products"</summary>
                <ul>
                    <li><a href="/phones">"Phones"</a></li>
                    <li><a href="/tablets">"Tablets"</a></li>
                </ul>
            </details>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("<details>") && html.contains("<summary>"));
}

#[test]
fn test_hamburger_menu() {
    let component = html! {
        <button aria-label="Menu" aria-expanded="false">
            <span>"☰"</span>
        </button>
    };
    let html = test::render(&component);
    assert!(html.contains("aria-label=\"Menu\""));
}

#[test]
fn test_footer_nav() {
    let component = html! {
        <footer>
            <nav>
                <div>
                    <h4>"Company"</h4>
                    <a href="/about">"About"</a>
                    <a href="/careers">"Careers"</a>
                </div>
                <div>
                    <h4>"Support"</h4>
                    <a href="/help">"Help"</a>
                    <a href="/docs">"Docs"</a>
                </div>
            </nav>
        </footer>
    };
    let html = test::render(&component);
    assert!(html.contains("Company") && html.contains("Support"));
}

#[test]
fn test_top_nav_with_logo() {
    let component = html! {
        <header>
            <a href="/">"Logo"</a>
            <nav>
                <a href="/products">"Products"</a>
                <a href="/pricing">"Pricing"</a>
            </nav>
            <button>"Sign In"</button>
        </header>
    };
    let html = test::render(&component);
    assert!(html.contains("Logo") && html.contains("Sign In"));
}

#[test]
fn test_skip_link() {
    let component = html! {
        <a href="#main-content">"Skip to main content"</a>
    };
    let html = test::render(&component);
    assert!(html.contains("Skip to main"));
}

#[test]
fn test_back_to_top() {
    let component = html! {
        <a href="#top" aria-label="Back to top">"↑"</a>
    };
    let html = test::render(&component);
    assert!(html.contains("#top"));
}

#[test]
fn test_social_links() {
    let socials = vec![
        ("Twitter", "https://twitter.com"),
        ("GitHub", "https://github.com"),
    ];
    let component = html! {
        <nav aria-label="Social links">
            @for (name, url) in &socials {
                <a href={*url} target="_blank" rel="noopener">{name}</a>
            }
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Twitter") && html.contains("GitHub"));
}

#[test]
fn test_language_switcher() {
    let languages = vec![("English", "en"), ("Español", "es"), ("Français", "fr")];
    let component = html! {
        <select name="language" aria-label="Select language">
            @for (name, code) in &languages {
                <option value={*code}>{name}</option>
            }
        </select>
    };
    let html = test::render(&component);
    assert!(html.contains("English") && html.contains("Español"));
}

#[test]
fn test_user_menu() {
    let component = html! {
        <div>
            <button aria-haspopup="true">"User"</button>
            <ul role="menu">
                <li role="menuitem"><a href="/profile">"Profile"</a></li>
                <li role="menuitem"><a href="/settings">"Settings"</a></li>
                <li role="menuitem"><button>"Sign out"</button></li>
            </ul>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("aria-haspopup") && html.contains("Profile"));
}

#[test]
fn test_search_bar() {
    let component = html! {
        <form role="search">
            <input type="search" name="q" placeholder="Search..." aria-label="Search" />
            <button type="submit">"🔍"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"search\"") && html.contains("type=\"search\""));
}

#[test]
fn test_mega_menu() {
    let component = html! {
        <nav>
            <div>
                <button aria-expanded="false">"Products"</button>
                <div>
                    <div>
                        <h4>"Category 1"</h4>
                        <a href="/cat1/item1">"Item 1"</a>
                    </div>
                    <div>
                        <h4>"Category 2"</h4>
                        <a href="/cat2/item1">"Item 1"</a>
                    </div>
                </div>
            </div>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Category 1") && html.contains("Category 2"));
}

#[test]
fn test_step_navigation() {
    let steps = vec![("Shipping", true), ("Payment", true), ("Review", false)];
    let component = html! {
        <nav aria-label="Checkout progress">
            <ol>
                @for (step, completed) in &steps {
                    <li data-completed={completed.to_string()}>{step}</li>
                }
            </ol>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Shipping") && html.contains("Review"));
}

#[test]
fn test_table_of_contents() {
    let sections = vec!["Introduction", "Getting Started", "API Reference", "FAQ"];
    let component = html! {
        <nav aria-label="Table of contents">
            <ul>
                @for section in &sections {
                    <li><a href={format!("#{}", section.to_lowercase().replace(' ', "-"))}>{section}</a></li>
                }
            </ul>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Introduction") && html.contains("API Reference"));
}

#[test]
fn test_action_bar() {
    let component = html! {
        <div role="toolbar" aria-label="Actions">
            <button>"Save"</button>
            <button>"Cancel"</button>
            <button>"Delete"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"toolbar\"") && html.contains("Delete"));
}

#[test]
fn test_filter_chips() {
    let filters = vec!["All", "Active", "Completed", "Archived"];
    let component = html! {
        <div role="group" aria-label="Filter by status">
            @for filter in &filters {
                <button>{filter}</button>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("All") && html.contains("Archived"));
}
