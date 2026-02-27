use std::env;
use zed_extension_api::{self as zed, Command, Extension, LanguageServerId, Result};

struct PleaseExtension;

impl Extension for PleaseExtension {
    fn new() -> Self {
        // Load .env file if it exists
        dotenv::dotenv().ok();
        PleaseExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Command> {
        let build_langserver_path = env::var("BUILD_LANGSERVER_PATH")
            .map_err(|_| "BUILD_LANGSERVER_PATH environment variable not set")?;

        Ok(Command {
            command: build_langserver_path,
            // BUILD_LANGSERVER_PATH environment variable must be set in .env file
            args: vec!["--stdio".to_string()], // Adjust args if needed; --stdio enables stdio communication
            env: Default::default(),
        })
    }
}

zed_extension_api::register_extension!(PleaseExtension);
