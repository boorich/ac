#![feature(type_ascription)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use reqwest::{self};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    #[derive(Deserialize, Debug)]
    struct Language {
        TypeScript: Option<i32>,
        Python: Option<i32>,
        JavaScript: Option<i32>,
        Rust: Option<i32>,
        Go: Option<i32>,
        Java: Option<i32>,
        C: Option<i32>,
        Swift: Option<i32>,
        Kotlin: Option<i32>,
        Ruby: Option<i32>,
        Haskell: Option<i32>,
        Lua: Option<i32>,
        Lisp: Option<i32>,
        CSS: Option<i32>,
        HTML: Option<i32>,
        Shell: Option<i32>,
        Dockerfile: Option<i32>,
        Mustache: Option<i32>,
    }

    impl Default for Language {
        fn default() -> Self {
            Language {
                TypeScript: None,
                Python: None,
                JavaScript: None,
                Rust: None,
                Go: None,
                Java: None,
                C: None,
                Swift:  None,
                Kotlin: None,
                Ruby: None,
                Haskell: None,
                Lua: None,
                Lisp: None,
                CSS: None,
                HTML: None,
                Shell: None,
                Dockerfile: None,
                Mustache: None,
            }
        }
    }

    
    // naming the client after the app
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    
    // some constants for debugging
    const URL1: &str = "https://api.github.com/repos/clearloop/allblue/languages";
    const URL2: &str = "https://api.github.com/repos/chainsafe/integrations/languages";
    const URL3: &str = "https://api.github.com/repos/empea-careercriminal/languages";
    
    // build the client
    let client = reqwest::Client::builder()
    .user_agent(APP_USER_AGENT)
    .build();
    
    // Create a Default Language struct
    let mut response: Language = Language::default();

    // type ascirption might be a bit overkill here
    response = client
        .expect("Didn´t work")
        .get(URL2)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    // print!("Response: {}", response.Python);
    println!("Output: {:?}", response);
}
