use axum::{response::Html, routing::get, Router};
use serde;
use std::net::SocketAddr;

mod page;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/", get(app_endpoint))
                .into_make_service(),
        )
        .await
        .unwrap();
}

fn get_urls() -> Vec<String> {
    // let env_var = env::var("VERSION_URLS").expect("No VERSION_URLS environment variable");
    // env_var
    //     .split(',')
    //     .map(|s| s.trim().to_owned())
    //     .collect::<Vec<String>>()
    vec![
        "http://localhost:8000/test.json".to_string(),
        "http://localhost:8000/test2.json".to_string(),
        "http://localhost:8000/test3.json".to_string(),
        "http://localhost:8765/test4.json".to_string(),
    ]
}

type Version = Result<VersionInfo, String>;

// call the urls in parallel using tokio and get the data and store it into a map
async fn get_data(urls: Vec<String>) -> Vec<(String, Version)> {
    let mut tasks = tokio::task::JoinSet::new();
    let mut index: usize = 0;
    for url in urls {
        tasks.spawn(async move { (index, url.clone(), perform_request(&url).await) });
        index += 1;
    }
    let mut result = Vec::new();
    while let Some(res) = tasks.join_next().await {
        result.push(res.unwrap());
    }
    // keep the order of the urls
    result.sort_unstable_by_key(|(index, _, _)| *index);
    result
        .iter()
        .map(|(_, url, version)| (url.clone(), version.clone()))
        .collect()
}

// FIXME pridany Clone, mozno sa da zmazat
#[derive(serde::Deserialize, PartialEq, Clone)]
pub struct VersionInfo {
    id: i32,
    text: String,
}

async fn perform_request(url: &String) -> Version {
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;

    response
        .json::<VersionInfo>()
        .await
        .map_err(|e| e.to_string())
}

async fn app_endpoint() -> Html<String> {
    let data = get_data(get_urls()).await;
    Html(page::render(data))
}
