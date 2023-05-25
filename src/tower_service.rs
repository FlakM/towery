use std::convert::Infallible;

use futures::future::BoxFuture;
use hyper::{Body, Request, StatusCode};
use reqwest::Client;
use tower::Service;

#[derive(Clone)]
pub struct CountBytesService {
    client: Client,
}

impl CountBytesService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

async fn get_url_bytes(client: &Client, url: &str) -> Result<usize, reqwest::Error> {
    let res = client.get(url).send().await?;
    let bytes = res.bytes().await?;
    Ok(bytes.len())
}

impl Service<Request<Body>> for CountBytesService {
    type Response = (StatusCode, String);
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let client = self.client.clone();
        let fut = async move {
            let fut1 = get_url_bytes(&client, "https://www.rust-lang.org/");
            let fut2 = get_url_bytes(&client, "https://www.rust-lang.org/learn");

            let (res1, res2) = futures::join!(fut1, fut2);

            // TODO: handle errors
            let sum = res1.unwrap() + res2.unwrap();

            Ok((StatusCode::OK, format!("Total bytes: {}", sum)))
        };
        Box::pin(fut)
    }
}
