ARG RUST_VERSION=1.86.0
ARG APP_NAME=rust-ml-inference-api

FROM rust:${RUST_VERSION}-bookworm AS build
ARG APP_NAME
WORKDIR /app

COPY ./onnx_models/mnist-8.onnx /bin/mnist-8.onnx

RUN echo "deb http://deb.debian.org/debian unstable main" \
      > /etc/apt/sources.list.d/unstable.list \
 && printf "Package: libonnxruntime-dev\nPin: release a=unstable\nPin-Priority: 200\n" \
      > /etc/apt/preferences.d/onnxruntime.pref \
 && apt-get update

RUN apt-get install -y --no-install-recommends \
      clang \
      lld \
      pkg-config \
      git \
      libonnxruntime-dev/unstable \
 && rm -rf /var/lib/apt/lists/*

RUN rm /etc/apt/sources.list.d/unstable.list


RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server


FROM debian:bookworm-slim AS final

COPY --from=build /usr/lib/x86_64-linux-gnu/ /usr/lib/x86_64-linux-gnu/

RUN ldconfig

WORKDIR /app

COPY --from=build /bin/mnist-8.onnx ./mnist-8.onnx
COPY --from=build /bin/server    ./server

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

EXPOSE 3000

CMD ["./server"]
