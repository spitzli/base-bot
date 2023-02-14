/* use crate::events::interaction_create::InteractionCreateEvent;
use crate::events::ready::ReadyEvent;
use crate::Context; */
use anyhow::Result;
use std::sync::Arc;
use twilight_gateway::Event;

use crate::Context;

pub async fn handle_event(event: Event, _context: Arc<Context>) -> Result<()> {
    let event = event.clone();
    match event {
        Event::Ready(data) => {
            //ReadyEvent::execute(*data, context.clone()).await?
            tracing::info!("Ready as {}", data.user.name);
        }
        /*         Event::InteractionCreate(data) => {
            InteractionCreateEvent::execute(*data, context.clone()).await?
        } */
        _ => {}
    }
    Ok(())
}
