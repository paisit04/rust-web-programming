FROM rust:1.62.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=build /app/target/release/task_queue /usr/local/bin/task_queue
EXPOSE 3000
ENTRYPOINT ["task_queue"]