use reqwest;
use std::env;

pub static DEBUG_TOKEN_1: &str = "Bearer debug_token_1";
pub static DEBUG_TOKEN_2: &str = "Bearer debug_token_2";

pub struct TestResources {
  pub http_client: reqwest::Client,
  pub base_url: String
}

pub fn build_test_resources() -> TestResources {
  TestResources { 
    http_client: reqwest::Client::new(),
    base_url: env::var("BASE_URL").unwrap()
  }
}