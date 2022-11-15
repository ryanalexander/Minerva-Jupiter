# Bundle Stage
FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates
RUN cargo install --locked --path .
COPY --from=builder /usr/local/cargo/bin/minerva-jupiter ./

# Final Stage
CMD ["./minerva-jupiter"]