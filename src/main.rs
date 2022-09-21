use reqwest::{self};
use serde::Deserialize;

#[tokio::main]
async fn main() {

    #[derive(Deserialize, Debug)]
    struct Language {
        Python: i32,
        Rust: i32,
        JavaScript: i32,
        Go: i32,
        Java: i32,
        C: i32,
        TypeScript: i32,
        Swift: i32,
        Kotlin: i32,
        Ruby: i32,
        Haskell: i32,
        Lua: i32,
        Lisp: i32,
    }

    impl Default for Language {
        fn default() -> Self {
            Language {
                Python: 0,
                Rust: 0,
                JavaScript: 0,
                Go: 0,
                Java: 0,
                C: 0,
                TypeScript: 0,
                Swift: 0,
                Kotlin: 0,
                Ruby: 0,
                Haskell: 0,
                Lua: 0,
                Lisp: 0,
            }
        }
    }

    // Create a Default Language struct
    let mut languages = Language::default();
    println!("{:?}", languages);

    // naming the client after your app?
    static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    );

    // with client
    const URL1: &str = "https://api.github.com/repos/clearloop/allblue/languages";
    const URL2: &str = "http://httpbin.org/ip";
    const URL3: &str = "https://api.github.com/repos/empea-careercriminal/languages";
    
    // build the client
    let client = reqwest::Client::builder()
    .user_agent(APP_USER_AGENT)
    .build();

    let response = languages;
    let response: Language = client.expect("Didn´t work").get(URL1)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    
    print!("Response: {}", response.Python);

}
