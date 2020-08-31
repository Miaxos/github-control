use crate::application::configuration::config_file::ApplicationConfiguration;
use anyhow::*;
use graphql_client::*;

type URI = String;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "github_schema.json",
    query_path = "src/infrastructure/github/getStatus.graphql",
    response_derives = "Debug"
)]
struct ViewTest;

/**
 * Get PRs from github
 * Should be rewrite with a Encoder/Decoder/Codec.
 */
pub fn get_prs_from_github(
    cfg: &ApplicationConfiguration,
) -> Result<Vec<(String, String)>, anyhow::Error> {
    let api = (*cfg).github_key();
    let q = ViewTest::build_query(view_test::Variables { first: 20 });
    let client = reqwest::blocking::Client::new();

    let res = client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("bearer {}", api))
        .header("User-Agent", "miaxos/github-control")
        .json(&q)
        .send()?;

    let response_body: Response<view_test::ResponseData> = res.json()?;
    let response_data: view_test::ResponseData = match response_body.data {
        Some(response) => response,
        None => {
            return Err(anyhow!("No responses"));
        }
    };

    let mut result: Vec<(String, String)> = vec![];

    for edge_opt in &response_data
        .viewer
        .pull_requests
        .edges
        .expect("missing data")
    {
        if let Some(edge) = edge_opt {
            if let Some(node) = &edge.node {
                if let (Some(commits), review_decision) =
                    (&node.commits.nodes, &node.review_decision)
                {
                    if let Some(Some(commit)) = &commits.first() {
                        if let Some(status) = &commit.commit.status_check_rollup {
                            let value = match &status.state {
                                view_test::StatusState::SUCCESS => &(cfg).ci_success,
                                view_test::StatusState::ERROR => &(cfg).ci_error,
                                view_test::StatusState::FAILURE => &(cfg).ci_failure,
                                view_test::StatusState::EXPECTED => &(cfg).ci_expected,
                                view_test::StatusState::PENDING => &(cfg).ci_pending,
                                view_test::StatusState::Other(_) => &(cfg).ci_other,
                            };

                            let v = match &review_decision {
                                Some(view_test::PullRequestReviewDecision::APPROVED) => {
                                    &cfg.review_approved
                                }
                                Some(view_test::PullRequestReviewDecision::REVIEW_REQUIRED) => {
                                    &cfg.review_required
                                }
                                Some(view_test::PullRequestReviewDecision::CHANGES_REQUESTED) => {
                                    &cfg.review_changes_requested
                                }
                                Some(view_test::PullRequestReviewDecision::Other(_)) => {
                                    &cfg.review_other
                                }
                                None => &cfg.review_no_required,
                            };
                            result.push((
                                format!("{:?} - [Review: {:?}] [CI: {}]", &node.title, v, value),
                                node.url.to_owned(),
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(result)
}
