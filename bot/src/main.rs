use anyhow::{Context, Result};
use log::{debug, error, info};
use teloxide::{
    net::Download,
    payloads::GetUpdatesSetters,
    prelude::Requester,
    types::{AllowedUpdate, MediaKind, MessageKind, Update, UpdateKind},
    Bot,
};
use tokio::fs::File;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Error: {:?}", e);
    }
}

fn init() -> Result<()> {
    pretty_env_logger::env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    info!("Initializing..");
    dotenv::dotenv().context("Failed to load env vars")?;
    info!(
        "TELOXIDE_TOKEN: {}",
        std::env::var("TELOXIDE_TOKEN").context("TELOXIDE_TOKEN unset")?
    );
    info!(
        "OWNER_ID: {}",
        std::env::var("OWNER_ID").context("OWNER_ID unset")?
    );
    info!("Finished Initializing..");
    Ok(())
}

async fn run() -> Result<()> {
    init()?;
    let owner = std::env::var("OWNER_ID")
        .unwrap()
        .parse()
        .context("INVALID OWNER_ID")?;
    let bot = Bot::from_env();
    let cancel = CancellationToken::new();
    loop {
        tokio::select! {
            _ = async {
                let updates = bot.get_updates().allowed_updates([AllowedUpdate::Message]).await?;
                tokio::spawn(handle_updates(bot.clone(), updates, owner, cancel.clone()));
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                Result::<()>::Ok(())
            } => {}
            _ = tokio::signal::ctrl_c() => {
                info!("Exiting..");
                cancel.cancel();
                break;
            }
        }
    }
    Ok(())
}

/// This will filter out updates from a specific user and spawn a new task to handle the update.
async fn handle_updates(
    bot: Bot,
    updates: impl IntoIterator<Item = Update>,
    owner: u64,
    cancel: CancellationToken,
) -> Result<()> {
    for update in updates.into_iter() {
        if let Some(true) = update.user().map(|user| user.id.0 == owner) {
            tokio::spawn(handle_update(bot.clone(), update, cancel.clone()));
        }
    }
    Ok(())
}

async fn handle_update(bot: Bot, update: Update, cancel: CancellationToken) -> Result<()> {
    if let UpdateKind::Message(msg) = update.kind {
        if let MessageKind::Common(msg) = msg.kind {
            let media = msg.media_kind;
            debug!("Get Msg: {}", serde_json::to_string_pretty(&media)?);
            if let Err(e) = handle_media(bot, media, cancel).await {
                error!("Error Handling Media: {:?}", e);
            }
        }
    }
    Ok(())
}

async fn handle_media(bot: Bot, media: MediaKind, cancel: CancellationToken) -> Result<()> {
    match media {
        MediaKind::Animation(_) => {}
        MediaKind::Audio(_) => {}
        MediaKind::Photo(_) => {}
        MediaKind::Text(_) => {}
        MediaKind::Video(v) => {
            let file_id = v.video.file.id;
            info!("{}", file_id);
            let path = bot.get_file(file_id).await?;
            // let mut destination = File::create("video.mp4").await?;
            // bot.download_file(&path.path, &mut destination).await?;
            info!("Download: {:?}", path);
        }
        MediaKind::VideoNote(_) => {}
        MediaKind::Voice(_) => {}
        _ => {}
    }
    Ok(())
}
