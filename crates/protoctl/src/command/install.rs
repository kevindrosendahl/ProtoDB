use crate::transport::grpc;

pub fn install() {
    let mut client = grpc::Client::new();
    println!("{:?}", client.list_databases().unwrap().databases);
}
