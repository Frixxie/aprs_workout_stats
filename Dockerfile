FROM rust:latest as builder
WORKDIR /usr/src/aprs_workout_stats
COPY . .
ENV APIKEY=REPLACEME
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/aprs_workout_stats /usr/local/bin/aprs_workout_stats
CMD ["aprs_workout_stats"]
