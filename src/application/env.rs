use std::io::{stdin, stdout, Stdin, Stdout};

/**
 * Environment is the main structure of the application, it should be availaible
 * everywhere, it's our dependency injection
 */
#[derive(Debug)]
pub struct Environment {
    /// Github API Key
    github_api_key: String,
    stdin: Stdin,
    stdout: Stdout,
}

impl Environment {
    /**
     * Create a new environment.
     */
    pub fn new(github_api_key: &String) -> Self {
        let stdin = stdin();
        let stdout = stdout();

        Environment {
            github_api_key: github_api_key.clone(),
            stdin,
            stdout,
        }
    }

    /**
     * Get the github api key
     */
    pub fn github_key(&self) -> &String {
        &self.github_api_key
    }

    /**
     * Get Stdin
     */
    pub fn stdin(&self) -> &Stdin {
        &self.stdin
    }

    /**
     * Get Stdout
     */
    pub fn stdout(&self) -> &Stdout {
        &self.stdout
    }
}
