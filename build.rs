
fn main() -> Result<(), Box<dyn std::error::Error>> {
     tonic_build::compile_protos("protobuf/hello.proto")?;
     // protoc_rust_grpc::Codegen::new()
     //    .out_dir("src")
     //    .input("./protobuf/hello.proto")
     //    .rust_protobuf(true)
     //    .run()
     //    .expect("error compliling protocol buffer");
     Ok(())
}

