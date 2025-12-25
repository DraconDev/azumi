//! HTML5 Element Tests
//!
//! Tests for every standard HTML5 element
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Document Metadata Elements (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_html_element() {
    let component = html! { <html lang="en">"Content"</html> };
    let html = test::render(&component);
    assert!(html.contains("<html") && html.contains("lang="));
}

#[test]
fn test_head_element() {
    let component = html! { <head><title>"Test"</title></head> };
    let html = test::render(&component);
    assert!(html.contains("<head>") && html.contains("</head>"));
}

#[test]
fn test_title_element() {
    let component = html! { <title>"Page Title"</title> };
    let html = test::render(&component);
    assert!(html.contains("<title>") && html.contains("Page Title"));
}

#[test]
fn test_base_element() {
    let component = html! { <base href="https://example.com/" /> };
    let html = test::render(&component);
    assert!(html.contains("<base") && html.contains("href="));
}

#[test]
fn test_link_element() {
    let component = html! { <link rel="stylesheet" href="https://cdn.example.com/style.css" /> };
    let html = test::render(&component);
    assert!(html.contains("<link") && html.contains("rel="));
}

#[test]
fn test_meta_name() {
    let component = html! { <meta name="description" content="A test page" /> };
    let html = test::render(&component);
    assert!(html.contains("<meta") && html.contains("description"));
}

#[test]
fn test_meta_charset() {
    let component = html! { <meta charset="utf-8" /> };
    let html = test::render(&component);
    assert!(html.contains("charset="));
}

#[test]
fn test_meta_viewport() {
    let component = html! { <meta name="viewport" content="width=device-width" /> };
    let html = test::render(&component);
    assert!(html.contains("viewport"));
}

#[test]
fn test_body_element() {
    let component = html! { <body>"Main content"</body> };
    let html = test::render(&component);
    assert!(html.contains("<body>") && html.contains("</body>"));
}

#[test]
fn test_style_element() {
    let component = html! {
        <div>"Content"</div>
        <style>
            div { color: "red"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("<style>") || html.contains("color"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Section Elements (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_article_element() {
    let component = html! { <article>"Article content"</article> };
    let html = test::render(&component);
    assert!(html.contains("<article>") && html.contains("</article>"));
}

#[test]
fn test_section_element() {
    let component = html! { <section>"Section content"</section> };
    let html = test::render(&component);
    assert!(html.contains("<section>") && html.contains("</section>"));
}

#[test]
fn test_nav_element() {
    let component = html! { <nav>"Navigation"</nav> };
    let html = test::render(&component);
    assert!(html.contains("<nav>") && html.contains("</nav>"));
}

#[test]
fn test_aside_element() {
    let component = html! { <aside>"Sidebar"</aside> };
    let html = test::render(&component);
    assert!(html.contains("<aside>") && html.contains("</aside>"));
}

#[test]
fn test_header_element() {
    let component = html! { <header>"Header"</header> };
    let html = test::render(&component);
    assert!(html.contains("<header>") && html.contains("</header>"));
}

#[test]
fn test_footer_element() {
    let component = html! { <footer>"Footer"</footer> };
    let html = test::render(&component);
    assert!(html.contains("<footer>") && html.contains("</footer>"));
}

#[test]
fn test_main_element() {
    let component = html! { <main>"Main"</main> };
    let html = test::render(&component);
    assert!(html.contains("<main>") && html.contains("</main>"));
}

#[test]
fn test_h1_element() {
    let component = html! { <h1>"Heading 1"</h1> };
    let html = test::render(&component);
    assert!(html.contains("<h1>") && html.contains("Heading 1"));
}

#[test]
fn test_h2_element() {
    let component = html! { <h2>"Heading 2"</h2> };
    let html = test::render(&component);
    assert!(html.contains("<h2>") && html.contains("Heading 2"));
}

#[test]
fn test_h3_element() {
    let component = html! { <h3>"Heading 3"</h3> };
    let html = test::render(&component);
    assert!(html.contains("<h3>") && html.contains("Heading 3"));
}

#[test]
fn test_h4_element() {
    let component = html! { <h4>"Heading 4"</h4> };
    let html = test::render(&component);
    assert!(html.contains("<h4>") && html.contains("Heading 4"));
}

#[test]
fn test_h5_element() {
    let component = html! { <h5>"Heading 5"</h5> };
    let html = test::render(&component);
    assert!(html.contains("<h5>") && html.contains("Heading 5"));
}

#[test]
fn test_h6_element() {
    let component = html! { <h6>"Heading 6"</h6> };
    let html = test::render(&component);
    assert!(html.contains("<h6>") && html.contains("Heading 6"));
}

#[test]
fn test_address_element() {
    let component = html! { <address>"Contact info"</address> };
    let html = test::render(&component);
    assert!(html.contains("<address>") && html.contains("</address>"));
}

#[test]
fn test_hgroup_element() {
    let component = html! { <hgroup><h1>"Title"</h1><p>"Subtitle"</p></hgroup> };
    let html = test::render(&component);
    assert!(html.contains("<hgroup>") && html.contains("</hgroup>"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Text Content Elements (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_p_element() {
    let component = html! { <p>"Paragraph"</p> };
    let html = test::render(&component);
    assert!(html.contains("<p>") && html.contains("Paragraph"));
}

#[test]
fn test_div_element() {
    let component = html! { <div>"Division"</div> };
    let html = test::render(&component);
    assert!(html.contains("<div>") && html.contains("Division"));
}

#[test]
fn test_pre_element() {
    let component = html! { <pre>"Preformatted"</pre> };
    let html = test::render(&component);
    assert!(html.contains("<pre>") && html.contains("Preformatted"));
}

#[test]
fn test_blockquote_element() {
    let component = html! { <blockquote>"Quote"</blockquote> };
    let html = test::render(&component);
    assert!(html.contains("<blockquote>") && html.contains("</blockquote>"));
}

#[test]
fn test_ol_element() {
    let component = html! { <ol><li>"Item"</li></ol> };
    let html = test::render(&component);
    assert!(html.contains("<ol>") && html.contains("</ol>"));
}

#[test]
fn test_ul_element() {
    let component = html! { <ul><li>"Item"</li></ul> };
    let html = test::render(&component);
    assert!(html.contains("<ul>") && html.contains("</ul>"));
}

#[test]
fn test_li_element() {
    let component = html! { <li>"List item"</li> };
    let html = test::render(&component);
    assert!(html.contains("<li>") && html.contains("List item"));
}

#[test]
fn test_dl_element() {
    let component = html! { <dl><dt>"Term"</dt><dd>"Definition"</dd></dl> };
    let html = test::render(&component);
    assert!(html.contains("<dl>") && html.contains("</dl>"));
}

#[test]
fn test_dt_element() {
    let component = html! { <dt>"Term"</dt> };
    let html = test::render(&component);
    assert!(html.contains("<dt>") && html.contains("Term"));
}

#[test]
fn test_dd_element() {
    let component = html! { <dd>"Definition"</dd> };
    let html = test::render(&component);
    assert!(html.contains("<dd>") && html.contains("Definition"));
}

#[test]
fn test_figure_element() {
    let component = html! { <figure><img src="test.jpg" alt="test" /><figcaption>"Caption"</figcaption></figure> };
    let html = test::render(&component);
    assert!(html.contains("<figure>") && html.contains("</figure>"));
}

#[test]
fn test_figcaption_element() {
    let component = html! { <figcaption>"Caption"</figcaption> };
    let html = test::render(&component);
    assert!(html.contains("<figcaption>") && html.contains("Caption"));
}

#[test]
fn test_hr_element() {
    let component = html! { <hr /> };
    let html = test::render(&component);
    assert!(html.contains("<hr"));
}

#[test]
fn test_menu_element() {
    let component = html! { <menu><li>"Option"</li></menu> };
    let html = test::render(&component);
    assert!(html.contains("<menu>") && html.contains("</menu>"));
}

#[test]
fn test_search_element() {
    let component = html! { <search>"Search area"</search> };
    let html = test::render(&component);
    assert!(html.contains("<search>") && html.contains("</search>"));
}

#[test]
fn test_ol_with_type() {
    let component = html! { <ol type="A"><li>"A"</li><li>"B"</li></ol> };
    let html = test::render(&component);
    assert!(html.contains("type="));
}

#[test]
fn test_ol_with_start() {
    let component = html! { <ol start="5"><li>"Five"</li></ol> };
    let html = test::render(&component);
    assert!(html.contains("start="));
}

#[test]
fn test_li_with_value() {
    let component = html! { <li value="10">"Ten"</li> };
    let html = test::render(&component);
    assert!(html.contains("value="));
}

#[test]
fn test_blockquote_with_cite() {
    let component = html! { <blockquote cite="https://example.com">"Quote"</blockquote> };
    let html = test::render(&component);
    assert!(html.contains("cite="));
}

#[test]
fn test_pre_with_code() {
    let component = html! { <pre><code>"let x = 1;"</code></pre> };
    let html = test::render(&component);
    assert!(html.contains("<pre>") && html.contains("<code>"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Inline Text Elements (25 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_a_element() {
    let component = html! { <a href="https://example.com">"Link"</a> };
    let html = test::render(&component);
    assert!(html.contains("<a ") && html.contains("href="));
}

#[test]
fn test_a_with_target() {
    let component = html! { <a href="https://example.com" target="_blank">"External"</a> };
    let html = test::render(&component);
    assert!(html.contains("target="));
}

#[test]
fn test_span_element() {
    let component = html! { <span>"Inline"</span> };
    let html = test::render(&component);
    assert!(html.contains("<span>") && html.contains("Inline"));
}

#[test]
fn test_strong_element() {
    let component = html! { <strong>"Bold"</strong> };
    let html = test::render(&component);
    assert!(html.contains("<strong>") && html.contains("Bold"));
}

#[test]
fn test_em_element() {
    let component = html! { <em>"Emphasis"</em> };
    let html = test::render(&component);
    assert!(html.contains("<em>") && html.contains("Emphasis"));
}

#[test]
fn test_b_element() {
    let component = html! { <b>"Bold"</b> };
    let html = test::render(&component);
    assert!(html.contains("<b>") && html.contains("Bold"));
}

#[test]
fn test_i_element() {
    let component = html! { <i>"Italic"</i> };
    let html = test::render(&component);
    assert!(html.contains("<i>") && html.contains("Italic"));
}

#[test]
fn test_u_element() {
    let component = html! { <u>"Underline"</u> };
    let html = test::render(&component);
    assert!(html.contains("<u>") && html.contains("Underline"));
}

#[test]
fn test_s_element() {
    let component = html! { <s>"Strikethrough"</s> };
    let html = test::render(&component);
    assert!(html.contains("<s>") && html.contains("Strikethrough"));
}

#[test]
fn test_mark_element() {
    let component = html! { <mark>"Highlighted"</mark> };
    let html = test::render(&component);
    assert!(html.contains("<mark>") && html.contains("Highlighted"));
}

#[test]
fn test_small_element() {
    let component = html! { <small>"Small text"</small> };
    let html = test::render(&component);
    assert!(html.contains("<small>") && html.contains("Small"));
}

#[test]
fn test_sub_element() {
    let component = html! { <sub>"subscript"</sub> };
    let html = test::render(&component);
    assert!(html.contains("<sub>") && html.contains("subscript"));
}

#[test]
fn test_sup_element() {
    let component = html! { <sup>"superscript"</sup> };
    let html = test::render(&component);
    assert!(html.contains("<sup>") && html.contains("superscript"));
}

#[test]
fn test_code_element() {
    let component = html! { <code>"console.log()"</code> };
    let html = test::render(&component);
    assert!(html.contains("<code>") && html.contains("console"));
}

#[test]
fn test_kbd_element() {
    let component = html! { <kbd>"Ctrl+C"</kbd> };
    let html = test::render(&component);
    assert!(html.contains("<kbd>") && html.contains("Ctrl"));
}

#[test]
fn test_samp_element() {
    let component = html! { <samp>"Output"</samp> };
    let html = test::render(&component);
    assert!(html.contains("<samp>") && html.contains("Output"));
}

#[test]
fn test_var_element() {
    let component = html! { <var>"x"</var> };
    let html = test::render(&component);
    assert!(html.contains("<var>") && html.contains("x"));
}

#[test]
fn test_abbr_element() {
    let component = html! { <abbr title="HyperText Markup Language">"HTML"</abbr> };
    let html = test::render(&component);
    assert!(html.contains("<abbr") && html.contains("HTML"));
}

#[test]
fn test_cite_element() {
    let component = html! { <cite>"Book Title"</cite> };
    let html = test::render(&component);
    assert!(html.contains("<cite>") && html.contains("Book"));
}

#[test]
fn test_q_element() {
    let component = html! { <q>"Quote"</q> };
    let html = test::render(&component);
    assert!(html.contains("<q>") && html.contains("Quote"));
}

#[test]
fn test_dfn_element() {
    let component = html! { <dfn>"Definition"</dfn> };
    let html = test::render(&component);
    assert!(html.contains("<dfn>") && html.contains("Definition"));
}

#[test]
fn test_time_element() {
    let component = html! { <time datetime="2024-01-01">"New Year"</time> };
    let html = test::render(&component);
    assert!(html.contains("<time") && html.contains("datetime="));
}

#[test]
fn test_data_element() {
    let component = html! { <data value="42">"Forty-two"</data> };
    let html = test::render(&component);
    assert!(html.contains("<data") && html.contains("value="));
}

#[test]
fn test_br_element() {
    let component = html! { <br /> };
    let html = test::render(&component);
    assert!(html.contains("<br"));
}

#[test]
fn test_wbr_element() {
    let component = html! { <wbr /> };
    let html = test::render(&component);
    assert!(html.contains("<wbr"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 5: Form Elements (25 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_form_element() {
    let component = html! { <form action="/submit" method="post">"Form"</form> };
    let html = test::render(&component);
    assert!(html.contains("<form") && html.contains("action="));
}

#[test]
fn test_input_text() {
    let component = html! { <input type="text" name="username" /> };
    let html = test::render(&component);
    assert!(html.contains("<input") && html.contains("type="));
}

#[test]
fn test_input_password() {
    let component = html! { <input type="password" name="pass" /> };
    let html = test::render(&component);
    assert!(html.contains("password"));
}

#[test]
fn test_input_email() {
    let component = html! { <input type="email" name="email" /> };
    let html = test::render(&component);
    assert!(html.contains("email"));
}

#[test]
fn test_input_number() {
    let component = html! { <input type="number" name="age" /> };
    let html = test::render(&component);
    assert!(html.contains("number"));
}

#[test]
fn test_input_checkbox() {
    let component = html! { <input type="checkbox" name="agree" /> };
    let html = test::render(&component);
    assert!(html.contains("checkbox"));
}

#[test]
fn test_input_radio() {
    let component = html! { <input type="radio" name="option" value="a" /> };
    let html = test::render(&component);
    assert!(html.contains("radio"));
}

#[test]
fn test_input_file() {
    let component = html! { <input type="file" name="upload" /> };
    let html = test::render(&component);
    assert!(html.contains("file"));
}

#[test]
fn test_input_hidden() {
    let component = html! { <input type="hidden" name="token" value="abc" /> };
    let html = test::render(&component);
    assert!(html.contains("hidden"));
}

#[test]
fn test_input_submit() {
    let component = html! { <input type="submit" value="Submit" /> };
    let html = test::render(&component);
    assert!(html.contains("submit"));
}

#[test]
fn test_textarea_element() {
    let component = html! { <textarea name="message">"Default text"</textarea> };
    let html = test::render(&component);
    assert!(html.contains("<textarea") && html.contains("</textarea>"));
}

#[test]
fn test_select_element() {
    let component = html! {
        <select name="country">
            <option value="us">"USA"</option>
            <option value="uk">"UK"</option>
        </select>
    };
    let html = test::render(&component);
    assert!(html.contains("<select") && html.contains("<option"));
}

#[test]
fn test_option_element() {
    let component = html! { <option value="test">"Test"</option> };
    let html = test::render(&component);
    assert!(html.contains("<option") && html.contains("value="));
}

#[test]
fn test_optgroup_element() {
    let component = html! {
        <optgroup label="Group">
            <option value="1">"One"</option>
        </optgroup>
    };
    let html = test::render(&component);
    assert!(html.contains("<optgroup") && html.contains("label="));
}

#[test]
fn test_button_element() {
    let component = html! { <button type="button">"Click"</button> };
    let html = test::render(&component);
    assert!(html.contains("<button") && html.contains("Click"));
}

#[test]
fn test_button_submit() {
    let component = html! { <button type="submit">"Submit"</button> };
    let html = test::render(&component);
    assert!(html.contains("submit"));
}

#[test]
fn test_label_element() {
    let component = html! { <label for="input1">"Label"</label> };
    let html = test::render(&component);
    assert!(html.contains("<label") && html.contains("for="));
}

#[test]
fn test_fieldset_element() {
    let component = html! { <fieldset><legend>"Group"</legend></fieldset> };
    let html = test::render(&component);
    assert!(html.contains("<fieldset>") && html.contains("</fieldset>"));
}

#[test]
fn test_legend_element() {
    let component = html! { <legend>"Legend"</legend> };
    let html = test::render(&component);
    assert!(html.contains("<legend>") && html.contains("Legend"));
}

#[test]
fn test_datalist_element() {
    let component = html! {
        <datalist>
            <option value="opt1" />
        </datalist>
    };
    let html = test::render(&component);
    assert!(html.contains("<datalist"));
}

#[test]
fn test_output_element() {
    let component = html! { <output name="result">"0"</output> };
    let html = test::render(&component);
    assert!(html.contains("<output") && html.contains("</output>"));
}

#[test]
fn test_progress_element() {
    let component = html! { <progress value="50" max="100">"50%"</progress> };
    let html = test::render(&component);
    assert!(html.contains("<progress") && html.contains("value="));
}

#[test]
fn test_meter_element() {
    let component = html! { <meter value="0.6">"60%"</meter> };
    let html = test::render(&component);
    assert!(html.contains("<meter") && html.contains("value="));
}

#[test]
fn test_input_date() {
    let component = html! { <input type="date" name="birthdate" /> };
    let html = test::render(&component);
    assert!(html.contains("date"));
}

#[test]
fn test_input_color() {
    let component = html! { <input type="color" name="favorite" /> };
    let html = test::render(&component);
    assert!(html.contains("color"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 6: Table Elements (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_table_element() {
    let component = html! { <table><tr><td>"Cell"</td></tr></table> };
    let html = test::render(&component);
    assert!(html.contains("<table>") && html.contains("</table>"));
}

#[test]
fn test_thead_element() {
    let component = html! { <thead><tr><th>"Header"</th></tr></thead> };
    let html = test::render(&component);
    assert!(html.contains("<thead>") && html.contains("</thead>"));
}

#[test]
fn test_tbody_element() {
    let component = html! { <tbody><tr><td>"Body"</td></tr></tbody> };
    let html = test::render(&component);
    assert!(html.contains("<tbody>") && html.contains("</tbody>"));
}

#[test]
fn test_tfoot_element() {
    let component = html! { <tfoot><tr><td>"Footer"</td></tr></tfoot> };
    let html = test::render(&component);
    assert!(html.contains("<tfoot>") && html.contains("</tfoot>"));
}

#[test]
fn test_tr_element() {
    let component = html! { <tr><td>"Cell"</td></tr> };
    let html = test::render(&component);
    assert!(html.contains("<tr>") && html.contains("</tr>"));
}

#[test]
fn test_th_element() {
    let component = html! { <th>"Header Cell"</th> };
    let html = test::render(&component);
    assert!(html.contains("<th>") && html.contains("Header"));
}

#[test]
fn test_td_element() {
    let component = html! { <td>"Data Cell"</td> };
    let html = test::render(&component);
    assert!(html.contains("<td>") && html.contains("Data"));
}

#[test]
fn test_caption_element() {
    let component = html! { <caption>"Table Caption"</caption> };
    let html = test::render(&component);
    assert!(html.contains("<caption>") && html.contains("Caption"));
}

#[test]
fn test_colgroup_element() {
    let component = html! { <colgroup><col /></colgroup> };
    let html = test::render(&component);
    assert!(html.contains("<colgroup>") && html.contains("</colgroup>"));
}

#[test]
fn test_col_element() {
    let component = html! { <col span="2" /> };
    let html = test::render(&component);
    assert!(html.contains("<col") && html.contains("span="));
}

#[test]
fn test_th_scope() {
    let component = html! { <th scope="col">"Header"</th> };
    let html = test::render(&component);
    assert!(html.contains("scope="));
}

#[test]
fn test_td_colspan() {
    let component = html! { <td colspan="2">"Spanning"</td> };
    let html = test::render(&component);
    assert!(html.contains("colspan="));
}

#[test]
fn test_td_rowspan() {
    let component = html! { <td rowspan="3">"Spanning"</td> };
    let html = test::render(&component);
    assert!(html.contains("rowspan="));
}

#[test]
fn test_complete_table() {
    let component = html! {
        <table>
            <caption>"Users"</caption>
            <thead>
                <tr><th>"Name"</th><th>"Age"</th></tr>
            </thead>
            <tbody>
                <tr><td>"Alice"</td><td>"30"</td></tr>
            </tbody>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice") && html.contains("Name"));
}

#[test]
fn test_table_with_loop() {
    let users = vec![("Alice", 30), ("Bob", 25)];
    let component = html! {
        <table>
            <tbody>
                @for (name, age) in &users {
                    <tr><td>{name}</td><td>{age}</td></tr>
                }
            </tbody>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice") && html.contains("Bob"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 7: Media Elements (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_img_element() {
    let component = html! { <img src="/test.jpg" alt="Test image" /> };
    let html = test::render(&component);
    assert!(html.contains("<img") && html.contains("alt="));
}

#[test]
fn test_audio_element() {
    let component = html! { <audio src="/audio.mp3" controls="true">"Audio"</audio> };
    let html = test::render(&component);
    assert!(html.contains("<audio") && html.contains("controls"));
}

#[test]
fn test_video_element() {
    let component = html! { <video src="/video.mp4" controls="true">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("<video") && html.contains("controls"));
}

#[test]
fn test_source_element() {
    let component = html! { <source src="/video.webm" type="video/webm" /> };
    let html = test::render(&component);
    assert!(html.contains("<source") && html.contains("type="));
}

#[test]
fn test_track_element() {
    let component = html! { <track src="/captions.vtt" kind="subtitles" /> };
    let html = test::render(&component);
    assert!(html.contains("<track") && html.contains("kind="));
}

#[test]
fn test_picture_element() {
    let component = html! {
        <picture>
            <source srcset="/large.jpg" media="(min-width: 800px)" />
            <img src="/small.jpg" alt="Responsive" />
        </picture>
    };
    let html = test::render(&component);
    assert!(html.contains("<picture>") && html.contains("</picture>"));
}

#[test]
fn test_iframe_element() {
    let component = html! { <iframe src="https://example.com" title="Embed"></iframe> };
    let html = test::render(&component);
    assert!(html.contains("<iframe") && html.contains("title="));
}

#[test]
fn test_embed_element() {
    let component = html! { <embed src="/plugin.swf" type="application/x-shockwave-flash" /> };
    let html = test::render(&component);
    assert!(html.contains("<embed") && html.contains("type="));
}

#[test]
fn test_object_element() {
    let component =
        html! { <object data-src="/file.pdf" type="application/pdf">"Fallback"</object> };
    let html = test::render(&component);
    assert!(html.contains("<object") && html.contains("data-src="));
}

#[test]
fn test_canvas_element() {
    let component = html! { <canvas width="300" height="200">"Canvas not supported"</canvas> };
    let html = test::render(&component);
    assert!(html.contains("<canvas") && html.contains("width="));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 8: Interactive Elements (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_details_element() {
    let component = html! { <details><summary>"Click"</summary>"Hidden"</details> };
    let html = test::render(&component);
    assert!(html.contains("<details>") && html.contains("</details>"));
}

#[test]
fn test_summary_element() {
    let component = html! { <summary>"Toggle"</summary> };
    let html = test::render(&component);
    assert!(html.contains("<summary>") && html.contains("Toggle"));
}

#[test]
fn test_dialog_element() {
    let component = html! { <dialog open="true">"Modal content"</dialog> };
    let html = test::render(&component);
    assert!(html.contains("<dialog") && html.contains("open"));
}

#[test]
fn test_template_element() {
    let component = html! { <template>"Template content"</template> };
    let html = test::render(&component);
    assert!(html.contains("<template>") && html.contains("</template>"));
}

#[test]
fn test_slot_element() {
    let component = html! { <slot name="header">"Default"</slot> };
    let html = test::render(&component);
    assert!(html.contains("<slot") && html.contains("name="));
}

#[test]
fn test_noscript_element() {
    let component = html! { <noscript>"Enable JavaScript"</noscript> };
    let html = test::render(&component);
    assert!(html.contains("<noscript>") && html.contains("</noscript>"));
}

#[test]
fn test_script_element() {
    let component = html! { <script src="/app.js"></script> };
    let html = test::render(&component);
    assert!(html.contains("<script") && html.contains("</script>"));
}

#[test]
fn test_details_open() {
    let component = html! { <details open="true"><summary>"Open"</summary>"Content"</details> };
    let html = test::render(&component);
    assert!(html.contains("open"));
}

#[test]
fn test_multiple_details() {
    let items = vec!["One", "Two", "Three"];
    let component = html! {
        <div>
            @for item in &items {
                <details>
                    <summary>{item}</summary>
                    <p>"Content for "{item}</p>
                </details>
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("One") && html.contains("Two") && html.contains("Three"));
}

#[test]
fn test_portal_pattern() {
    let component = html! {
        <div>
            <div data-portal="true">"Portal target"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("data-portal"));
}
