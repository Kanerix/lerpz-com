use std::{convert::Infallible, pin::Pin, task::Context};

use axum::response::sse::Event;
use lerpz_axum::error::HandlerError;
use tokio_stream::Stream;
use uuid::Uuid;

pub enum AgentEvent {
    /// The agent has been initialized.
    Init(Uuid),
    /// A message from the agent.
    Message(String),
    /// A tool call from the agent.
    ToolCall(String),
    /// A tool result from the agent.
    ToolResult(String),
    /// Conversation state has been saved.
    Saved,
    /// An error occurred while processing the event.
    Error(String),
}

impl From<AgentEvent> for Event {
    fn from(event: AgentEvent) -> Self {
        match event {
            AgentEvent::Init(id) => Event::default().event("init").data(id.to_string()),
            AgentEvent::Message(text) => Event::default().event("message").data(text),
            AgentEvent::ToolCall(name) => Event::default().event("tool_call").data(name),
            AgentEvent::ToolResult(result) => Event::default().event("tool_result").data(result),
            AgentEvent::Saved => Event::default().event("saved"),
            AgentEvent::Error(error) => Event::default().event("error").data(error),
        }
    }
}

pub struct AgentStream {
    db: sqlx::PgPool,
}

impl AgentStream {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }

    pub fn init(&self) -> AgentEvent {
        AgentEvent::Init(Uuid::new_v4())
    }
}

impl Stream for AgentStream {
    type Item = Result<Event, Infallible>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}
