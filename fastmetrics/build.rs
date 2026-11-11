fn main() {
    #[cfg(feature = "protobuf")]
    generate_protobuf();
}

#[cfg(feature = "protobuf")]
fn generate_protobuf() {
    // generate `<OUT_DIR>/*.rs`
    prost_build::Config::new()
        .compile_protos(&["src/format/protobuf/prometheus_data_model.proto"], &[""])
        .unwrap();
}
