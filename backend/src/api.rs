use usdpl_back::core::serdes::Primitive;

use super::control::ControlRuntime;
use super::json::GraphPointJson;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn hello(params: Vec<Primitive>) -> Vec<Primitive> {
    if let Some(Primitive::String(name)) = params.get(0) {
        vec![Primitive::String(format!("Hello {}", name))]
    } else {
        vec![]
    }
}

pub fn echo(params: Vec<Primitive>) -> Vec<Primitive> {
    params
}

pub fn version(_: Vec<Primitive>) -> Vec<Primitive> {
    vec![VERSION.into()]
}

pub fn name(_: Vec<Primitive>) -> Vec<Primitive> {
    vec![NAME.into()]
}

pub fn get_fan_rpm(_: Vec<Primitive>) -> Vec<Primitive> {
    if let Some(rpm) = crate::sys::read_fan() {
        log::debug!("get_fan_rpm() success: {}", rpm);
        vec![rpm.into()]
    } else {
        log::error!("get_fan_rpm failed to read fan speed");
        Vec::new()
    }
}

pub fn get_temperature(_: Vec<Primitive>) -> Vec<Primitive> {
    if let Some(temperature) = crate::sys::read_thermal_zone(0) {
        let real_temp = temperature as f64 / 1000.0;
        log::debug!("get_temperature() success: {}", real_temp);
        vec![real_temp.into()]
    } else {
        log::error!("get_fan_rpm failed to read fan speed");
        Vec::new()
    }
}

pub fn set_enable_gen(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_settings = runtime.settings_clone();
    let runtime_state = runtime.state_clone();
    move |params| {
        if let Some(Primitive::Bool(enabled)) = params.get(0) {
            let mut settings = match runtime_settings.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("set_enable failed to acquire settings write lock: {}", e);
                    return vec![];
                }
            };
            if settings.enable != *enabled {
                settings.enable = *enabled;
                let mut state = match runtime_state.write() {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("set_enable failed to acquire state write lock: {}", e);
                        return vec![];
                    }
                };
                state.dirty = true;
                log::debug!("set_enable({}) success", enabled);
            }
            vec![(*enabled).into()]
        } else {
            Vec::new()
        }
    }
}

pub fn get_enable_gen(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_settings = runtime.settings_clone();
    move |_| {
        let lock = match runtime_settings.read() {
            Ok(x) => x,
            Err(e) => {
                log::error!("get_enable failed to acquire settings read lock: {}", e);
                return vec![];
            }
        };
        log::debug!("get_enable() success");
        vec![lock.enable.into()]
    }
}

pub fn set_interpolate_gen(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_settings = runtime.settings_clone();
    let runtime_state = runtime.state_clone();
    move |params| {
        if let Some(Primitive::Bool(enabled)) = params.get(0) {
            let mut settings = match runtime_settings.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("set_enable failed to acquire settings write lock: {}", e);
                    return vec![];
                }
            };
            if settings.interpolate != *enabled {
                settings.interpolate = *enabled;
                let mut state = match runtime_state.write() {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("set_interpolate failed to acquire state write lock: {}", e);
                        return vec![];
                    }
                };
                state.dirty = true;
                log::debug!("set_interpolate({}) success", enabled);
            }
            vec![(*enabled).into()]
        } else {
            Vec::new()
        }
    }
}

pub fn get_interpolate_gen(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_settings = runtime.settings_clone();
    move |_| {
        let lock = match runtime_settings.read() {
            Ok(x) => x,
            Err(e) => {
                log::error!("get_interpolate failed to acquire settings read lock: {}", e);
                return vec![];
            }
        };
        log::debug!("get_interpolate() success");
        vec![lock.interpolate.into()]
    }
}

fn curve_to_json(curve: &Vec<super::datastructs::GraphPoint>) -> serde_json::Result<String> {
    let mut curve_points = Vec::<GraphPointJson>::with_capacity(curve.len());
    for point in curve.iter() {
        curve_points.push(point.clone().into());
    }
    serde_json::to_string(&curve_points)
}

pub fn get_curve_gen(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_settings = runtime.settings_clone();
    move |_| {
        let lock = match runtime_settings.read() {
            Ok(x) => x,
            Err(e) => {
                log::error!("get_curve failed to acquire settings read lock: {}", e);
                return vec![];
            }
        };
        let json_str = match curve_to_json(&lock.curve) {
            Ok(x) => x,
            Err(e) => {
                log::error!("get_curve failed to serialize points: {}", e);
                return vec![];
            }
        };
        log::debug!("get_curve() success");
        vec![Primitive::Json(json_str)]
    }
}

pub fn add_curve_point_gen(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_settings = runtime.settings_clone();
    let runtime_state = runtime.state_clone();
    move |params| {
        if let Some(Primitive::Json(json_str)) = params.get(0) {
            let mut settings = match runtime_settings.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("add_curve_point failed to acquire settings write lock: {}", e);
                    return vec![];
                }
            };
            let new_point: GraphPointJson = match serde_json::from_str(&json_str) {
                Ok(x) => x,
                Err(e) => {
                    log::error!("add_curve_point failed deserialize point json: {}", e);
                    return vec![];
                }
            };
            let version = settings.version;
            settings.curve.push(super::datastructs::GraphPoint::from_json(new_point, version));
            settings.sort_curve();
            let mut state = match runtime_state.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("add_curve_point failed to acquire state write lock: {}", e);
                    return vec![];
                }
            };
            state.dirty = true;
            let json_str = match curve_to_json(&settings.curve) {
                Ok(x) => x,
                Err(e) => {
                    log::error!("add_curve_point failed to serialize points: {}", e);
                    return vec![];
                }
            };
            log::debug!("add_curve_point({}) success", json_str);
            vec![Primitive::Json(json_str)]
        } else {
            Vec::new()
        }
    }
}

pub fn remove_curve_point_gen(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_settings = runtime.settings_clone();
    let runtime_state = runtime.state_clone();
    move |params| {
        if let Some(Primitive::F64(index)) = params.get(0) {
            let mut settings = match runtime_settings.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("remove_curve_point failed to acquire settings write lock: {}", e);
                    return vec![];
                }
            };
            let rounded = index.round();
            if rounded >= 0.0 && rounded < settings.curve.len() as _ {
                let index = rounded as usize;
                settings.curve.swap_remove(index);
                settings.sort_curve();
                let mut state = match runtime_state.write() {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("remove_curve_point failed to acquire state write lock: {}", e);
                        return vec![];
                    }
                };
                state.dirty = true;
                let json_str = match curve_to_json(&settings.curve) {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("remove_curve_point failed to serialize points: {}", e);
                        return vec![];
                    }
                };
                log::debug!("remove_curve_point({}) success", json_str);
                vec![Primitive::Json(json_str)]
            } else {
                log::error!("remove_curve_point received index out of bounds: {} indexing array of length {}", index, settings.curve.len());
                return vec![];
            }
        } else {
            Vec::new()
        }
    }
}
