//! Semantic HTML Tests
//!
//! Tests for proper semantic HTML structure usage
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Sectioning Elements (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_article_element() {
    let component = html! {
        <article>
            <h2>"Article Title"</h2>
            <p>"Article content"</p>
        </article>
    };
    let html = test::render(&component);
    assert!(html.contains("<article>") && html.contains("</article>"));
}

#[test]
fn test_section_element() {
    let component = html! {
        <section>
            <h2>"Section Heading"</h2>
            <p>"Section content"</p>
        </section>
    };
    let html = test::render(&component);
    assert!(html.contains("<section>") && html.contains("</section>"));
}

#[test]
fn test_nav_element() {
    let component = html! {
        <nav>
            <ul>
                <li><a href="/">"Home"</a></li>
                <li><a href="/about">"About"</a></li>
            </ul>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("<nav>") && html.contains("</nav>"));
}

#[test]
fn test_aside_element() {
    let component = html! {
        <aside>
            <h3>"Related"</h3>
            <p>"Sidebar content"</p>
        </aside>
    };
    let html = test::render(&component);
    assert!(html.contains("<aside>") && html.contains("</aside>"));
}

#[test]
fn test_header_element() {
    let component = html! {
        <header>
            <h1>"Site Title"</h1>
            <nav>"Navigation"</nav>
        </header>
    };
    let html = test::render(&component);
    assert!(html.contains("<header>") && html.contains("</header>"));
}

#[test]
fn test_footer_element() {
    let component = html! {
        <footer>
            <p>"Copyright 2024"</p>
        </footer>
    };
    let html = test::render(&component);
    assert!(html.contains("<footer>") && html.contains("</footer>"));
}

#[test]
fn test_main_element() {
    let component = html! {
        <main>
            <h1>"Main Content"</h1>
            <p>"Body"</p>
        </main>
    };
    let html = test::render(&component);
    assert!(html.contains("<main>") && html.contains("</main>"));
}

#[test]
fn test_address_element() {
    let component = html! {
        <address>
            <p>"123 Main St"</p>
            <p>"City, ST 12345"</p>
        </address>
    };
    let html = test::render(&component);
    assert!(html.contains("<address>") && html.contains("</address>"));
}

#[test]
fn test_hgroup_concept() {
    let component = html! {
        <div>
            <h1>"Main Title"</h1>
            <p>"Subtitle"</p>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Main Title") && html.contains("Subtitle"));
}

#[test]
fn test_nested_sections() {
    let component = html! {
        <article>
            <section>
                <h2>"Part 1"</h2>
            </section>
            <section>
                <h2>"Part 2"</h2>
            </section>
        </article>
    };
    let html = test::render(&component);
    assert!(html.contains("Part 1") && html.contains("Part 2"));
}

#[test]
fn test_heading_h1() {
    let component = html! { <h1>"Heading 1"</h1> };
    let html = test::render(&component);
    assert!(html.contains("<h1>") && html.contains("</h1>"));
}

#[test]
fn test_heading_h2() {
    let component = html! { <h2>"Heading 2"</h2> };
    let html = test::render(&component);
    assert!(html.contains("<h2>") && html.contains("</h2>"));
}

#[test]
fn test_heading_h3() {
    let component = html! { <h3>"Heading 3"</h3> };
    let html = test::render(&component);
    assert!(html.contains("<h3>") && html.contains("</h3>"));
}

#[test]
fn test_heading_h4() {
    let component = html! { <h4>"Heading 4"</h4> };
    let html = test::render(&component);
    assert!(html.contains("<h4>") && html.contains("</h4>"));
}

#[test]
fn test_heading_h5() {
    let component = html! { <h5>"Heading 5"</h5> };
    let html = test::render(&component);
    assert!(html.contains("<h5>") && html.contains("</h5>"));
}

#[test]
fn test_heading_h6() {
    let component = html! { <h6>"Heading 6"</h6> };
    let html = test::render(&component);
    assert!(html.contains("<h6>") && html.contains("</h6>"));
}

#[test]
fn test_heading_hierarchy() {
    let component = html! {
        <div>
            <h1>"Title"</h1>
            <h2>"Subtitle"</h2>
            <h3>"Section"</h3>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("<h1>") && html.contains("<h2>") && html.contains("<h3>"));
}

#[test]
fn test_paragraph() {
    let component = html! { <p>"Paragraph text"</p> };
    let html = test::render(&component);
    assert!(html.contains("<p>") && html.contains("</p>"));
}

#[test]
fn test_blockquote() {
    let component = html! { <blockquote cite="source">"Quoted text"</blockquote> };
    let html = test::render(&component);
    assert!(html.contains("<blockquote") && html.contains("</blockquote>"));
}

#[test]
fn test_figure_figcaption() {
    let component = html! {
        <figure>
            <img src="/img.jpg" alt="Image" />
            <figcaption>"Image caption"</figcaption>
        </figure>
    };
    let html = test::render(&component);
    assert!(html.contains("<figure>") && html.contains("<figcaption>"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Text Semantics (25 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_strong() {
    let component = html! { <p><strong>"Important"</strong></p> };
    let html = test::render(&component);
    assert!(html.contains("<strong>"));
}

#[test]
fn test_em() {
    let component = html! { <p><em>"Emphasized"</em></p> };
    let html = test::render(&component);
    assert!(html.contains("<em>"));
}

#[test]
fn test_mark() {
    let component = html! { <p><mark>"Highlighted"</mark></p> };
    let html = test::render(&component);
    assert!(html.contains("<mark>"));
}

#[test]
fn test_small() {
    let component = html! { <p><small>"Fine print"</small></p> };
    let html = test::render(&component);
    assert!(html.contains("<small>"));
}

#[test]
fn test_del() {
    let component = html! { <p><del>"Deleted"</del></p> };
    let html = test::render(&component);
    assert!(html.contains("<del>"));
}

#[test]
fn test_ins() {
    let component = html! { <p><ins>"Inserted"</ins></p> };
    let html = test::render(&component);
    assert!(html.contains("<ins>"));
}

#[test]
fn test_sub() {
    let component = html! { <p>"H"<sub>"2"</sub>"O"</p> };
    let html = test::render(&component);
    assert!(html.contains("<sub>"));
}

#[test]
fn test_sup() {
    let component = html! { <p>"x"<sup>"2"</sup></p> };
    let html = test::render(&component);
    assert!(html.contains("<sup>"));
}

#[test]
fn test_code() {
    let component = html! { <p><code>"console.log()"</code></p> };
    let html = test::render(&component);
    assert!(html.contains("<code>"));
}

#[test]
fn test_kbd() {
    let component = html! { <p><kbd>"Ctrl+C"</kbd></p> };
    let html = test::render(&component);
    assert!(html.contains("<kbd>"));
}

#[test]
fn test_samp() {
    let component = html! { <p><samp>"Output"</samp></p> };
    let html = test::render(&component);
    assert!(html.contains("<samp>"));
}

#[test]
fn test_var() {
    let component = html! { <p><var>"x"</var>" = 5"</p> };
    let html = test::render(&component);
    assert!(html.contains("<var>"));
}

#[test]
fn test_pre() {
    let component = html! { <pre>"Preformatted\n  text"</pre> };
    let html = test::render(&component);
    assert!(html.contains("<pre>"));
}

#[test]
fn test_abbr() {
    let component = html! { <abbr title="HyperText Markup Language">"HTML"</abbr> };
    let html = test::render(&component);
    assert!(html.contains("<abbr") && html.contains("title="));
}

#[test]
fn test_cite() {
    let component = html! { <p><cite>"The Great Gatsby"</cite></p> };
    let html = test::render(&component);
    assert!(html.contains("<cite>"));
}

#[test]
fn test_dfn() {
    let component = html! { <p><dfn>"Term"</dfn>" is defined as..."</p> };
    let html = test::render(&component);
    assert!(html.contains("<dfn>"));
}

#[test]
fn test_time() {
    let component = html! { <time datetime="2024-01-01">"Jan 1, 2024"</time> };
    let html = test::render(&component);
    assert!(html.contains("<time") && html.contains("datetime="));
}

#[test]
fn test_data() {
    let component = html! { <data value="123">"Product"</data> };
    let html = test::render(&component);
    assert!(html.contains("<data") && html.contains("value="));
}

#[test]
fn test_q() {
    let component = html! { <p><q>"Inline quote"</q></p> };
    let html = test::render(&component);
    assert!(html.contains("<q>"));
}

#[test]
fn test_s() {
    let component = html! { <p><s>"Strikethrough"</s></p> };
    let html = test::render(&component);
    assert!(html.contains("<s>"));
}

#[test]
fn test_u() {
    let component = html! { <p><u>"Underline"</u></p> };
    let html = test::render(&component);
    assert!(html.contains("<u>"));
}

#[test]
fn test_i() {
    let component = html! { <p><i>"Italic"</i></p> };
    let html = test::render(&component);
    assert!(html.contains("<i>"));
}

#[test]
fn test_b() {
    let component = html! { <p><b>"Bold"</b></p> };
    let html = test::render(&component);
    assert!(html.contains("<b>"));
}

#[test]
fn test_br() {
    let component = html! { <p>"Line 1"<br />"Line 2"</p> };
    let html = test::render(&component);
    assert!(html.contains("<br"));
}

#[test]
fn test_wbr() {
    let component = html! { <p>"supercali"<wbr />"fragilistic"</p> };
    let html = test::render(&component);
    assert!(html.contains("<wbr"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Lists (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ul() {
    let component = html! {
        <ul>
            <li>"Item 1"</li>
            <li>"Item 2"</li>
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("<ul>") && html.contains("<li>"));
}

#[test]
fn test_ol() {
    let component = html! {
        <ol>
            <li>"First"</li>
            <li>"Second"</li>
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("<ol>") && html.contains("<li>"));
}

#[test]
fn test_ol_start() {
    let component = html! {
        <ol start="5">
            <li>"Fifth"</li>
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("start=\"5\""));
}

#[test]
fn test_ol_reversed() {
    let component = html! {
        <ol reversed="true">
            <li>"Third"</li>
            <li>"Second"</li>
            <li>"First"</li>
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("reversed"));
}

#[test]
fn test_ol_type_a() {
    let component = html! {
        <ol type="a">
            <li>"Item"</li>
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("type=\"a\""));
}

#[test]
fn test_nested_ul() {
    let component = html! {
        <ul>
            <li>"Item"
                <ul>
                    <li>"Nested"</li>
                </ul>
            </li>
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("Nested"));
}

#[test]
fn test_dl() {
    let component = html! {
        <dl>
            <dt>"Term"</dt>
            <dd>"Definition"</dd>
        </dl>
    };
    let html = test::render(&component);
    assert!(html.contains("<dl>") && html.contains("<dt>") && html.contains("<dd>"));
}

#[test]
fn test_dl_multiple() {
    let terms = vec![("HTML", "Markup language"), ("CSS", "Styling language")];
    let component = html! {
        <dl>
            @for (term, def) in &terms {
                <dt>{term}</dt>
                <dd>{def}</dd>
            }
        </dl>
    };
    let html = test::render(&component);
    assert!(html.contains("HTML") && html.contains("CSS"));
}

#[test]
fn test_menu() {
    let component = html! {
        <menu>
            <li><button>"Action"</button></li>
        </menu>
    };
    let html = test::render(&component);
    assert!(html.contains("<menu>"));
}

#[test]
fn test_li_value() {
    let component = html! {
        <ol>
            <li value="10">"Ten"</li>
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("value=\"10\""));
}

#[test]
fn test_dynamic_list() {
    let items = vec!["Apple", "Banana", "Cherry"];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("Apple") && html.contains("Banana") && html.contains("Cherry"));
}

#[test]
fn test_numbered_list() {
    let items = vec!["First", "Second", "Third"];
    let component = html! {
        <ol>
            @for item in &items {
                <li>{item}</li>
            }
        </ol>
    };
    let html = test::render(&component);
    assert!(html.contains("First") && html.contains("Third"));
}

#[test]
fn test_empty_list() {
    let items: Vec<&str> = vec![];
    let component = html! {
        <ul>
            @for item in &items {
                <li>{item}</li>
            }
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("<ul>") && !html.contains("<li>"));
}

#[test]
fn test_list_with_links() {
    let component = html! {
        <ul>
            <li><a href="/">"Home"</a></li>
            <li><a href="/about">"About"</a></li>
        </ul>
    };
    let html = test::render(&component);
    assert!(html.contains("Home") && html.contains("About"));
}

#[test]
fn test_breadcrumb_list() {
    let items = vec![
        ("Home", "/"),
        ("Products", "/products"),
        ("Widget", "/products/widget"),
    ];
    let component = html! {
        <nav aria-label="breadcrumb">
            <ol>
                @for (label, href) in &items {
                    <li><a href={*href}>{label}</a></li>
                }
            </ol>
        </nav>
    };
    let html = test::render(&component);
    assert!(html.contains("Home") && html.contains("Widget"));
}
