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
      .post(&format!("{}{}", test_resources.base_url, "/private/poll"))
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
      .post(&format!("{}{}{}{}", test_resources.base_url, "/private/poll/", create_poll_resource.poll.id, "/proposal"))
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
      .post(&format!("{}{}{}{}", test_resources.base_url, "/private/poll/", create_poll_resource.poll.id, "/invite-user"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .json(&invite_user_payload)
      .send()
      .unwrap();

    assert_eq!(invite_user_response.status(), 200);

    // TODO try to vote without starting poll check 403
    // TODO try starting already started poll check 404
    let start_poll_response: reqwest::Response = test_resources
      .http_client
      .put(&format!("{}{}{}{}", test_resources.base_url, "/private/poll/", create_poll_resource.poll.id, "/start-poll"))
      .header("Authorization", utils::DEBUG_TOKEN_1)
      .json(&invite_user_payload)
      .send()
      .unwrap();

    assert_eq!(start_poll_response.status(), 200);
}