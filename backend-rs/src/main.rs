mod api;
mod control;
mod datastructs;
mod json;
mod sys;

use simplelog::{WriteLogger, LevelFilter};

use usdpl_back::Instance;

const PORT: u16 = 44444;

fn main() -> Result<(), ()> {
    WriteLogger::init(
        #[cfg(debug_assertions)]{LevelFilter::Debug},
        #[cfg(not(debug_assertions))]{LevelFilter::Info},
        Default::default(),
        std::fs::File::create("/tmp/fantastic.log").unwrap()
    ).unwrap();

    log::info!("Starting back-end ({} v{})", api::NAME, api::VERSION);
    println!("Starting back-end ({} v{})", api::NAME, api::VERSION);
    let runtime = control::ControlRuntime::new();
    runtime.run();
    Instance::new(PORT)
        .register("echo", api::echo)
        .register("hello", api::hello)
        .register("version", api::version)
        .register("name", api::name)
        .register("get_fan_rpm", api::get_fan_rpm)
        .register("get_temperature", api::get_temperature)
        .register("set_enable", api::set_enable_gen(&runtime))
        .register("get_enable", api::get_enable_gen(&runtime))
        .register("set_interpolate", api::set_interpolate_gen(&runtime))
        .register("get_interpolate", api::get_interpolate_gen(&runtime))
        .register("get_curve", api::get_curve_gen(&runtime))
        .register("add_curve_point", api::add_curve_point_gen(&runtime))
        .register("remove_curve_point", api::remove_curve_point_gen(&runtime))
        .run_blocking()
    //Ok(())
    //println!("Hello, world!");
}
