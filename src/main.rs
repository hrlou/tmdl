use std::{vec, collections::BTreeMap};
use json::JsonValue;
use rhai::{Engine, Scope, Dynamic, Map, serde::to_dynamic};
// use serde_json::Value;
// use json::
// use rhai::serf
use futures::executor::block_on;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new();
    let mut scope = Scope::new();

    scope.push_constant("URL", "https://danbooru.donmai.us/posts/5201568.json");

    let get = |url: String| -> String {
        let r = block_on(reqwest::get(url)).unwrap();
        let r = block_on(r.text()).unwrap();
        r
    };

    let parse = |s: String| -> Map {
        // this is kinda hacky, I'm sure there is a better method
        let tool = Engine::new();
        let r: Map = tool.parse_json(s, true).unwrap();
        r
    };

    engine
        .register_fn("parse", parse)
        .register_fn("get", get);
    // engine.parse_json(json, has_null)

    engine.run_file_with_scope(&mut scope, "scripts/danbooru.rhai".into())?;

    // let body = reqwest::get("https://www.rust-lang.org").await?.text().await?;
    // println!("body = {:?}", body);


    Ok(())
}