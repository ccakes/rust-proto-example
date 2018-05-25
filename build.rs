extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        input: &["proto.proto"],
        includes: &["."],
        out_dir: "src/",
        ..Default::default()
    }).unwrap();
}