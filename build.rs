use std::{env, path::PathBuf};

fn main() {
    let proto_file_bookstore = "./proto/bookstore.proto";
    let proto_file_user = "./proto/user.proto";

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("greeter_descriptor.bin"))
        .out_dir("./src")
        .compile(
            &[proto_file_bookstore, proto_file_user],
            &["."]
        )
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file_bookstore);
    println!("cargo:rerun-if-changed={}", proto_file_user);
}