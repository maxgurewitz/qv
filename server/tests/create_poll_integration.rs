extern crate reqwest;
extern crate qv;
mod utils;

#[test]
fn create_poll_integration() {
    let test_resources = utils::build_test_resources();

    let user_info_resource: qv::models::UserInfoResource = test_resources
      .http_client
      .get(&format!("{}{}", test_resources.base_url, "/private/user-info"))
      .header("Authorization", utils::DEBUG_TOKEN)
      .send()
      .unwrap()
      .json()
      .unwrap();


    assert_eq!(user_info_resource.user.email_verified, Option::Some(true));
    
    let create_poll_payload = qv::models::CreatePollPayload {
      title: "test-poll".to_string(),
      poll_type: "qv".to_string(),
    };

    // let poll: qv::models::Poll = test_resources
    //   .http_client
    //   .post(&format!("{}{}", test_resources.base_url, "/private/poll"))
    //   .header("Authorization", utils::DEBUG_TOKEN)
    //   .json(&create_poll_payload)
    //   .send()
    //   .unwrap()
    //   .json()
    //   .unwrap();

    // assert_eq!(poll.email, user_info.email);
    // assert_eq!(poll.current_progress, qv::schema::ProgressEnum::NotStarted);
}