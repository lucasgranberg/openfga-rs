use prost_wkt_build::*;
use std::{env, io::Result, path::PathBuf};

const PROTO_DEFAULT_DIR: &str = "proto";

fn main() -> Result<()> {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let proto_dir: PathBuf =
        PathBuf::from(env::var("PROTO_DIR").unwrap_or(PROTO_DEFAULT_DIR.to_string()));
    let proto_files = &[
        "openfga/v1/authzmodel.proto",
        "openfga/v1/errors_ignore.proto",
        "openfga/v1/openapi.proto",
        "openfga/v1/openfga.proto",
        "openfga/v1/openfga_service.proto",
    ]
    .map(|p| {
        let mut ret = proto_dir.clone();
        ret.push(&p);
        ret
    });
    let includes = &[proto_dir
        .to_str()
        .expect("expected proto dir to convert to str")];
    let descriptor_file = out.join("descriptors.bin");

    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Struct", "::prost_wkt_types::Struct")
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .extern_path(".google.protobuf.ListValue", "::prost_wkt_types::ListValue")
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".openfga.v1.UserTypeFilter", "UserTypeFilter")
        .file_descriptor_set_path(&descriptor_file);

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_with_config(prost_build, proto_files, includes)
        .unwrap_or_else(|e| {
            let current_dir = env::current_dir()
                .expect("expected to get current dir")
                .into_os_string()
                .into_string()
                .expect("expected to convert os string of current dir to string");
            let current_dir_contents = std::fs::read_dir(&current_dir)
                .expect("expected to read current dir")
                .map(|entry| entry.expect("expected dir to entry to unwrap ok").file_name().into_string().expect("expected to convert os string of current dir to string"))
                .collect::<Vec<String>>()
                .join(",\n");
            let out_dir = out.display().to_string();
            let out_dir_contents = std::fs::read_dir(&out_dir)
                .expect("expected to read out dir")
                .map(|entry| entry.expect("expected dir entry to unwrap ok").file_name().into_string().expect("expected to convert os string of current dir to string"))
                .collect::<Vec<String>>()
                .join(",\n");
            panic!("failed to compile protos, error: {e}, current dir: {current_dir}, current dir contents: {current_dir_contents}, out_dir: {out_dir}, out_dir contents: {out_dir_contents}")
        });

    let descriptor_bytes = std::fs::read(descriptor_file).expect("failed to read descriptor file");
    let descriptor =
        FileDescriptorSet::decode(&descriptor_bytes[..]).expect("failed to decode descriptor file");
    prost_wkt_build::add_serde(out, descriptor);

    Ok(())
}
