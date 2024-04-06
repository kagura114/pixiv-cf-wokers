use worker::*;
use reqwest::header::REFERER;

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let mut endpoint = req.url()?;
    endpoint.set_host(Some("i.pximg.net"))?;

    let client = reqwest::Client::new();
    let data =
        client.get(endpoint)
        .header(REFERER, "https://www.pixiv.net/")
        .send()
        .await;

    let mut headers = Headers::new();
    headers.set("Content-Disposition", "inline")?;

    let bytes = match data {
        Ok(r) => r.bytes(),
        Err(e) => return Response::error(e.to_string(), e.status().unwrap().as_u16())
    } .await;

    let response =  Response::from_bytes(bytes.unwrap().to_vec());
    match response {
        Ok(r) => Ok(r.with_headers(headers)),
        Err(e) => Err(e)
    }
}
