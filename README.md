# Pixiv Proxy for CF Workers


A proxy let you use images from pixiv without pain, with [`workers-rs`](https://github.com/cloudflare/workers-rs).

## Deploy

[![Deploy with Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/kagura114/pixiv-cf-wokers)


## Development

Clone this repo, then `npx wrangler dev` to build and start a local server, you have to **turn off** local mode in your local server.  
To deploy on your cloudflare account, change the `name` field, which is the name of your worker, in `wrangler.toml` and then use `npx wrangler deploy` to upload.

## Useful hints

### Proxying
On building client, change the code to below to use proxies.
```Rust
let proxy = reqwest::Proxy::http("http://114.5.1.4");
let client = match proxy{
    Ok(p) => reqwest::Client::builder().proxy(p).build(),
    Err(_) => Ok(reqwest::Client::new()) // In this case we just do not use proxy
}.unwrap();
```
For more help, see [docs.rs/reqwest](https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html)
## Issues

If you have any problems with this, you are welcome to leave an issue.