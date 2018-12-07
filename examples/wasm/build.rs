extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["../protos/user.proto", "../protos/user_statistics.proto"],
        &["../protos"],
    )
    .unwrap();
}
