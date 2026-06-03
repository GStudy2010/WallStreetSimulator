use axum::{
    extract::{
        ws::WebSocketUpgrade,
        State,
        Query,
    },
    response::IntoResponse,
};
use axum::extract::ws::{
    Message,
    WebSocket,
};

use futures::{
    SinkExt,
    StreamExt,
};
use serde::Serialize;
use tokio::sync::mpsc;
use crate::db::user::init::{AppState, Room};

#[derive(Serialize)]
pub struct GameStartedMessage {
    #[serde(rename = "type")]
    msg_type: String,
}
#[derive(serde::Deserialize)]
pub struct WsQuery {
    room_token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {
        handle_socket(socket, query.room_token, state)
    })
}
async fn handle_socket(
    socket: WebSocket,
    room_token: String,
    state: AppState,
) {
    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .entry(room_token.clone())
            .or_insert(Room{
                clients: Vec::new(),
                game_started: false,
            });

        room.clients.push(tx);
    }

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(_)) = receiver.next().await {}

    send_task.abort();
}
