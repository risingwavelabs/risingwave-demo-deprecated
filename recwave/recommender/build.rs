use std::{env, path::PathBuf};
use std::fs;

fn main() {
    let actor_proto = "./actor.proto";
    let model_proto = "./model/model.proto";

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src")
        .compile(&[actor_proto], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
    fs::copy("./src/recommender.rs", "../simulator/src/recommender.rs").unwrap();

    tonic_build::configure()
        .build_client(true)
        .out_dir("./src")
        .compile(&[model_proto], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
    println!("cargo:rerun-if-changed={}", actor_proto);
}
