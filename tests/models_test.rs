use discord_ipc_rp::{Result, activity, DiscordIpc, DiscordIpcClient};

#[test]
fn test_models() -> Result<()> {
    let mut client = DiscordIpcClient::new("771124766517755954");
    client.connect()?;

    let activity = activity::Activity::new()
        .state("A test")
        .details("A placeholder")
        .assets(
            activity::Assets::new()
                .large_image("large-image")
                .large_text("Large text"),
        )
        .buttons(vec![activity::Button::new(
            "A button",
            "https://github.com",
        )]);
    client.set_activity(activity)?;

    std::thread::sleep(std::time::Duration::from_secs(10));

    client.close()?;
    Ok(())
}
