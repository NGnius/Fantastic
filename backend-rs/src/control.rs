//! Fan control

use std::sync::{RwLock, Arc};

use std::thread;
use std::time::{Duration, Instant};

use super::datastructs::{Settings, State, GraphPoint};
use super::json::SettingsJson;

const VALVE_FAN_SERVICE: &str = "jupiter-fan-control.service";

pub struct ControlRuntime {
    settings: Arc<RwLock<Settings>>,
    state: Arc<RwLock<State>>,
}

impl ControlRuntime {
    pub fn new() -> Self {
        let new_state = State::new();
        let settings_p = settings_path(&new_state.home);
        Self {
            settings: Arc::new(RwLock::new(super::json::SettingsJson::open(settings_p).unwrap_or_default().into())),
            state: Arc::new(RwLock::new(new_state)),
        }
    }

    pub(crate) fn settings_clone(&self) -> Arc<RwLock<Settings>> {
        self.settings.clone()
    }

    pub(crate) fn state_clone(&self) -> Arc<RwLock<State>> {
        self.state.clone()
    }

    pub fn run(&self) -> thread::JoinHandle<()> {
        let runtime_settings = self.settings_clone();
        let runtime_state = self.state_clone();
        thread::spawn(move || {
            let sleep_duration = Duration::from_millis(1000);
            let mut start_time = Instant::now();
            loop {
                if Instant::now().duration_since(start_time).as_secs_f64() * 0.95 > sleep_duration.as_secs_f64() {
                    // resumed from sleep; do fan re-init
                    log::debug!("Detected resume from sleep, overriding fan again");
                    {
                        let state = match runtime_state.read() {
                            Ok(x) => x,
                            Err(e) => {
                                log::error!("runtime failed to acquire state read lock: {}", e);
                                continue;
                            }
                        };
                        let settings = match runtime_settings.read() {
                            Ok(x) => x,
                            Err(e) => {
                                log::error!("runtime failed to acquire settings read lock: {}", e);
                                continue;
                            }
                        };
                        if settings.enable {
                            Self::on_set_enable(&settings, &state);
                        }
                    }
                }
                start_time = Instant::now();
                { // save to file
                    let state = match runtime_state.read() {
                        Ok(x) => x,
                        Err(e) => {
                            log::error!("runtime failed to acquire state read lock: {}", e);
                            continue;
                        }
                    };
                    if state.dirty {
                        // save settings to file
                        let settings = match runtime_settings.read() {
                            Ok(x) => x,
                            Err(e) => {
                                log::error!("runtime failed to acquire settings read lock: {}", e);
                                continue;
                            }
                        };
                        let settings_json: SettingsJson = settings.clone().into();
                        if let Err(e) = settings_json.save(settings_path(&state.home)) {
                            log::error!("SettingsJson.save({}) error: {}", settings_path(&state.home).display(), e);
                        }
                        Self::on_set_enable(&settings, &state);
                        drop(state);
                        let mut state = match runtime_state.write() {
                            Ok(x) => x,
                            Err(e) => {
                                log::error!("runtime failed to acquire state write lock: {}", e);
                                continue;
                            }
                        };
                        state.dirty = false;
                    }
                }
                { // fan control
                    let settings = match runtime_settings.read() {
                        Ok(x) => x,
                        Err(e) => {
                            log::error!("runtime failed to acquire settings read lock: {}", e);
                            continue;
                        }
                    };
                    if settings.enable {
                        Self::enforce_jupiter_status(true);
                        Self::do_fan_control(&settings);
                    }
                }
                thread::sleep(sleep_duration);
            }
        })
    }

    fn on_set_enable(settings: &Settings, _state: &State) {
        // stop/start jupiter fan control (since the client-side way of doing this was removed :( )
        Self::enforce_jupiter_status(settings.enable);
        if let Err(e) = crate::sys::write_fan_recalc(settings.enable) {
            log::error!("runtime failed to write to fan recalculate file: {}", e);
        }
    }

    fn do_fan_control(settings: &Settings) {
        /*
            curve = self.settings["curve"]
            fan_ratio = 0 # unnecessary in Python, but stupid without
            if len(curve) == 0:
                fan_ratio = 1
            else:
                index = -1
                temperature_ratio = (thermal_zone(0) - TEMPERATURE_MINIMUM) / (TEMPERATURE_MAXIMUM - TEMPERATURE_MINIMUM)
                for i in range(len(curve)-1, -1, -1):
                    if curve[i]["x"] < temperature_ratio:
                        index = i
                        break
                if self.settings["interpolate"]:
                    fan_ratio = self.interpolate_fan(self, index, temperature_ratio)
                else:
                    fan_ratio = self.step_fan(self, index, temperature_ratio)
            set_fan_target(int((fan_ratio * FAN_MAXIMUM) + FAN_MINIMUM))
        */
        let fan_ratio: f64 = if let Some(thermal_zone) = crate::sys::read_thermal_zone(0) {
            let temperature_ratio = (((thermal_zone as f64)/1000.0) - settings.temperature_bounds.min)
                / (settings.temperature_bounds.max - settings.temperature_bounds.min);
            let mut index = None;
            for i in (0..settings.curve.len()).rev() {
                if settings.curve[i].x < temperature_ratio {
                    index = Some(i);
                    break;
                }
            }
            if settings.interpolate {
                Self::interpolate_fan(settings, index, temperature_ratio)
            } else {
                Self::step_fan(settings, index, temperature_ratio)
            }
        } else {
            1.0
        };
        let fan_speed: u64 = ((fan_ratio * (settings.fan_bounds.max - settings.fan_bounds.min)) + settings.fan_bounds.min) as _;
        if let Err(e) = crate::sys::write_fan_target(fan_speed) {
            log::error!("runtime failed to write to fan target file: {}", e);
        }
    }

    fn interpolate_fan(settings: &Settings, index: Option<usize>, t_ratio: f64) -> f64 {
        /*
            curve = self.settings["curve"]
            upper_point = {"x": 1.0, "y": 0.0}
            lower_point = {"x": 0.0, "y": 1.0}
            if index != -1: # guaranteed to not be empty
                lower_point = curve[index]
            if index != len(curve) - 1:
                upper_point = curve[index+1]
            #logging.debug(f"lower_point: {lower_point}, upper_point: {upper_point}")
            upper_y = 1-upper_point["y"]
            lower_y = 1-lower_point["y"]
            slope_m = (upper_y - lower_y) / (upper_point["x"] - lower_point["x"])
            y_intercept_b = lower_y - (slope_m * lower_point["x"])
            logging.debug(f"interpolation: y = {slope_m}x + {y_intercept_b}")
            return (slope_m * temperature_ratio) + y_intercept_b
        */
        let (upper, lower) = if let Some(i) = index {
            (if i != settings.curve.len() - 1 {
                settings.curve[i+1].clone()
            } else {
                GraphPoint{x: 1.0, y: 1.0}
            },
            settings.curve[i].clone())
        } else {
            (if settings.curve.is_empty() {
                GraphPoint{x: 1.0, y: 1.0}
            } else {
                settings.curve[0].clone()
            },
            GraphPoint{x: 0.0, y: 0.0})
        };
        let slope_m = (upper.y - lower.y) / (upper.x - lower.x);
        let y_intercept_b = lower.y - (slope_m * lower.x);
        log::debug!("interpolation: y = {}x + {} (between {:?} and {:?})", slope_m, y_intercept_b, upper, lower);
        (slope_m * t_ratio) + y_intercept_b
    }

    fn step_fan(settings: &Settings, index: Option<usize>, _t_ratio: f64) -> f64 {
        /*
            curve = self.settings["curve"]
            if index != -1:
                return 1 - curve[index]["y"]
            else:
                if len(curve) == 0:
                    return 1
                else:
                    return 0.5
        */
        // step fan, what are you doing?
        if let Some(index) = index {
            settings.curve[index].y
        } else {
            if settings.curve.is_empty() {
                1.0
            } else {
                0.5
            }
        }
    }

    fn enforce_jupiter_status(enabled: bool) {
        // enabled refers to whether this plugin's functionality is enabled,
        // not the jupiter fan control service
        let service_status = Self::detect_jupiter_fan_service();
        log::debug!("fan control service is enabled? {}", service_status);
        if enabled == service_status {
            // do not run Valve's fan service along with Fantastic, since they fight
            if enabled {
                Self::stop_fan_service();
            } else {
                Self::start_fan_service();
            }
        }
    }

    fn detect_jupiter_fan_service() -> bool {
        match std::process::Command::new("systemctl")
            .args(["is-active", VALVE_FAN_SERVICE])
            .output() {
            Ok(cmd) => String::from_utf8_lossy(&cmd.stdout).trim() == "active",
            Err(e) => {
                log::error!("`systemctl is-active {}` err: {}", VALVE_FAN_SERVICE, e);
                false
            }
        }
    }

    fn start_fan_service() {
        match std::process::Command::new("systemctl")
            .args(["start", VALVE_FAN_SERVICE])
            .output() {
            Err(e) => log::error!("`systemctl start {}` err: {}", VALVE_FAN_SERVICE, e),
            Ok(out) => log::debug!("started `{}`:\nstdout:{}\nstderr:{}", VALVE_FAN_SERVICE, String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)),
        }
    }

    fn stop_fan_service() {
        match std::process::Command::new("systemctl")
            .args(["stop", VALVE_FAN_SERVICE])
            .output() {
            Err(e) => log::error!("`systemctl stop {}` err: {}", VALVE_FAN_SERVICE, e),
            Ok(out) => log::debug!("stopped `{}`:\nstdout:{}\nstderr:{}", VALVE_FAN_SERVICE, String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)),
        }
    }
}

fn settings_path<P: AsRef<std::path::Path>>(home: P) -> std::path::PathBuf {
    home.as_ref().join(".config/fantastic/fantastic.json")
}
