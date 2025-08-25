fn main() {
    #[cfg(feature = "prost")]
    generate_prost();

    #[cfg(feature = "protobuf")]
    generate_protobuf();
}

#[cfg(feature = "prost")]
fn generate_prost() {
    let prost_out_dir = {
        let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR env var not set");
        let prost_out_dir = std::path::Path::new(&out_dir).join("prost");
        std::fs::create_dir_all(&prost_out_dir).expect("Failed to create prost subdirectory");
        prost_out_dir
    };

    // generate `out/prost/openmetrics.rs`
    prost_build::Config::new()
        .out_dir(prost_out_dir)
        .compile_protos(&["src/format/proto/openmetrics_data_model.proto"], &[""])
        .unwrap();
}

#[cfg(feature = "protobuf")]
fn generate_protobuf() {
    // generate `out/protobuf/openmetrics_data_model.rs`
    protobuf_codegen::Codegen::new()
        .includes([""])
        .inputs(["src/format/proto/openmetrics_data_model.proto"])
        .customize(protobuf_codegen::Customize::default())
        .pure()
        .cargo_out_dir("protobuf")
        .run_from_script();
}
