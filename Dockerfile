FROM rust:1.62-alpine
LABEL maintainers="rv178"

WORKDIR /opt/snowstry
COPY . .

RUN	rustup target add wasm32-unknown-unknown; \
	cargo --verbose install wasm-bindgen-cli;

RUN	curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64; \
	chmod +x tailwindcss-linux-x64; \
	mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss; \
	curl -L https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-; \
	mv trunk /usr/local/bin/trunk; \
	cd frontend;

cmd ["trunk", "serve"]
