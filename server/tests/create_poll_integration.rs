extern crate reqwest;
extern crate qv;
mod utils;

#[test]
fn create_poll_integration() {
    let test_resources = utils::build_test_resources();

    let user_info_resource: qv::models::UserInfoResource = test_resources
      .http_client
      .get(&format!("{}{}", test_resources.base_url, "/private/user-info"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .send()
      .unwrap()
      .json()
      .unwrap();

    assert_eq!(user_info_resource.user.email_verified, Option::Some(true));
    
    let create_poll_payload = qv::models::CreatePollPayload {
      title: "test-poll".to_string(),
      poll_type: "qv".to_string(),
    };

    let create_poll_resource: qv::models::CreatePollResource = test_resources
      .http_client
      .post(&format!("{}{}", test_resources.base_url, "/private/polls"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .json(&create_poll_payload)
      .send()
      .unwrap()
      .json()
      .unwrap();

    assert_eq!(create_poll_resource.poll.email, user_info_resource.user.email);

    assert_eq!(create_poll_resource.poll.current_progress, qv::sql_enum_types::ProgressEnum::NotStarted);

    let create_proposal_payload = qv::models::CreateProposalPayload {
      summary: "My special proposal.".to_string(),
      full_description_link: Option::Some("https://proposal-website.com".to_string()),
    };

    let create_proposal_resource: qv::models::CreateProposalResource = test_resources
      .http_client
      .post(&format!("{}{}{}{}", test_resources.base_url, "/private/polls/", create_poll_resource.poll.id, "/proposals"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .json(&create_proposal_payload)
      .send()
      .unwrap()
      .json()
      .unwrap();

    assert_eq!(create_proposal_resource.proposal.poll_id, create_poll_resource.poll.id);

    // TODO try to vote without user invite check 403
    let invite_user_payload = qv::models::InviteUserPayload {
      email: "fake_2@email.com".to_string()
    };

    let invite_user_response: reqwest::Response = test_resources
      .http_client
      .post(&format!("{}{}{}{}", test_resources.base_url, "/private/polls/", create_poll_resource.poll.id, "/invite-user"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .json(&invite_user_payload)
      .send()
      .unwrap();

    assert_eq!(invite_user_response.status(), 200);

    let invite_self_payload = qv::models::InviteUserPayload {
      email: "fake_1@email.com".to_string()
    };

    let invite_self_response: reqwest::Response = test_resources
      .http_client
      .post(&format!("{}{}{}{}", test_resources.base_url, "/private/polls/", create_poll_resource.poll.id, "/invite-user"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .json(&invite_self_payload)
      .send()
      .unwrap();

    assert_eq!(invite_self_response.status(), 200);

    // TODO try to vote without starting poll check 403
    // TODO try starting already started poll check 400
    let start_poll_response: reqwest::Response = test_resources
      .http_client
      .put(&format!("{}{}{}{}", test_resources.base_url, "/private/polls/", create_poll_resource.poll.id, "/start"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .json(&invite_user_payload)
      .send()
      .unwrap();

    assert_eq!(start_poll_response.status(), 200);

    let create_vote_payload1 = qv::models::CreateVotePayload {
      points: 9.0
    };

    let vote_response1: reqwest::Response = test_resources
      .http_client
      .put(&format!("{}{}{}{}", test_resources.base_url, "/private/proposals/", create_proposal_resource.proposal.id, "/vote"))
      .header("Authorization", utils::DEBUG_TOKEN_2)
      .json(&create_vote_payload1)
      .send()
      .unwrap();

    assert_eq!(vote_response1.status(), 200);

    let create_vote_payload2 = qv::models::CreateVotePayload {
      points: -4.0
    };

    let vote_response2: reqwest::Response = test_resources
      .http_client
      .put(&format!("{}{}{}{}", test_resources.base_url, "/private/proposals/", create_proposal_resource.proposal.id, "/vote"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .json(&create_vote_payload2)
      .send()
      .unwrap();

    assert_eq!(vote_response2.status(), 200);

    let mut home_response: reqwest::Response = test_resources
      .http_client
      .get(&format!("{}{}", test_resources.base_url, "/private/home"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .send()
      .unwrap();

    assert_eq!(home_response.status(), 200);

    let home_resource: qv::models::HomeResource = home_response.json().unwrap();

    assert!(home_resource.polls.len() >= 2);
    assert!(home_resource.polls.iter().cloned().find(|p| p.id == create_poll_resource.poll.id).is_some());

    let finish_response: reqwest::Response = test_resources
      .http_client
      .put(&format!("{}{}{}{}", test_resources.base_url, "/private/polls/", create_poll_resource.poll.id, "/finish"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .send()
      .unwrap();

    assert_eq!(finish_response.status(), 200);

    let mut finished_get_poll_response: reqwest::Response = test_resources
      .http_client
      .get(&format!("{}{}{}", test_resources.base_url, "/private/polls/", create_poll_resource.poll.id))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .send()
      .unwrap();

    assert_eq!(finished_get_poll_response.status(), 200);

    let finished_get_poll_resource: qv::models::GetPollResource = finished_get_poll_response.json().unwrap();

    assert_eq!(finished_get_poll_resource.point_totals.is_some(), true);
    assert_eq!(finished_get_poll_resource.point_totals.unwrap().get(&create_proposal_resource.proposal.id).unwrap(), &1.0);

    assert_eq!(finished_get_poll_resource.proposals.is_some(), true);
    assert_eq!(finished_get_poll_resource.proposals.unwrap().len(), 1);
}