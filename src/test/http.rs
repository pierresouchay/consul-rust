use curl::http;

#[test]
pub fn test_simple_get() {
  let resp = http::handle()
    .get("http://www.zhihu.com")
    .exec().unwrap();

  println!("code={}; headers={}; body={}",
        resp.get_code(), resp.get_headers(), resp.get_body());

  assert!(resp.get_code() == 200, "code is {}", resp.get_code());
}
