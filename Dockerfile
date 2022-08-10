FROM alpine:3.16
LABEL maintainers="rv178"

WORKDIR /opt/app
COPY . .

RUN apk add --no-cache \
		ca-certificates \
		gcc \
		curl

ENV RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	PATH=/usr/local/cargo/bin:$PATH \
	RUST_VERSION=1.62.1

RUN set -eux; \
	apkArch="$(apk --print-arch)"; \
	case "$apkArch" in \
		x86_64) rustArch='x86_64-unknown-linux-musl'; rustupSha256='bdf022eb7cba403d0285bb62cbc47211f610caec24589a72af70e1e900663be9' ;; \
		aarch64) rustArch='aarch64-unknown-linux-musl'; rustupSha256='89ce657fe41e83186f5a6cdca4e0fd40edab4fd41b0f9161ac6241d49fbdbbbe' ;; \
		*) echo >&2 "unsupported architecture: $apkArch"; exit 1 ;; \
	esac; \
	url="https://static.rust-lang.org/rustup/archive/1.24.3/${rustArch}/rustup-init"; \
	wget "$url"; \
	echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
	chmod +x rustup-init; \
	./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
	rm rustup-init; \
	chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
	rustup --version; \
	cargo --version; \
	rustc --version;

RUN	rustup target add wasm32-unknown-unknown; \
	cargo --verbose install wasm-bindgen-cli;

RUN	curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64; \
	chmod +x tailwindcss-linux-x64; \
	mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss; \
	curl -L https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-; \
	mv trunk /usr/local/bin/trunk; \
	cd frontend;

cmd ["trunk", "serve"]
