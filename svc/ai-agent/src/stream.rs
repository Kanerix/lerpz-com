use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
};

use axum::response::sse::Event;
use lerpz_axum::problem::Problem;
use rig::tool::Tool;
use tokio_stream::Stream;
use uuid::Uuid;

pub enum AgentEvent<T>
where
    T: Tool,
{
    /// The agent has been initialized.
    Init(Uuid),
    /// A message from the agent.
    Message(String),
    /// A tool call from the agent.
    ToolCall(T),
    /// A tool result from the agent.
    ToolResult(String),
    /// Conversation state has been saved.
    Saved,
    /// An error occurred while processing the event.
    Error(String),
}

impl<T> From<AgentEvent<T>> for Event
where
    T: Tool,
{
    fn from(event: AgentEvent<T>) -> Self {
        match event {
            AgentEvent::Init(id) => Event::default().event("init").data(id.to_string()),
            AgentEvent::Message(text) => Event::default().event("message").data(text),
            AgentEvent::ToolCall(tool) => Event::default().event("tool_call").data(tool.name()),
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

    pub fn init<T: Tool>(&self) -> AgentEvent<T> {
        AgentEvent::Init(Uuid::new_v4())
    }
}

impl Stream for AgentStream {
    type Item = Result<Event, Infallible>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
