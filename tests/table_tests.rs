//! Table Tests
//!
//! Comprehensive tests for table elements and structure
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Basic Tables (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_basic_table() {
    let component = html! {
        <table>
            <tr>
                <td>"Cell"</td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("<table>") && html.contains("</table>"));
}

#[test]
fn test_table_with_header() {
    let component = html! {
        <table>
            <thead>
                <tr><th>"Name"</th><th>"Age"</th></tr>
            </thead>
            <tbody>
                <tr><td>"Alice"</td><td>"30"</td></tr>
            </tbody>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("<thead>") && html.contains("<tbody>"));
}

#[test]
fn test_table_footer() {
    let component = html! {
        <table>
            <tfoot>
                <tr><td>"Total"</td><td>"100"</td></tr>
            </tfoot>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("<tfoot>"));
}

#[test]
fn test_table_caption() {
    let component = html! {
        <table>
            <caption>"Sales Data"</caption>
            <tr><td>"Data"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("<caption>") && html.contains("Sales Data"));
}

#[test]
fn test_th_scope_col() {
    let component = html! {
        <table>
            <tr><th scope="col">"Column"</th></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("scope=\"col\""));
}

#[test]
fn test_th_scope_row() {
    let component = html! {
        <table>
            <tr><th scope="row">"Row"</th><td>"Data"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("scope=\"row\""));
}

#[test]
fn test_colspan() {
    let component = html! {
        <table>
            <tr><td colspan="2">"Spanning"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("colspan=\"2\""));
}

#[test]
fn test_rowspan() {
    let component = html! {
        <table>
            <tr><td rowspan="2">"Spanning"</td><td>"A"</td></tr>
            <tr><td>"B"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("rowspan=\"2\""));
}

#[test]
fn test_colgroup() {
    let component = html! {
        <table>
            <colgroup>
                <col span="2" />
            </colgroup>
            <tr><td>"A"</td><td>"B"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("<colgroup>") && html.contains("<col"));
}

#[test]
fn test_col_span() {
    let component = html! {
        <table>
            <colgroup>
                <col span="3" />
            </colgroup>
            <tr><td>"1"</td><td>"2"</td><td>"3"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("span=\"3\""));
}

#[test]
fn test_table_multiple_rows() {
    let rows = vec![("Alice", 30), ("Bob", 25), ("Carol", 35)];
    let component = html! {
        <table>
            <thead>
                <tr><th>"Name"</th><th>"Age"</th></tr>
            </thead>
            <tbody>
                @for (name, age) in &rows {
                    <tr><td>{name}</td><td>{age}</td></tr>
                }
            </tbody>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("Alice") && html.contains("Bob") && html.contains("Carol"));
}

#[test]
fn test_table_empty_body() {
    let rows: Vec<(&str, i32)> = vec![];
    let component = html! {
        <table>
            <thead>
                <tr><th>"Name"</th></tr>
            </thead>
            <tbody>
                @for (name, _age) in &rows {
                    <tr><td>{name}</td></tr>
                }
            </tbody>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("<thead>") && html.contains("<tbody>"));
}

#[test]
fn test_data_table_with_index() {
    let items = vec!["A", "B", "C"];
    let component = html! {
        <table>
            <tbody>
                @for (i, item) in items.iter().enumerate() {
                    <tr><td>{i}</td><td>{item}</td></tr>
                }
            </tbody>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("0") && html.contains("2"));
}

#[test]
fn test_table_aria_describedby() {
    let component = html! {
        <table aria-describedby="desc">
            <tr><td>"Data"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("aria-describedby"));
}

#[test]
fn test_table_headers_attr() {
    let header_id = "name";
    let component = html! {
        <table>
            <tr><th>{header_id}</th></tr>
            <tr><td headers="name">"Alice"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("headers="));
}

#[test]
fn test_multiple_tbody() {
    let component = html! {
        <table>
            <tbody>
                <tr><td>"Group 1"</td></tr>
            </tbody>
            <tbody>
                <tr><td>"Group 2"</td></tr>
            </tbody>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("Group 1") && html.contains("Group 2"));
}

#[test]
fn test_complex_table_structure() {
    let component = html! {
        <table>
            <caption>"Quarterly Report"</caption>
            <colgroup>
                <col span="1" />
            </colgroup>
            <thead>
                <tr><th>"Q1"</th><th>"Q2"</th></tr>
            </thead>
            <tbody>
                <tr><td>"100"</td><td>"150"</td></tr>
            </tbody>
            <tfoot>
                <tr><td colspan="2">"Total: 250"</td></tr>
            </tfoot>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("Quarterly Report") && html.contains("Total: 250"));
}

#[test]
fn test_sortable_table_header() {
    let component = html! {
        <table>
            <thead>
                <tr>
                    <th aria-sort="ascending">"Name"</th>
                    <th aria-sort="none">"Date"</th>
                </tr>
            </thead>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("aria-sort"));
}

#[test]
fn test_striped_table() {
    let rows = vec![1, 2, 3, 4];
    let component = html! {
        <table>
            @for (i, row) in rows.iter().enumerate() {
                <tr data-odd={if i % 2 == 1 { "true" } else { "false" }}>
                    <td>{row}</td>
                </tr>
            }
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("data-odd"));
}

#[test]
fn test_responsive_table_wrapper() {
    let component = html! {
        <div role="region" aria-label="Data Table" tabindex="0">
            <table>
                <tr><td>"Data"</td></tr>
            </table>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"region\"") && html.contains("tabindex"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Data Grid Patterns (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_grid_role() {
    let component = html! {
        <table role="grid">
            <tr><td>"Cell"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"grid\""));
}

#[test]
fn test_gridcell_role() {
    let component = html! {
        <table role="grid">
            <tr><td role="gridcell">"Cell"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"gridcell\""));
}

#[test]
fn test_rowheader() {
    let component = html! {
        <table>
            <tr><th role="rowheader">"Row 1"</th><td>"Data"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"rowheader\""));
}

#[test]
fn test_columnheader() {
    let component = html! {
        <table>
            <tr><th role="columnheader">"Column"</th></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"columnheader\""));
}

#[test]
fn test_selected_row() {
    let component = html! {
        <table>
            <tr aria-selected="true"><td>"Selected"</td></tr>
            <tr aria-selected="false"><td>"Not Selected"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("aria-selected"));
}

#[test]
fn test_expandable_row() {
    let component = html! {
        <table>
            <tr aria-expanded="false"><td>"Expandable"</td></tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("aria-expanded"));
}

#[test]
fn test_table_with_checkbox() {
    let component = html! {
        <table>
            <tr>
                <td><input type="checkbox" /></td>
                <td>"Item"</td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("checkbox"));
}

#[test]
fn test_table_with_actions() {
    let component = html! {
        <table>
            <tr>
                <td>"Item"</td>
                <td>
                    <button>"Edit"</button>
                    <button>"Delete"</button>
                </td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("Edit") && html.contains("Delete"));
}

#[test]
fn test_table_with_links() {
    let component = html! {
        <table>
            <tr>
                <td><a href="/item/1">"View"</a></td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("href=") && html.contains("View"));
}

#[test]
fn test_table_with_status() {
    let statuses = vec![
        ("Active", "green"),
        ("Pending", "yellow"),
        ("Inactive", "gray"),
    ];
    let component = html! {
        <table>
            @for (status, color) in &statuses {
                <tr>
                    <td data-color={*color}>{status}</td>
                </tr>
            }
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("Active") && html.contains("Inactive"));
}

#[test]
fn test_table_with_images() {
    let component = html! {
        <table>
            <tr>
                <td><img src="/avatar.jpg" alt="User" /></td>
                <td>"Name"</td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("<img") && html.contains("alt="));
}

#[test]
fn test_table_with_badge() {
    let component = html! {
        <table>
            <tr>
                <td>"Task"</td>
                <td><span>"Urgent"</span></td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("Urgent"));
}

#[test]
fn test_table_with_progress() {
    let component = html! {
        <table>
            <tr>
                <td>"Download"</td>
                <td><progress value="75" max="100">"75%"</progress></td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("<progress"));
}

#[test]
fn test_table_with_tooltip() {
    let component = html! {
        <table>
            <tr>
                <td><span title="Full description">"Short"</span></td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("title=") && html.contains("Full description"));
}

#[test]
fn test_table_with_icon() {
    let component = html! {
        <table>
            <tr>
                <td><span data-icon="check">"✓"</span></td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("data-icon"));
}

#[test]
fn test_table_with_input() {
    let component = html! {
        <table>
            <tr>
                <td><input type="number" value="10" /></td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("type=\"number\""));
}

#[test]
fn test_table_with_select() {
    let component = html! {
        <table>
            <tr>
                <td>
                    <select>
                        <option>"Option 1"</option>
                        <option>"Option 2"</option>
                    </select>
                </td>
            </tr>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("<select>") && html.contains("<option>"));
}

#[test]
fn test_table_pagination_info() {
    let component = html! {
        <div>
            <table>
                <tr><td>"Data"</td></tr>
            </table>
            <p>"Showing 1-10 of 100"</p>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("Showing 1-10"));
}

#[test]
fn test_table_filter_row() {
    let component = html! {
        <table>
            <thead>
                <tr><th>"Name"</th></tr>
                <tr><th><input type="search" placeholder="Filter..." /></th></tr>
            </thead>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("type=\"search\"") && html.contains("Filter"));
}

#[test]
fn test_table_empty_state() {
    let items: Vec<&str> = vec![];
    let component = html! {
        <table>
            <tbody>
                @if items.is_empty() {
                    <tr><td colspan="3">"No data available"</td></tr>
                } else {
                    @for item in &items {
                        <tr><td>{item}</td></tr>
                    }
                }
            </tbody>
        </table>
    };
    let html = test::render(&component);
    assert!(html.contains("No data available"));
}
