# Pixiv Proxy for CF Workers


A proxy let you use images from pixiv without pain, with [`workers-rs`](https://github.com/cloudflare/workers-rs).

## Deploy

[![Deploy with Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/kagura114/pixiv-cf-wokers)


## Development

Clone this repo, then `npx wrangler dev` to build and start a local server, you have to **turn off** local mode in your local server.\
To deploy on your cloudflare account, change the `name` field, which is the name of your worker, in `wrangler.toml` and then use `npx wrangler deploy` to upload.

## Usage
For every image in pixiv, jpg/png files are accessed under `i.pximg.net`, and need a valid `REFERER` to access data. So we add referer and then proxy our request to the original server.\
So the only thing users need to do is to change `i.pximg.net` to `your.worker.domain` and everything will go.

## Useful hints
For more help in reqwest, see [docs.rs/reqwest](https://docs.rs/reqwest/latest/reqwest/)

## Issues
If you have any problems with this, you are welcome to leave an issue.