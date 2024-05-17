use axum::{response::Html, routing::get, Router};
use std::{net::SocketAddr, sync::Mutex};
use serde::{Serialize, Deserialize};
use std::fs;

// Define a struct to represent the state of each peer
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PeerState {
    peer_id: String,
    role: String,
}

// Define a struct to represent the state of the entire network
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NetworkState {
    peer_list: Vec<PeerState>,
}


// Global state to hold the network state (shared between all requests)
static NETWORK_STATE: Mutex<NetworkState> = Mutex::new(NetworkState { peer_list: Vec::new() });


#[tokio::main]
async fn main() {
    // build our application with routes
    let app = Router::new()
        .route("/", get(handler))
        .route("/toggle", get(toggle_handler))
        .route("/update", get(update_handler));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handles requests to "/"
async fn handler() -> Html<String> {
    let network_state = NETWORK_STATE.lock().unwrap();

    let mut peer_list_html = String::new();
    for peer in &network_state.peer_list {
        let checked = if peer.role == "byzantine" { "checked" } else { "" };
        peer_list_html.push_str(&format!(
            "<li>
                {} - Role: {}
                <label class=\"switch\">
                    <input type=\"checkbox\" {} onclick=\"toggle('{}')\">
                    <span class=\"slider round\"></span>
                </label>
            </li>",
            peer.peer_id, peer.role, checked, peer.peer_id
        ));
    }

    let html = format!(
        "<html>
        <head>
            <title>Peer Server</title>
        </head>
        <body>
            <h1>Peer List</h1>
            <ul>
                {}
            </ul>
            <script>
                async function toggle(peerId) {{
                    const response = await fetch(`/toggle?peer_id=${{peerId}}`);
                    if (response.ok) {{
                        location.reload(); // Reload the page after toggling
                    }} else {{
                        console.error('Toggle failed');
                    }}
                }}
            </script>
            <style>
                .switch {{
                    position: relative;
                    display: inline-block;
                    width: 60px;
                    height: 34px;
                }}
                .switch input {{
                    opacity: 0;
                    width: 0;
                    height: 0;
                }}
                .slider {{
                    position: absolute;
                    cursor: pointer;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background-color: #ccc;
                    -webkit-transition: .4s;
                    transition: .4s;
                }}
                .slider:before {{
                    position: absolute;
                    content: \"\";
                    height: 26px;
                    width: 26px;
                    left: 4px;
                    bottom: 4px;
                    background-color: white;
                    -webkit-transition: .4s;
                    transition: .4s;
                }}
                input:checked + .slider {{
                    background-color: #2196F3;
                }}
                input:focus + .slider {{
                    box-shadow: 0 0 1px #2196F3;
                }}
                input:checked + .slider:before {{
                    -webkit-transform: translateX(26px);
                    -ms-transform: translateX(26px);
                    transform: translateX(26px);
                }}
                .slider.round {{
                    border-radius: 34px;
                }}
                .slider.round:before {{
                    border-radius: 50%;
                }}
            </style>
        </body>
        </html>",
        peer_list_html
    );

    Html(html)
}


/*
// Global state to hold the network state (shared between all requests)
static NETWORK_STATE: Mutex<NetworkState> = Mutex::new(NetworkState { peer_list: Vec::new() });

#[tokio::main]
async fn main() {
    // build our application with routes
    let app = Router::new()
        .route("/", get(handler))
        .route("/toggle", get(toggle_handler))
        .route("/update", get(update_handler));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handles requests to "/"
async fn handler() -> Html<&'static str> {
    println!("Request received\nSending response.");
     let html = "
        <html>
        <head>
            <title>Peer Server</title>
        </head>
        <body>
            <h1>Peer List</h1>
            <ul>
                {% for peer in peers %}
                <li>
                    {{ peer.peer_id }} - Role: {{ peer.role }}
                    <label class=\"switch\">
                        <input type=\"checkbox\" {% if peer.role == \"byzantine\" %} checked {% endif %} onclick=\"toggle('{{ peer.peer_id }}')\">
                        <span class=\"slider round\"></span>
                    </label>
                </li>
                {% endfor %}
            </ul>
            <script>
                async function toggle(peerId) {
                    const response = await fetch(`/toggle?peer_id=${peerId}`);
                    if (response.ok) {
                        location.reload(); // Reload the page after toggling
                    } else {
                        console.error('Toggle failed');
                    }
                }
            </script>
            <style>
                .switch {
                    position: relative;
                    display: inline-block;
                    width: 60px;
                    height: 34px;
                }
                .switch input {
                    opacity: 0;
                    width: 0;
                    height: 0;
                }
                .slider {
                    position: absolute;
                    cursor: pointer;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background-color: #ccc;
                    -webkit-transition: .4s;
                    transition: .4s;
                }
                .slider:before {
                    position: absolute;
                    content: \"\";
                    height: 26px;
                    width: 26px;
                    left: 4px;
                    bottom: 4px;
                    background-color: white;
                    -webkit-transition: .4s;
                    transition: .4s;
                }
                input:checked + .slider {
                    background-color: #2196F3;
                }
                input:focus + .slider {
                    box-shadow: 0 0 1px #2196F3;
                }
                input:checked + .slider:before {
                    -webkit-transform: translateX(26px);
                    -ms-transform: translateX(26px);
                    transform: translateX(26px);
                }
                .slider.round {
                    border-radius: 34px;
                }
                .slider.round:before {
                    border-radius: 50%;
                }
            </style>
        </body>
        </html>
    ";
    Html(html)
}

*/

// Handles requests to "/toggle?peer_id=peer01"
async fn toggle_handler(params: axum::extract::Query<(String,)>) -> Html<&'static str> {
    let (peer_id,) = params.0;

    // Lock the global network state
    let mut network_state = NETWORK_STATE.lock().unwrap();

    // Find the peer with the specified ID and toggle its role
    for peer in &mut network_state.peer_list {
        if peer.peer_id == peer_id {
            if peer.role == "normal" {
                peer.role = "byzantine".to_string();
            } else {
                peer.role = "normal".to_string();
            }
        }
    }

    Html("Toggle successful")
}

// Handles requests to "/update"
async fn update_handler() -> Html<&'static str> {
    // Lock the global network state
    let network_state = NETWORK_STATE.lock().unwrap();

    // Clone the network state to avoid holding the lock while serializing
    let network_state = network_state.clone();

    // Drop the lock to prevent deadlock and unnecessary blocking
    drop(network_state);

    // Serialize the network state to JSON
    let json = serde_json::to_string_pretty(&NETWORK_STATE.lock().unwrap().peer_list).unwrap();

    // Write the JSON to a file
    fs::write("network_state.json", json).unwrap();

    Html("Update successful")
}
