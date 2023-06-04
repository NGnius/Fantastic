fn main() {
    //println!("CWD: {}", std::env::current_dir().unwrap().display());
    usdpl_build::back::build([
        format!("{}/protos/fantastic.proto", std::env::current_dir().unwrap().display())
    ].into_iter(),
    [
        format!("{}/protos/", std::env::current_dir().unwrap().display())
    ].into_iter())
}
