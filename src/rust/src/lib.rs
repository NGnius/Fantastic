#[allow(missing_docs)]
#[allow(dead_code)]
pub mod services {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

pub use usdpl_front;
