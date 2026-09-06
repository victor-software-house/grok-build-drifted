<<<<<<< HEAD
//! `/recap` (alias `/summarize`) -- summarize the session so far ("where was I").
=======
//! `/recap` (alias `/summarize`): summarize the session so far ("where was I").
>>>>>>> 72a61251fcffb464bcc687aeb5a998e5a98ec0c9
//!
//! Returns `CommandResult::Action(Action::SendRecap { auto: false })`.
//! The dispatch layer fires it as the ACP ext method `x.ai/recap`, which bypasses the prompt queue.
//! The recap arrives asynchronously as a scrollback line and is never added to the model conversation.

use crate::app::actions::Action;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand, slash_meta};

pub struct RecapCommand;

impl SlashCommand for RecapCommand {
<<<<<<< HEAD
    fn name(&self) -> &str {
        "recap"
    }

    fn aliases(&self) -> &[&str] {
        &["summarize"]
    }

    fn description(&self) -> &str {
        "Summarize the session so far"
    }

    fn session_scoped(&self) -> bool {
        true
    }

    fn usage(&self) -> &str {
        "/recap"
=======
    slash_meta! {
        name: "recap",
        aliases: ["summarize"],
        description: "Summarize the session so far",
        usage: "/recap",
        session_scoped: true,
>>>>>>> 72a61251fcffb464bcc687aeb5a998e5a98ec0c9
    }

    fn run(&self, _ctx: &mut CommandExecCtx, _args: &str) -> CommandResult {
        CommandResult::Action(Action::SendRecap { auto: false })
    }
}
