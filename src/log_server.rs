use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

/// Snapshot of terminal content for a single tab
#[derive(Clone)]
pub struct TerminalSnapshot {
    pub tab_id: usize,
    pub tab_name: String,
    pub content: String,
}

/// Snapshot of a file being viewed
#[derive(Clone)]
pub struct FileSnapshot {
    pub tab_id: usize,
    pub file_path: String,
    pub content: String,
}

/// Shared state between the Iced app and HTTP server
#[derive(Clone)]
pub struct ServerState {
    pub terminals: Arc<RwLock<HashMap<usize, TerminalSnapshot>>>,
    pub files: Arc<RwLock<HashMap<usize, FileSnapshot>>>,
    pub shutdown: Arc<tokio::sync::Notify>,
    pub bound_port: Arc<std::sync::Mutex<Option<u16>>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            terminals: Arc::new(RwLock::new(HashMap::new())),
            files: Arc::new(RwLock::new(HashMap::new())),
            shutdown: Arc::new(tokio::sync::Notify::new()),
            bound_port: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    /// Get the base URL of the log server, if it's running
    pub fn base_url(&self) -> Option<String> {
        self.bound_port
            .lock()
            .ok()
            .and_then(|port| port.map(|p| format!("http://localhost:{}", p)))
    }
}

/// Find an available port, trying 3030-3039 first, then OS-assigned
fn find_available_port() -> u16 {
    for port in 3030..3040 {
        if std::net::TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
    // Fallback: let OS assign a port
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind to any port");
    listener.local_addr().unwrap().port()
}

/// Start the HTTP log server with graceful shutdown
pub async fn start_server(state: ServerState) {
    let shutdown = state.shutdown.clone();
    let bound_port = state.bound_port.clone();
    let state_filter = warp::any().map(move || state.clone());

    // Route: GET / - List all tabs
    let index = warp::path::end()
        .and(state_filter.clone())
        .and_then(handle_index);

    // Route: GET /tab/{id} - Show terminal content for a specific tab
    let tab = warp::path!("tab" / usize)
        .and(state_filter.clone())
        .and_then(handle_tab);

    // Route: GET /file/{id} - Show file content for a specific tab
    let file = warp::path!("file" / usize)
        .and(state_filter.clone())
        .and_then(handle_file);

    let routes = index.or(tab).or(file);

    let port = find_available_port();

    let (_addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], port), async move {
            shutdown.notified().await;
        });

    // Store the actual port so the app can reference it
    if let Ok(mut p) = bound_port.lock() {
        *p = Some(port);
    }

    println!("Log server started at http://localhost:{}", port);
    server.await;
    println!("Log server shut down");
}

/// Handler for index page - lists all tabs
async fn handle_index(state: ServerState) -> Result<impl warp::Reply, warp::Rejection> {
    let snapshots = state.terminals.read().await;

    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>GitTerm Log Viewer</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            max-width: 1200px;
            margin: 40px auto;
            padding: 20px;
            background: #1e1e1e;
            color: #d4d4d4;
        }
        h1 {
            color: #4ec9b0;
            border-bottom: 2px solid #4ec9b0;
            padding-bottom: 10px;
        }
        .tab-list {
            list-style: none;
            padding: 0;
        }
        .tab-item {
            background: #252526;
            margin: 10px 0;
            padding: 15px 20px;
            border-radius: 5px;
            border-left: 4px solid #4ec9b0;
        }
        .tab-item a {
            color: #4ec9b0;
            text-decoration: none;
            font-size: 18px;
            font-weight: 500;
        }
        .tab-item a:hover {
            text-decoration: underline;
        }
        .tab-id {
            color: #858585;
            font-size: 14px;
            margin-left: 10px;
        }
    </style>
</head>
<body>
    <h1>GitTerm Log Viewer</h1>
    <p>Select a tab to view its terminal output:</p>
    <ul class="tab-list">
"#,
    );

    let mut tabs: Vec<_> = snapshots.values().collect();
    tabs.sort_by_key(|t| t.tab_id);

    for snapshot in tabs {
        html.push_str(&format!(
            r#"        <li class="tab-item">
            <a href="/tab/{}">{}</a>
            <span class="tab-id">Tab #{}</span>
        </li>
"#,
            snapshot.tab_id, snapshot.tab_name, snapshot.tab_id
        ));
    }

    html.push_str(
        r#"    </ul>
</body>
</html>"#,
    );

    Ok(warp::reply::html(html))
}

/// Handler for tab page - shows terminal content
async fn handle_tab(
    tab_id: usize,
    state: ServerState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let snapshots = state.terminals.read().await;

    if let Some(snapshot) = snapshots.get(&tab_id) {
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{} - GitTerm Logs</title>
    <style>
        body {{
            font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
            margin: 0;
            padding: 20px;
            background: #1e1e1e;
            color: #d4d4d4;
        }}
        .header {{
            position: sticky;
            top: 0;
            background: #252526;
            padding: 15px 20px;
            margin: -20px -20px 20px -20px;
            border-bottom: 2px solid #4ec9b0;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        .header h1 {{
            margin: 0;
            color: #4ec9b0;
            font-size: 20px;
        }}
        .header a {{
            color: #4ec9b0;
            text-decoration: none;
            font-size: 14px;
        }}
        .header a:hover {{
            text-decoration: underline;
        }}
        .actions {{
            margin-bottom: 20px;
            background: #252526;
            padding: 10px 15px;
            border-radius: 5px;
        }}
        .actions button {{
            background: #4ec9b0;
            color: #1e1e1e;
            border: none;
            padding: 8px 16px;
            border-radius: 3px;
            cursor: pointer;
            font-size: 14px;
            margin-right: 10px;
        }}
        .actions button:hover {{
            background: #5fd9c0;
        }}
        pre {{
            background: #252526;
            padding: 20px;
            border-radius: 5px;
            overflow-x: auto;
            white-space: pre-wrap;
            word-wrap: break-word;
            line-height: 1.5;
            font-size: 13px;
        }}
        .search-box {{
            display: inline-block;
            margin-left: 10px;
        }}
        .search-box input {{
            background: #3c3c3c;
            border: 1px solid #555;
            color: #d4d4d4;
            padding: 6px 12px;
            border-radius: 3px;
            font-size: 14px;
            width: 300px;
        }}
        .search-box input:focus {{
            outline: none;
            border-color: #4ec9b0;
        }}
        mark {{
            background: #ffd700;
            color: #000;
            padding: 2px;
        }}
    </style>
    <script>
        function copyToClipboard() {{
            const pre = document.getElementById('terminal-content');
            const text = pre.textContent;
            navigator.clipboard.writeText(text).then(() => {{
                const btn = document.getElementById('copy-btn');
                const oldText = btn.textContent;
                btn.textContent = 'Copied!';
                setTimeout(() => {{ btn.textContent = oldText; }}, 2000);
            }});
        }}

        function searchText() {{
            const query = document.getElementById('search-input').value;
            const content = document.getElementById('terminal-content');
            const text = content.getAttribute('data-original') || content.textContent;

            if (!content.hasAttribute('data-original')) {{
                content.setAttribute('data-original', text);
            }}

            if (query === '') {{
                content.textContent = text;
                return;
            }}

            // Escape HTML and highlight matches
            const escaped = text.replace(/[&<>"']/g, m => ({{
                '&': '&amp;',
                '<': '&lt;',
                '>': '&gt;',
                '"': '&quot;',
                "'": '&#39;'
            }})[m]);

            const regex = new RegExp(query.replace(/[.*+?^${{}}()|[\]\\]/g, '\\$&'), 'gi');
            const highlighted = escaped.replace(regex, match => `<mark>${{match}}</mark>`);

            content.innerHTML = highlighted;
        }}

        function refreshPage() {{
            location.reload();
        }}
    </script>
</head>
<body>
    <div class="header">
        <h1>{} (Tab #{})</h1>
        <a href="/">← Back to all tabs</a>
    </div>
    <div class="actions">
        <button id="copy-btn" onclick="copyToClipboard()">📋 Copy All</button>
        <button onclick="refreshPage()">🔄 Refresh</button>
        <div class="search-box">
            <input type="text" id="search-input" placeholder="Search in output..." onkeyup="searchText()">
        </div>
    </div>
    <pre id="terminal-content">{}</pre>
</body>
</html>"#,
            snapshot.tab_name,
            snapshot.tab_name,
            tab_id,
            html_escape(&snapshot.content)
        );

        Ok(warp::reply::html(html))
    } else {
        Ok(warp::reply::html(format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Tab Not Found</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            max-width: 800px;
            margin: 40px auto;
            padding: 20px;
            background: #1e1e1e;
            color: #d4d4d4;
            text-align: center;
        }}
        h1 {{ color: #f48771; }}
        a {{
            color: #4ec9b0;
            text-decoration: none;
        }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <h1>Tab #{} Not Found</h1>
    <p><a href="/">← Back to all tabs</a></p>
</body>
</html>"#,
            tab_id
        )))
    }
}

/// Handler for file page - shows file content
async fn handle_file(
    tab_id: usize,
    state: ServerState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let files = state.files.read().await;

    if let Some(file_snapshot) = files.get(&tab_id) {
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{} - GitTerm File Viewer</title>
    <style>
        body {{
            font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
            margin: 0;
            padding: 20px;
            background: #1e1e1e;
            color: #d4d4d4;
        }}
        .header {{
            position: sticky;
            top: 0;
            background: #252526;
            padding: 15px 20px;
            margin: -20px -20px 20px -20px;
            border-bottom: 2px solid #4ec9b0;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        .header h1 {{
            margin: 0;
            color: #4ec9b0;
            font-size: 18px;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
        }}
        .header a {{
            color: #4ec9b0;
            text-decoration: none;
            font-size: 14px;
        }}
        .header a:hover {{
            text-decoration: underline;
        }}
        .actions {{
            margin-bottom: 20px;
            background: #252526;
            padding: 10px 15px;
            border-radius: 5px;
        }}
        .actions button {{
            background: #4ec9b0;
            color: #1e1e1e;
            border: none;
            padding: 8px 16px;
            border-radius: 3px;
            cursor: pointer;
            font-size: 14px;
            margin-right: 10px;
        }}
        .actions button:hover {{
            background: #5fd9c0;
        }}
        pre {{
            background: #252526;
            padding: 20px;
            border-radius: 5px;
            overflow-x: auto;
            white-space: pre;
            line-height: 1.5;
            font-size: 13px;
        }}
        .line-numbers {{
            display: inline-block;
            color: #858585;
            padding-right: 15px;
            border-right: 1px solid #3c3c3c;
            margin-right: 15px;
            user-select: none;
        }}
    </style>
    <script>
        function copyToClipboard() {{
            const pre = document.getElementById('file-content');
            const text = pre.textContent;
            navigator.clipboard.writeText(text).then(() => {{
                const btn = document.getElementById('copy-btn');
                const oldText = btn.textContent;
                btn.textContent = 'Copied!';
                setTimeout(() => {{ btn.textContent = oldText; }}, 2000);
            }});
        }}
    </script>
</head>
<body>
    <div class="header">
        <h1>{}</h1>
        <a href="/">← Back to all tabs</a>
    </div>
    <div class="actions">
        <button id="copy-btn" onclick="copyToClipboard()">📋 Copy All</button>
    </div>
    <pre id="file-content">{}</pre>
</body>
</html>"#,
            file_snapshot.file_path,
            file_snapshot.file_path,
            add_line_numbers(&html_escape(&file_snapshot.content))
        );

        Ok(warp::reply::html(html))
    } else {
        Ok(warp::reply::html(format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>File Not Found</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            max-width: 800px;
            margin: 40px auto;
            padding: 20px;
            background: #1e1e1e;
            color: #d4d4d4;
            text-align: center;
        }}
        h1 {{ color: #f48771; }}
        a {{
            color: #4ec9b0;
            text-decoration: none;
        }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <h1>No file currently viewed in Tab #{}</h1>
    <p><a href="/">← Back to all tabs</a></p>
</body>
</html>"#,
            tab_id
        )))
    }
}

/// Add line numbers to content
fn add_line_numbers(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let max_line = lines.len();
    let width = max_line.to_string().len();

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            format!(
                "<span class=\"line-numbers\">{:width$}</span>{}",
                i + 1,
                line,
                width = width
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
