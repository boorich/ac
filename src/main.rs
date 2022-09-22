#![allow(non_snake_case)]
#![allow(dead_code)]

use reqwest::{self};
use serde::{Deserialize, Serializer, ser::SerializeStruct};
use serde_json;

#[tokio::main]
async fn main() {
    #[derive(Deserialize, Debug)]
    struct Language {
        TypeScript: Option<i32>,
        Solidity: Option<i32>,
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

    trait Extract {
        fn extract_to_vec(&self) -> Vec<i32>;
        fn extract_to_map(&self) -> Vec<(String, i32)>;
    }

    impl Extract for Language {
        fn extract_to_vec(&self) -> Vec<i32> {
            vec![
                self.TypeScript.unwrap_or(0),
                self.Solidity.unwrap_or(0),
                self.Python.unwrap_or(0),
                self.JavaScript.unwrap_or(0),
                self.Rust.unwrap_or(0),
                self.Go.unwrap_or(0),
                self.Java.unwrap_or(0),
                self.C.unwrap_or(0),
                self.Swift.unwrap_or(0),
                self.Kotlin.unwrap_or(0),
                self.Ruby.unwrap_or(0),
                self.Haskell.unwrap_or(0),
                self.Lua.unwrap_or(0),
                self.Lisp.unwrap_or(0),
                self.CSS.unwrap_or(0),
                self.HTML.unwrap_or(0),
                self.Shell.unwrap_or(0),
                self.Dockerfile.unwrap_or(0),
                self.Mustache.unwrap_or(0),
            ]
        }
        fn extract_to_map(&self) -> Vec<(String, i32)> {
            vec![
                ("TypeScript".to_string(), self.TypeScript.unwrap_or(0)),
                ("Solidity".to_string(), self.TypeScript.unwrap_or(0)),
                ("Python".to_string(), self.Python.unwrap_or(0)),
                ("JavaScript".to_string(), self.JavaScript.unwrap_or(0)),
                ("Rust".to_string(), self.Rust.unwrap_or(0)),
                ("Go".to_string(), self.Go.unwrap_or(0)),
                ("Java".to_string(), self.Java.unwrap_or(0)),
                ("C".to_string(), self.C.unwrap_or(0)),
                ("Swift".to_string(), self.Swift.unwrap_or(0)),
                ("Kotlin".to_string(), self.Kotlin.unwrap_or(0)),
                ("Ruby".to_string(), self.Ruby.unwrap_or(0)),
                ("Haskell".to_string(), self.Haskell.unwrap_or(0)),
                ("Lua".to_string(), self.Lua.unwrap_or(0)),
                ("Lisp".to_string(), self.Lisp.unwrap_or(0)),
                ("CSS".to_string(), self.CSS.unwrap_or(0)),
                ("HTML".to_string(), self.HTML.unwrap_or(0)),
                ("Shell".to_string(), self.Shell.unwrap_or(0)),
                ("Dockerfile".to_string(), self.Dockerfile.unwrap_or(0)),
                ("Mustache".to_string(), self.Mustache.unwrap_or(0)),
            ]
        }

    }

    impl Default for Language {
        fn default() -> Self {
            Language {
                TypeScript: None,
                Solidity: None,
                Python: None,
                JavaScript: None,
                Rust: None,
                Go: None,
                Java: None,
                C: None,
                Swift: None,
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

    impl serde::Serialize for Language {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("Language", 18)?;
            SerializeStruct::serialize_field(&mut state, "TypeScript", &self.TypeScript)?;
            state.serialize_field("Python", &self.Python)?;
            state.serialize_field("Solidity", &self.Solidity)?;
            state.serialize_field("JavaScript", &self.JavaScript)?;
            state.serialize_field("Rust", &self.Rust)?;
            state.serialize_field("Go", &self.Go)?;
            state.serialize_field("Java", &self.Java)?;
            state.serialize_field("C", &self.C)?;
            state.serialize_field("Swift", &self.Swift)?;
            state.serialize_field("Kotlin", &self.Kotlin)?;
            state.serialize_field("Ruby", &self.Ruby)?;
            state.serialize_field("Haskell", &self.Haskell)?;
            state.serialize_field("Lua", &self.Lua)?;
            state.serialize_field("Lisp", &self.Lisp)?;
            state.serialize_field("CSS", &self.CSS)?;
            state.serialize_field("HTML", &self.HTML)?;
            state.serialize_field("Shell", &self.Shell)?;
            state.serialize_field("Dockerfile", &self.Dockerfile)?;
            SerializeStruct::serialize_field(&mut state, "Mustache", &self.Mustache)?;
            state.end()
        }
    }

    // naming the client after the app
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    // some constants for fetching different repo's data
    const URL1: &str = "https://api.github.com/repos/clearloop/allblue/languages";
    const URL2: &str = "https://api.github.com/repos/chainsafe/integrations/languages";
    const URL3: &str = "https://api.github.com/repos/empea-careercriminal/concierge/languages";
    const URL4: &str = "https://api.github.com/repos/paritytech/substrate/languages";
    const URL5: &str = "https://api.github.com/repos/lastperson/vector-withdraw-helpers/languages";

    // build the client
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build();

    // Create a Default Language struct
    let mut response: Language = Language::default();

    // fetch the data from the repo
    response = client
        .expect("Didn´t work")
        .get(URL1)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    // debug print
    println!("{:?}", response);
    // println!("Output: {:?}", response.extract_to_vec());
    // println!("Output: {:?}", response.extract_to_map());

    // create folder structure
    match std::fs::create_dir_all("json") {
        Ok(it) => it,
        Err(err) => panic!("Error creating directory: {}", err),
    };
    // Generate Json
    let output_path = format!("json/output.json");
    let output_file = std::fs::File::create(output_path).unwrap();
    let mut output_writer = std::io::BufWriter::new(output_file);
    serde_json::to_writer_pretty(&mut output_writer, &response).unwrap();

}
