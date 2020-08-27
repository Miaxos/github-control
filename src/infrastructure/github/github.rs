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

pub fn get_truc(api: &String) -> Result<Vec<(String, String)>, anyhow::Error> {
    let q = ViewTest::build_query(view_test::Variables { first: 20 });
    let client = reqwest::blocking::Client::new();

    let mut res = client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("bearer {}", api))
        .header("User-Agent", "miaxos/github-control")
        .json(&q)
        .send()
        .unwrap();

    let response_body: Response<view_test::ResponseData> = res.json().unwrap();
    let response_data: view_test::ResponseData = response_body.data.expect("missing response data");

    let mut result: Vec<(String, String)> = vec![];

    for edge_opt in &response_data
        .viewer
        .pull_requests
        .edges
        .expect("missing data")
    {
        if let Some(edge) = edge_opt {
            if let Some(node) = &edge.node {
                if let (Some(commits), Some(review_decision)) =
                    (&node.commits.nodes, &node.review_decision)
                {
                    if let Some(Some(commit)) = &commits.first() {
                        if let Some(status) = &commit.commit.status_check_rollup {
                            let value = match &status.state {
                                view_test::StatusState::SUCCESS => "✅",
                                view_test::StatusState::ERROR => "❌",
                                view_test::StatusState::FAILURE => "⚠️",
                                view_test::StatusState::EXPECTED => "✴️",
                                view_test::StatusState::PENDING => "💤",
                                view_test::StatusState::Other(_) => "🤔",
                            };

                            let v = match &review_decision {
                                view_test::PullRequestReviewDecision::APPROVED => "✅",
                                view_test::PullRequestReviewDecision::REVIEW_REQUIRED => "👋",
                                view_test::PullRequestReviewDecision::CHANGES_REQUESTED => "🚫",
                                view_test::PullRequestReviewDecision::Other(_) => "🤔",
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
