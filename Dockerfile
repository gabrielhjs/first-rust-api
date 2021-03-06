FROM rust:latest as builder

# Create an empty rust project
RUN USER=root cargo new --bin web_server
WORKDIR ./web_server
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml

# Build empty rust project with libs
RUN cargo build --release
RUN rm src/*.rs

# Copy project files
COPY ./src ./src

RUN rm ./target/release/deps/web_server*

# Build the final release
RUN cargo build --release

FROM postgres:latest
ARG APP=/usr/src/app

ENV TZ=Etc/UTC \
    APP_USER=appuser

# Create an user for access the app
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# Copy final release from builder
COPY --from=builder /web_server/target/release/web_server ${APP}/web_server
COPY ./migrations ./web_server/migrations

# Apply the user to app directory
RUN chown -R $APP_USER:$APP_USER ${APP}

# Using app user
USER $APP_USER
WORKDIR ${APP}

# Run binary application
CMD ["./web_server"]
