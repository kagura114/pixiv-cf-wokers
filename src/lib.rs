use regex::Regex;
use reqwest::{
    header::{AUTHORIZATION, REFERER},
    Client, StatusCode,
};
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.path().starts_with("/pid") {
        get_image_by_pid(req, env).await
    } else if req.path().starts_with("/img-original") {
        get_image_by_url(req, env).await
    } else {
        return Response::ok("Hello World");
    }
}

async fn get_image_by_url(req: Request, env: Env) -> Result<Response> {
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

    let allowed_origin = match env.secret("allow-origin") {
        Ok(origin) => origin.to_string(),
        Err(_) => "*".to_string(),
    };

    headers.set("Access-Control-Allow-Origin", allowed_origin.as_str())?;

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

async fn get_image_by_pid(req: Request, env: Env) -> Result<Response> {
    let mut pid: i64 = -1;
    let mut page: i64 = 0; // {pid}_p{page}.#ext

    req.path().split('/').for_each({
        |sp| match sp.parse::<i64>() {
            Ok(p) => {
                if pid == -1 {
                    pid = p
                } else {
                    page = p
                }
            }
            Err(_) => (),
        }
    });

    if pid == -1 {
        return Response::error(
            "Failed to parse pid from your request",
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        );
    }

    let host = match env.secret("host") {
        Ok(h) => h.to_string(),
        Err(_) => {
            return Response::error("Not available.", StatusCode::SERVICE_UNAVAILABLE.as_u16())
        }
    };

    let auth = match env.secret("auth") {
        Ok(a) => a.to_string(),
        Err(_) => {
            return Response::error("Not available.", StatusCode::SERVICE_UNAVAILABLE.as_u16())
        }
    };

    let data = Client::new()
        .get(format!("https://{host}/artworks/{pid}"))
        .header(AUTHORIZATION, format!("Bearer {auth}"))
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
            return Ok(get_image_by_url(
                Request::new(
                    url.as_str()
                        .replace("_p0", format!("_p{page}").as_str())
                        .as_str(),
                    Method::Post,
                )
                .unwrap(),
                env
            )
            .await
            .unwrap());
        }
    }

    Response::error(response, StatusCode::INTERNAL_SERVER_ERROR.as_u16())
}
