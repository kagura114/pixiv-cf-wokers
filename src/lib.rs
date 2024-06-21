use regex::Regex;
use reqwest::{header::REFERER, Client, StatusCode};
use worker::*;

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if req.path().starts_with("/pid") {
        get_image_by_pid(req).await
    } else if req.path().starts_with("/img-original") {
        get_image_by_url(req).await
    } else {
        return Response::ok("Hello World");
    }
}

async fn get_image_by_url(req: Request) -> Result<Response> {
    let mut endpoint = req.url()?;
    endpoint.set_host(Some("i.pximg.net"))?;

    let client = reqwest::Client::new();
    let data = client
        .get(endpoint)
        .header(REFERER, "https://www.pixiv.net/")
        .send()
        .await;

    let mut headers = Headers::new();
    headers.set("Content-Disposition", "inline")?;

    let bytes = match data {
        Ok(r) => r.bytes(),
        Err(e) => return Response::error(e.to_string(), e.status().unwrap().as_u16()),
    }
    .await;

    let response = Response::from_bytes(bytes.unwrap().to_vec());
    match response {
        Ok(r) => Ok(r.with_headers(headers)),
        Err(e) => Err(e),
    }
}

async fn get_image_by_pid(req: Request) -> Result<Response> {
    let pid = match req.path().split('/').last() {
        Some(p) => p.parse::<i64>().unwrap(),
        None => {
            return Response::error(
                "Failed to parse pid from your request",
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            )
        }
    };
    let client = Client::new();
    let data = client
        .get(format!("https://www.pixiv.net/artworks/{pid}"))
        .header(REFERER, "https://www.pixiv.net/")
        .send()
        .await;

    let response = match data {
        Ok(page) => match page.text().await {
            Ok(text) => text,
            Err(e) => return Response::error(e.to_string(), e.status().unwrap().as_u16()),
        },
        Err(e) => return Response::error(e.to_string(), e.status().unwrap().as_u16()),
    };

    let re = Regex::new(r#""original":"(https://i\.pximg\.net/img-original/[^"]*)""#).unwrap();

    if let Some(captures) = re.captures(response.as_str()) {
        if let Some(url) = captures.get(1) {
            return Ok(
                get_image_by_url(Request::new(url.as_str(), Method::Post).unwrap())
                    .await
                    .unwrap(),
            );
        }
    }

    Response::error(
        format!("Failed to get img source for request\nresponse:\n{response}"),
        StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    )
}
