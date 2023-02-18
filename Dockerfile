# First, specify the base image that we'll use for the container
FROM rust:latest AS build

# Set up a working directory in the container
WORKDIR /app

# Copy the contents of your Rust project into the container
COPY . .

# Build the Rust application in release mode
RUN cargo build --release

# Now that we've built the application, we'll use a smaller, more lightweight
# base image for the final container that doesn't require Rust or build tools.
FROM debian:buster-slim

# Copy the compiled Rust application from the build container into the final container
COPY --from=build /app/target/release/lightning-db /usr/local/bin/lightning-db

# Expose the port that the API will listen on
EXPOSE 8080

# Set the default command to run when the container starts up
CMD ["lightning-db"]
