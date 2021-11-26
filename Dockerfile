FROM rust:1.54.0 as builder
WORKDIR /todo_api
COPY Cargo.toml Cargo.toml

# ビルドするために何もしないソースコードを入れておく
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs

# 上記で作成したコードと依存クレートをビルド
RUN cargo build --release
COPY ./src ./src

# 先程ビルドした生成物のうち、アプリに関するものだけ削除
RUN rm -f target/release/deps/todo*

# 再度ビルド
RUN cargo build --release

FROM debian:10.4
COPY --from=builder /todo_api/target/release/todo_api /usr/local/bin/todo_api
CMD ["todo_api"]