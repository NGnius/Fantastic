fn main() {
    println!("CWD: {}", std::env::current_dir().unwrap().display());
    usdpl_build::front::build(
        [format!(
            "{}/../../backend-rs/protos/fantastic.proto",
            std::env::current_dir().unwrap().display()
        )]
        .into_iter(),
        [format!(
            "{}/../../backend-rs/protos/",
            std::env::current_dir().unwrap().display()
        )]
        .into_iter(),
    )
}
