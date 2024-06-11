# syntax=docker/dockerfile:1

##
## Build the application from source
##

FROM rust AS build-stage
WORKDIR /app
COPY . .
RUN cargo build --release

##
## Deploy the application binary into a lean image
##

FROM gcr.io/distroless/cc
ENV DATABASE_URL=sqlite://vol/zidian.db
ENV RUST_LOG=ruzdman=inf
WORKDIR /
COPY --from=build-stage /app/target/release/ruzdman /ruzdman
COPY ./vol ./vol
EXPOSE 8090
ENTRYPOINT ["/ruzdman"]
