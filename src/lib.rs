mod ffi;

#[cfg(feature = "jni")]
mod java;

const URL: &'static str = "https://api.todoist.com";
const API_VERSION: &'static str = "/sync/v8";

pub fn add(token: String, content: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = URL.to_string();

    url.push_str(API_VERSION);
    url.push_str("/quick/add");

    let params = [("text", content)];
    let bearer_token = "Bearer ".to_string() + &token;

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url.as_str())
        .header("Authorization", bearer_token)
        .query(&params)
        .send()?;

    return Ok(res.text().unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
