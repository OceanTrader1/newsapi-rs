use env_logger::Env;
use newsapi_rs::client::NewsApiClient;
use newsapi_rs::constant::DEFAULT_LOG_LEVEL;
use newsapi_rs::error::ApiClientError;
use newsapi_rs::model::{GetTopHeadlinesRequest, NewsCategory};

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_LEVEL)).init();

    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Create client from environment variable
    let client = NewsApiClient::from_env();

    // Build top headlines request
    let request = GetTopHeadlinesRequest::builder()
        .category(NewsCategory::Business)
        .search_term(String::from("Nvidia Stock"))
        .page_size(5)
        .build();

    match client.get_top_headlines(&request.unwrap()) {
        Ok(response) => {
            println!("Total Results: {}", response.get_total_results());
            println!("Articles retrieved: {}", response.get_articles().len());

            for (i, article) in response.get_articles().iter().enumerate() {
                println!("Article #{}: {}", i + 1, article.get_title());
                println!("  Source: {}", article.get_source().get_name());
                println!("  URL: {}", article.get_url());
                println!();
            }
        }
        Err(err) => {
            eprintln!(
                "Error: {}",
                match err {
                    ApiClientError::InvalidResponse(response) => response.message.clone(),
                    _ => err.to_string(),
                }
            );
        }
    }
}
