use azumi::prelude::*;
use sqlx::{Pool, Sqlite};
use std::sync::OnceLock;

// Global DB pool for the demo
static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

/// Initialize the DB (called from main.rs)
pub async fn init_db() -> Pool<Sqlite> {
    let pool = Pool::<Sqlite>::connect("sqlite::memory:").await.unwrap();

    // Create table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    // Seed data
    sqlx::query("INSERT INTO todos (text) VALUES ('Learn Azumi'), ('Build a lovely app')")
        .execute(&pool)
        .await
        .unwrap();

    let _ = DB_POOL.set(pool.clone());
    pool
}

#[azumi::live]
pub struct DatabaseTodo {
    pub todos: Vec<TodoItem>,
    pub input: String,
    pub loading: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TodoItem {
    pub id: i64,
    pub text: String,
    pub completed: bool,
}

#[azumi::live_impl(component = "database_todo_view")]
impl DatabaseTodo {
    /// Async Action: Load from DB
    pub async fn load_todos(&mut self) {
        self.loading = true;
        // Optimistic update not needed for load

        if let Some(pool) = DB_POOL.get() {
            // Real Async Query!
            let rows = sqlx::query_as::<_, TodoItem>("SELECT * FROM todos ORDER BY id DESC")
                .fetch_all(pool)
                .await;

            if let Ok(items) = rows {
                self.todos = items;
            }
        }

        self.loading = false;
    }

    /// Async Action: Add Todo
    pub async fn add_todo(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let text = self.input.clone();

        // 1. Optimistic Update (Instant feedback)
        // We use a temporary negative ID that will be replaced by the server re-render
        self.todos.insert(
            0,
            TodoItem {
                id: -1,
                text: text.clone(),
                completed: false,
            },
        );
        self.input.clear();

        // 2. Real Async DB Insert
        if let Some(pool) = DB_POOL.get() {
            // Artificial delay to show off the optimistic UI vs server consistency
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            let _ = sqlx::query("INSERT INTO todos (text) VALUES (?)")
                .bind(text)
                .execute(pool)
                .await;

            // 3. Re-fetch to get correct ID and state
            self.load_todos().await;
        }
    }

    /// Async Action: Toggle Todo
    pub async fn toggle_todo(&mut self) {
        // NOTE: In a real app we'd pass the ID as a param, but for this demo
        // we'll rely on the client knowing which item was clicked via a comprehensive state update
        // or just toggle the first one for simplicity?
        //
        // Actually, Azumi Live's current demo structure often passes the *entire* state.
        // To toggle a specific item, we need to know WHICH one.
        // The current macro limitations mean we often just mutate the global live struct.
        //
        // Ideally, we'd have `fn toggle(&mut self, id: i64)`.
        // Since we don't have argument support in macros yet, we will skip toggle for this simple demo
        // or implement a "Clear All" instead which is easier.
    }

    pub async fn clear_all(&mut self) {
        // Optimistic
        self.todos.clear();

        // Async content
        if let Some(pool) = DB_POOL.get() {
            let _ = sqlx::query("DELETE FROM todos").execute(pool).await;
        }
    }
}

#[azumi::component]
pub fn database_todo_view<'a>(state: &'a DatabaseTodo) -> impl Component + 'a {
    html! {
        <style>
            .container { max-width: "500px"; margin: "0 auto"; padding: "2rem"; }
            .header { text-align: "center"; color: "#333"; }
            .input_group { display: "flex"; gap: "0.5rem"; margin-bottom: "1rem"; }
            .input { flex: "1"; padding: "0.5rem"; border: "1px solid #ccc"; border-radius: "4px"; }
            .btn { padding: "0.5rem 1rem"; background: "#2563eb"; color: "white"; border: "none"; border-radius: "4px"; cursor: "pointer"; }
            .btn:disabled { background: "#93c5fd"; }
            .list { list-style: "none"; padding: "0"; }
            .item {
                padding: "0.75rem"; border-bottom: "1px solid #eee";
                display: "flex"; justify-content: "space-between";
                // Linter complains about align_items if not quoted, but macro might not support dash-idents?
                // Using "align-items" as a string key if needed, or ensuring valid ident.
                // Assuming macro transforms `align_items` -> `align-items`?
                // The error `Unknown CSS property: 'align_items'` suggests it does NOT, but sees the ident.
                // If I use "align-items": "center", standard Rust syntax for struct field is broken ??
                // Azumi style macro parses `ident: value` OR `literal: value` presumably?
                // Checking usage: `justify-content` works above? No, I wrote `justify-content: "..."` in my previous write,
                // but wait, line 143 says: `justify-content: "space-between"`
                // `justify-content` is NOT A VALID RUST IDENTIFIER.
                // So the macro MUST be parsing keys as custom tokens or strings.
                // If so, `align-items` should work fine.
                align-items: "center";
            }
            .item_optimistic { color: "#888"; font-style: "italic"; }
            .spinner { display: "inline-block"; animation: "spin 1s linear infinite"; margin-left: "0.5rem"; }
            .btn_danger { background: "#dc2626"; }
            .text_center { text-align: "center"; color: "#666"; }
            @keyframes spin { 100% { transform: "rotate(360deg)"; } }
        </style>

        <div class={container}>
            <h2 class={header}>"Async SQLite Todo List"</h2>

            <div class={input_group}>
                <input class={input} type="text" name="input" value={state.input} placeholder="Add persistent todo..." />

                <button class={btn} on:click={state.add_todo} disabled={state.loading}>
                    @if state.loading {
                        "Saving..."
                    } else {
                        "Add Async"
                    }
                </button>
            </div>

            <button class={btn btn_danger} on:click={state.clear_all}>
                "Clear DB"
            </button>

            <ul class={list}>
                @for todo in &state.todos {
                    <li class={if todo.id == -1 { "item item_optimistic" } else { "item" }}>
                        <span>
                            {if todo.id == -1 { "⏳ " } else { "✅ " }}
                            {&todo.text}
                        </span>
                        <small>"ID: " {todo.id}</small>
                    </li>
                }
            </ul>

            @if state.todos.is_empty() {
                <p class={text_center}>"No todos in SQLite database."</p>
            }
        </div>
    }
}

pub async fn lesson16_handler() -> axum::response::Html<String> {
    // Ensure DB is init
    if DB_POOL.get().is_none() {
        init_db().await;
    }

    let mut state = DatabaseTodo {
        todos: vec![],
        input: String::new(),
        loading: false,
    };

    // Initial fetch
    state.load_todos().await;

    use database_todo_view_component::*;
    let component_html = azumi::render_to_string(&render(
        Props::builder().state(&state).build().expect("props"),
    ));

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Lesson 16: Async Database</title>
    <style>body {{ font-family: system-ui; background: #fff; margin: 0; }}</style>
</head>
<body>
    {}
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        component_html
    );
    axum::response::Html(html)
}
