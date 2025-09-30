FROM rust:1.89-alpine AS builder


RUN apk add --no-cache build-base openssl-dev pkgconfig

RUN cargo install trunk --locked

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

RUN rm -f target/release/deps/hangar_front*

COPY . .

RUN trunk build --release


FROM nginx:alpine AS runner

COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]