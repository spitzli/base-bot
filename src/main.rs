use std::{env, sync::Arc};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{Intents, Shard, ShardId};
use twilight_http::Client;

mod handler;

pub struct Context {
    pub client: Arc<Client>,
    pub cache: Arc<InMemoryCache>,
    //shard: Arc<Shard>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord Token in the environment");

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;
    let client = Arc::new(Client::new(token.clone()));
    let cache = Arc::new(InMemoryCache::new());
    let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);

    let context = Arc::new(Context {
        client,
        cache,
        //shard: shard.clone(),
    });

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tokio::spawn(handler::handle_event(event, context.clone()));
    }

    Ok(())
}
