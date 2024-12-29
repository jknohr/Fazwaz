use tokio::sync::mpsc;
use futures::{SinkExt, StreamExt};
use axum_ws::{WebSocket, Message};
use serde::{Serialize, Deserialize};
use tracing::{info, error, instrument};
use std::sync::Arc;
use std::collections::HashMap;
use anyhow::Result;

use crate::backend::{
    common::types::{
        image_types::{ImageUploadSession, ImageChunk, UploadStatus},
        website_sections::WebsiteSections,
    },
    f_ai_core::state::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WebSocketMessage {
    // Session management
    InitUpload {
        listing_id: String,
        section: WebsiteSections,
    },
    SessionCreated {
        session_id: String,
    },
    
    // Upload
    ChunkUpload(ImageChunk),
    ChunkReceived {
        session_id: String,
        sequence: u32,
    },
    
    // Status updates
    UploadProgress {
        session_id: String,
        status: UploadStatus,
    },
    
    // Errors
    Error {
        code: String,
        message: String,
    },
}

pub struct WebSocketHandler {
    state: Arc<AppState>,
    upload_sessions: Arc<tokio::sync::RwLock<HashMap<String, ImageUploadSession>>>,
}

impl WebSocketHandler {
    #[instrument(skip(socket, state))]
    pub async fn handle_connection(socket: WebSocket, state: Arc<AppState>) {
        let (mut sender, mut receiver) = socket.split();
        let (tx, mut rx) = mpsc::channel::<Message>(100);
        
        let handler = Arc::new(Self {
            state,
            upload_sessions: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        });

        // Handle incoming messages
        let handle_incoming = {
            let handler = handler.clone();
            async move {
                while let Some(Ok(msg)) = receiver.next().await {
                    if let Message::Text(text) = msg {
                        if let Err(e) = handler.process_message(&text, &tx).await {
                            error!("Error processing message: {}", e);
                            let error_msg = WebSocketMessage::Error {
                                code: "PROCESSING_ERROR".into(),
                                message: e.to_string(),
                            };
                            let _ = tx.send(Message::Text(serde_json::to_string(&error_msg)?)).await;
                        }
                    }
                }
                Ok::<_, anyhow::Error>(())
            }
        };

        // Forward messages to client
        let forward_messages = async move {
            while let Some(msg) = rx.recv().await {
                if sender.send(msg).await.is_err() {
                    break;
                }
            }
        };

        // Run both tasks concurrently
        tokio::select! {
            _ = handle_incoming => {}
            _ = forward_messages => {}
        }
    }

    #[instrument(skip(self, tx))]
    async fn process_message(
        &self,
        text: &str,
        tx: &mpsc::Sender<Message>,
    ) -> Result<(), anyhow::Error> {
        let msg: WebSocketMessage = serde_json::from_str(text)?;
        
        match msg {
            WebSocketMessage::InitUpload { listing_id, section } => {
                self.handle_init_upload(listing_id, section, tx).await?
            }
            WebSocketMessage::ChunkUpload(chunk) => {
                self.handle_chunk_upload(chunk, tx).await?
            }
            _ => {
                error!("Unexpected message type");
            }
        }
        
        Ok(())
    }

    #[instrument(skip(self, tx))]
    async fn handle_init_upload(
        &self,
        listing_id: String,
        section: WebsiteSections,
        tx: &mpsc::Sender<Message>,
    ) -> Result<()> {
        // Create new upload session
        let session = ImageUploadSession {
            session_id: uuid7::uuid7().to_string(),
            listing_id,
            section,
            status: UploadStatus::Initialized,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Store session
        {
            let mut sessions = self.upload_sessions.write().await;
            sessions.insert(session.session_id.clone(), session.clone());
        }

        // Send session created message
        let response = WebSocketMessage::SessionCreated {
            session_id: session.session_id,
        };
        tx.send(Message::Text(serde_json::to_string(&response)?)).await?;

        Ok(())
    }

    #[instrument(skip(self, tx))]
    async fn handle_chunk_upload(
        &self,
        chunk: ImageChunk,
        tx: &mpsc::Sender<Message>,
    ) -> Result<()> {
        // Get session
        let mut session = {
            let sessions = self.upload_sessions.read().await;
            sessions.get(&chunk.session_id)
                .ok_or_else(|| anyhow::anyhow!("Session not found"))?
                .clone()
        };

        // Update session status
        if let UploadStatus::Initialized = session.status {
            session.status = UploadStatus::Uploading {
                chunks_received: 0,
                total_chunks: 0, // Will be updated as we receive chunks
            };
        }

        // Store chunk data
        self.state.store_image_chunk(&chunk).await?;

        // Send chunk received acknowledgment
        let response = WebSocketMessage::ChunkReceived {
            session_id: chunk.session_id,
            sequence: chunk.sequence,
        };
        tx.send(Message::Text(serde_json::to_string(&response)?)).await?;

        // If this is the final chunk, start processing
        if chunk.is_final {
            session.status = UploadStatus::Processing;
            self.state.process_image_upload(&session).await?;
        }

        // Update session
        {
            let mut sessions = self.upload_sessions.write().await;
            sessions.insert(session.session_id.clone(), session.clone());
        }

        // Send status update
        let status_msg = WebSocketMessage::UploadProgress {
            session_id: session.session_id,
            status: session.status,
        };
        tx.send(Message::Text(serde_json::to_string(&status_msg)?)).await?;

        Ok(())
    }
} 