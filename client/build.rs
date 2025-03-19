fn main() -> std::io::Result<()> {
    #[cfg(feature = "protobuf")]
    prost_build::Config::new()
        .compile_protos(&["src/format/protobuf/openmetrics_data_model.proto"], &[""])?;
    Ok(())
}
