use newsapi_rs::client::NewsApiClient;
use newsapi_rs::error::ApiClientError;
use newsapi_rs::model::{GetTopHeadlinesRequest, NewsCategory};

/// This example requires the "blocking" feature to be enabled
/// Run with: cargo run --example top_headlines --features blocking
#[cfg(feature = "blocking")]
fn main() {
    dotenvy::dotenv().ok();

    let client = NewsApiClient::builder_blocking()
        .build()
        .expect("Failed to build NewsApiClient");

    let request = GetTopHeadlinesRequest::builder()
        .category(NewsCategory::Business)
        .search_term(String::from("china"))
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
