use serde::{Deserialize, Serialize};
use std::process::exit;

/**
 * Actual settings are stored:
 * MAC: ~/Library/Preferences/rs.github-control/github-control.toml
 * Linux:
 * Win:
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationConfiguration {
    github_api_key: String,
}

impl ::std::default::Default for ApplicationConfiguration {
    fn default() -> Self {
        Self {
            /// Github API Key to get pull requests
            github_api_key: "".into(),
        }
    }
}

impl ApplicationConfiguration {
    /**
     * Load actual configuration or exit the application with an IOERR
     */
    pub fn load_or_exit() -> Self {
        match confy::load::<ApplicationConfiguration>("github-control") {
            Ok(env) => env,
            Err(err) => {
                println!("[ERROR]:{}", err);
                exit(exitcode::IOERR);
            }
        }
    }

    /**
     * Get the github api key
     */
    pub fn github_key(&self) -> &String {
        &self.github_api_key
    }
}
