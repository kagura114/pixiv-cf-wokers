# Pixiv Proxy for CF Workers


A proxy let you use images from pixiv without pain, with [`workers-rs`](https://github.com/cloudflare/workers-rs).

## Deploy

[![Deploy with Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/kagura114/pixiv-cf-wokers)


## Development

Clone this repo, then `npx wrangler dev` to build and start a local server, you have to **turn off** local mode in your local server.\
To deploy on your cloudflare account, change the `name` field, which is the name of your worker, in `wrangler.toml` and then use `npx wrangler deploy` to upload.

## Usage
### General
For every image in pixiv, jpg/png files are accessed under `i.pximg.net`, and need a valid `REFERER` to access data. So we add referer and then proxy our request to the original server.\
So the only thing users need to do is to change `i.pximg.net` to `your.worker.domain` and everything will go.

### PID API: /pid/{pid}/{page}
`pid` is the pid of the gallary\
`page` is the page (e.g. `_p0`) of the gallary\
As pixiv blocks cloudflare workers from `pixiv.net` and proxying is not easy on wasm, another proxy is needed for getting data from pixiv.\
To enable this, put `host` and `auth` into your worker's [secret](https://developers.cloudflare.com/workers/cli-wrangler/commands#secret), usually use `wrangler secret put <KEY>` and prompt your secret \
`host` is the `hostname` of your proxy, e.g. `p.example.com`, do not add scheme\
`auth` is the `secret token` of your proxy, e.g. `123456`, don't leave blank!\
Also here is an example of your nginx config
```
server {
	listen [::]:443 ssl;
	listen 443 ssl;

	server_name <hostname>;
	resolver 8.8.8.8;	

	ssl_certificate <your cert>;
	ssl_certificate_key <your key>;

	location / {
	        proxy_set_header Host www.pixiv.net;
	        proxy_set_header Referer "https://www.pixiv.net";
	        proxy_pass https://www.pixiv.net;
	        proxy_ssl_server_name on;
	        proxy_set_header Accept-Encoding "";
		
            if ($http_authorization != "Bearer <secret token>") {
                return 401;
            }

	        sub_filter_once off;
	        sub_filter_types *;
		    client_max_body_size 8000m;
	}
}
```
## Useful hints
For more help in reqwest, see [docs.rs/reqwest](https://docs.rs/reqwest/latest/reqwest/)

## Known Problems
- PID API cannot get image url if the content is R18

## Issues
If you have any problems with this, you are welcome to leave an issue.