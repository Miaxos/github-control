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
    // CI
    pub ci_success: String,
    pub ci_error: String,
    pub ci_failure: String,
    pub ci_expected: String,
    pub ci_pending: String,
    pub ci_other: String,
    // Review
    pub review_approved: String,
    pub review_required: String,
    pub review_changes_requested: String,
    pub review_other: String,
    pub review_no_required: String,
}

impl ::std::default::Default for ApplicationConfiguration {
    fn default() -> Self {
        Self {
            /// Github API Key to get pull requests
            github_api_key: "".into(),
            ci_success: "✅".into(),
            ci_error: "❌".into(),
            ci_failure: "⚠️".into(),
            ci_expected: "✴️".into(),
            ci_pending: "⚙️".into(),
            ci_other: "🤔".into(),
            review_approved: "✅".into(),
            review_required: "👋".into(),
            review_changes_requested: "🚫".into(),
            review_other: "🤔".into(),
            review_no_required: "👻".into(),
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
