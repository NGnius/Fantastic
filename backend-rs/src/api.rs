use crate::services::fantastic::*;

use super::control::ControlRuntime;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct FanService {
    ctrl: ControlRuntime,
}

impl FanService {
    pub fn new(runtime: ControlRuntime) -> Self {
        runtime.run();
        Self {
            ctrl: runtime,
        }
    }
}

#[usdpl_back::nrpc::_helpers::async_trait::async_trait]
impl IFan for FanService {
    async fn echo(
            &mut self,
            input: EchoMessage,
        ) -> Result<EchoMessage, Box<dyn std::error::Error>> {
            Ok(input)
        }
        async fn hello(
            &mut self,
            input: NameMessage,
        ) -> Result<HelloResponse, Box<dyn std::error::Error>> {
            Ok(HelloResponse {
                phrase: format!("Hello {}", input.name)
            })
        }
        async fn version(
            &mut self,
            _input: Empty,
        ) -> Result<VersionMessage, Box<dyn std::error::Error>> {
            Ok(
                VersionMessage {
                    major: 0,
                    minor: 0,
                    patch: 0,
                    //display: VERSION.to_string(),
                }
            )
        }
        async fn version_str(
            &mut self,
            _input: Empty,
        ) -> Result<VersionDisplayMessage, Box<dyn std::error::Error>> {
            Ok(
                VersionDisplayMessage {
                    display: VERSION.to_owned(),
                }
            )
        }
        async fn name(
            &mut self,
            _input: Empty,
        ) -> Result<NameMessage, Box<dyn std::error::Error>> {
            Ok(
                NameMessage {
                    name: NAME.to_string(),
                }
            )
        }
        async fn get_fan_rpm(
            &mut self,
            _input: Empty,
        ) -> Result<RpmMessage, Box<dyn std::error::Error>> {
            if let Some(rpm) = crate::sys::read_fan() {
                log::debug!("get_fan_rpm() success: {}", rpm);
                Ok(RpmMessage { rpm: rpm as u32 })
            } else {
                Err("Failed to read fan speed".into())
            }
        }
        async fn get_temperature(
            &mut self,
            _input: Empty,
        ) -> Result<TemperatureMessage, Box<dyn std::error::Error>>{
            if let Some(temperature) = crate::sys::read_thermal_zone(0) {
                let real_temp = temperature as f64 / 1000.0;
                log::debug!("get_temperature() success: {}", real_temp);
                Ok(TemperatureMessage { temperature: real_temp })
            } else {
                Err("get_temperature failed to read thermal zone 0".into())
            }
        }
        async fn set_enable(
            &mut self,
            input: EnablementMessage,
        ) -> Result<EnablementMessage, Box<dyn std::error::Error>>{
            let mut settings = self.ctrl.settings().write().await;
            if settings.enable != input.is_enabled {
                let mut state = self.ctrl.state().write().await;
                settings.enable = input.is_enabled;
                state.dirty = true;
            }
            log::debug!("set_enable({}) success", input.is_enabled);
            Ok(input)
        }
        async fn get_enable(
            &mut self,
            _input: Empty,
        ) -> Result<EnablementMessage, Box<dyn std::error::Error>>{
            let is_enabled = self.ctrl.settings().read().await.enable;
            log::debug!("get_enable() success");
            Ok(EnablementMessage { is_enabled })
        }
        async fn set_interpolate(
            &mut self,
            input: EnablementMessage,
        ) -> Result<EnablementMessage, Box<dyn std::error::Error>>{
            let mut settings = self.ctrl.settings().write().await;
            if settings.interpolate != input.is_enabled {
                let mut state = self.ctrl.state().write().await;
                settings.interpolate = input.is_enabled;
                state.dirty = true;
            }
            log::debug!("set_interpolate({}) success", input.is_enabled);
            Ok(input)
        }
        async fn get_interpolate(
            &mut self,
            _input: Empty,
        ) -> Result<EnablementMessage, Box<dyn std::error::Error>>{
            let is_enabled = self.ctrl.settings().read().await.interpolate;
            log::debug!("get_interpolate() success");
            Ok(EnablementMessage { is_enabled })
        }
        async fn get_curve_x(
            &mut self,
            _input: Empty,
        ) -> Result<CurveMessageX, Box<dyn std::error::Error>>{
            let settings = self.ctrl.settings().read().await;
            let x = settings.curve.iter().map(|p| p.x).collect();
            log::debug!("get_curve_x() success");
            Ok(CurveMessageX { x })
        }
        async fn get_curve_y(
            &mut self,
            _input: Empty,
        ) -> Result<CurveMessageY, Box<dyn std::error::Error>>{
            let settings = self.ctrl.settings().read().await;
            let y = settings.curve.iter().map(|p| p.y).collect();
            log::debug!("get_curve_x() success");
            Ok(CurveMessageY { y })
        }
        async fn add_curve_point(
            &mut self,
            point: GraphPoint,
        ) -> Result<Empty, Box<dyn std::error::Error>>{
            let mut settings = self.ctrl.settings().write().await;
            settings.curve.push(super::datastructs::GraphPoint {
                x: point.x,
                y: point.y
            });
            settings.sort_curve();
            let mut state = self.ctrl.state().write().await;
            state.dirty = true;
            Ok(Empty { ok: true })
        }
        async fn remove_curve_point(
            &mut self,
            input: IndexMessage,
        ) -> Result<Empty, Box<dyn std::error::Error>>{
            let mut settings = self.ctrl.settings().write().await;
            let i = input.index as usize;
            if settings.curve.len() < i {
                settings.curve.swap_remove(i);
                settings.sort_curve();
                let mut state = self.ctrl.state().write().await;
                state.dirty = true;
                Ok(Empty { ok: true })
            } else {
                Ok(Empty { ok: false })
            }
        }
}
