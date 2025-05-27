use crate::{Context, Error};

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Vote for something
///
/// Enter `~vote pumpkin` to vote for pumpkins
#[poise::command(prefix_command, slash_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    let num_votes = {
        let mut hash_map = ctx.data().votes.lock().unwrap();
        let num_votes = hash_map.entry(choice.clone()).or_default();
        *num_votes += 1;
        *num_votes
    };

    let response = format!("Successfully voted for {choice}. {choice} now has {num_votes} votes!");
    ctx.say(response).await?;
    Ok(())
}

/// Retrieve number of votes
///
/// Retrieve the number of votes either in general, or for a specific choice:
/// ```
/// ~getvotes
/// ~getvotes pumpkin
/// ```
#[poise::command(prefix_command, track_edits, aliases("votes"), slash_command)]
pub async fn getvotes(
    ctx: Context<'_>,
    #[description = "Choice to retrieve votes for"] choice: Option<String>,
) -> Result<(), Error> {
    if let Some(choice) = choice {
        let num_votes = *ctx.data().votes.lock().unwrap().get(&choice).unwrap_or(&0);
        let response = match num_votes {
            0 => format!("Nobody has voted for {choice} yet"),
            _ => format!("{num_votes} people have voted for {choice}"),
        };
        ctx.say(response).await?;
    } else {
        let mut response = String::new();
        for (choice, num_votes) in ctx.data().votes.lock().unwrap().iter() {
            response += &format!("{choice}: {num_votes} votes");
        }

        if response.is_empty() {
            response += "Nobody has voted for anything yet :(";
        }

        ctx.say(response).await?;
    };

    Ok(())
}

/// Move the robot
///
/// Enter `~move_robot` to move the robot forward for 1 second
#[poise::command(prefix_command, slash_command)]
pub async fn move_robot(
    ctx: Context<'_>,
    #[description = "What direction to move in (default forward)"] direction: Option<
        crate::robot_command::Direction,
    >,
    #[description = "How long to move for (default 1 seconds)"] duration: Option<f32>,
    #[description = "How fast to move for (top speed 127, default 50)"] speed: Option<u8>,
) -> Result<(), Error> {
    let duration = duration.unwrap_or(1.0);
    if duration < 0.0 {
        ctx.say("Duration must be positive").await?;
        return Ok(());
    }
    let duration = std::time::Duration::from_secs_f32(duration);

    let speed = speed.unwrap_or(50);
    if !(0..=127).contains(&speed) {
        ctx.say("Speed must be between 0 and 127").await?;
        return Ok(());
    }

    let command = crate::robot_command::RobotCommand::MoveInDirection {
        direction: direction.unwrap_or(crate::robot_command::Direction::Forward),
        duration,
        speed,
    };

    // Send the command to the serial sender
    ctx.data()
        .sender
        .send(command)
        .expect("Failed to send command to serial sender");

    // Respond to the user
    let duration = duration.as_millis();
    ctx.say(format!("Moving forward for {duration} milliseconds"))
        .await?;

    Ok(())
}

/// Stop the robot
#[poise::command(prefix_command, slash_command)]
pub async fn stop_robot(ctx: Context<'_>) -> Result<(), Error> {
    let command = crate::robot_command::RobotCommand::Stop;

    // Send the command to the serial sender
    ctx.data()
        .sender
        .send(command)
        .expect("Failed to send command to serial sender");

    // Respond to the user
    ctx.say("Stopping the robot".to_string()).await?;

    Ok(())
}

