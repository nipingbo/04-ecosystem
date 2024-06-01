# Geektime Rust 语言训练营
第四周 rust 生态


run docker command:
docker run -d -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest

docker container prune


//install tokio-console,use below command
cargo install --locked tokio-console

//compile 
RUSTFLAGS="--cfg tokio_unstable" cargo build

run
RUSTFLAGS="--cfg tokio_unstable" cargo run --example chat
