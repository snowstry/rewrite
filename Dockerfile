FROM rust:1.31
LABEL maintainers="rv178"

WORKDIR /opt/app
COPY . .

#RUN echo "{\n\t\"dns\": [\"8.8.8.8\", \"8.8.4.4\"]\n}" >> /etc/docker/daemon.json

RUN	rustup target add wasm32-unknown-unknown; \
	cargo install trunk wasm-bindgen-cli;

RUN	curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64; \
	chmod +x tailwindcss-linux-x64; \
	mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss; \
	cd frontend;

cmd ["trunk", "serve"]
