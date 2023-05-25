use axum::{extract::State, response::IntoResponse};
use hyper::StatusCode;
use reqwest::Client;


async fn get_url_bytes(client: &Client, url: &str) -> Result<usize, reqwest::Error> {
    let res = client.get(url).send().await?;
    let bytes = res.bytes().await?;
    Ok(bytes.len())
}


// issues 2 http request concurrently and returns the sum of the response bodies
pub async fn count_bytes(State(client): State<Client>) -> impl IntoResponse {

    let url1 = "https://www.rust-lang.org/";
    let url2 = "https://www.rust-lang.org/learn";

    let fut1 = get_url_bytes(&client, url1);
    let fut2 = get_url_bytes(&client, url2);

    let (res1, res2) = futures::join!(fut1, fut2);

    let sum = res1.unwrap() + res2.unwrap();

    (StatusCode::OK, format!("Total bytes: {}", sum))
}
