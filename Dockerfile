# build stage
FROM rustlang/rust:nightly-slim as build

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# create new empty binary project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# build this project to cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

# add .env and secret.key for Docker env
RUN touch .env
RUN mv src/secret.key.sample src/secret.key

# rebuild app with project source
RUN rm ./target/release/deps/actix_web_rest_api_with_jwt*
RUN cargo build --release

# deploy stage
FROM debian:stretch-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/release/actix-web-rest-api-with-jwt .
COPY --from=build /app/.env .
COPY --from=build /app/diesel.toml .

# expose port
EXPOSE 8000

# run the binary
ENTRYPOINT ["/app/actix-web-rest-api-with-jwt"]
