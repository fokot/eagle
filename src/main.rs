use axum::{response::Html, routing::get, Router};
use serde;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
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
    // vec!["https://httpbin.org/ip".to_string()]
    vec![
        "http://localhost:8000/test.json".to_string(),
        "http://localhost:8000/test2.json".to_string(),
        "http://localhost:8000/test3.json".to_string(),
        "http://localhost:8765/test4.json".to_string(),
    ]
}

// FIXME zmenit mapu na nieco co drzi poradie
// call the urls in parallel using tokio and get the data and store it into a map
async fn get_data(urls: Vec<String>) -> HashMap<String, Result<VersionInfo, String>> {
    // standard library Mutex would work to
    let results: Arc<Mutex<HashMap<String, Result<VersionInfo, String>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // Create a vector to store the tasks with size of urls
    let mut tasks = Vec::with_capacity(urls.len());

    // Iterate over the URLs
    for url in urls {
        // Clone the Arc for each task
        let results_clone = Arc::clone(&results);

        // Spawn a new task
        let task = tokio::spawn(async move {
            // Perform the HTTP request or any other operation
            // and store the result in the HashMap
            let result = perform_request(&url).await;
            results_clone.lock().await.insert(url.to_string(), result);
        });

        // Store the task in the vector
        tasks.push(task);
    }

    // Wait for all tasks to finish
    for task in tasks {
        task.await.unwrap();
    }

    // // Print the results
    // for (url, result) in results.lock().await.iter() {
    //     println!("{}: {}", url, result);
    // }

    // FIXME preco tu musim let
    let x = results.lock().await.clone();
    x
}

// FIXME pridany Clone, mozno sa da zmazat
#[derive(serde::Deserialize, PartialEq, Clone)]
pub struct VersionInfo {
    id: i32,
    text: String,
}

async fn perform_request(url: &String) -> Result<VersionInfo, String> {
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
