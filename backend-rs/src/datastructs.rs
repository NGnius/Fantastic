use std::default::Default;
use std::convert::{Into, From};
use std::path::PathBuf;

use super::json::{SettingsJson, GraphPointJson, BoundsJson};

#[derive(Debug, Clone)]
pub struct Settings {
    pub version: u64,
    pub enable: bool,
    pub interpolate: bool,
    pub curve: Vec<GraphPoint>,
    pub fan_bounds: Bounds<f64>,
    pub temperature_bounds: Bounds<f64>,
}

impl Settings {
    pub fn sort_curve(&mut self) {
        self.curve.sort_by(|a, b| a.x.total_cmp(&b.x))
    }
}

impl From<SettingsJson> for Settings {
    fn from(mut other: SettingsJson) -> Self {
        let mut result = match other.version {
            0 => Self {
                version: 1,
                enable: other.enable,
                interpolate: other.interpolate,
                curve: other.curve.drain(..).map(|x| GraphPoint::from_json(x, other.version)).collect(),
                fan_bounds: Bounds {
                    min: 1.0,
                    max: 7000.0,
                },
                temperature_bounds: Bounds {
                    min: 0.0,
                    max: 100.0,
                },
            },
            1 => Self {
                version: 1,
                enable: other.enable,
                interpolate: other.interpolate,
                curve: other.curve.drain(..).map(|x| GraphPoint::from_json(x, other.version)).collect(),
                fan_bounds: other.fan_bounds.map(|x| Bounds::<f64>::from_json(x, other.version)).unwrap_or(Bounds {
                    min: 1.0,
                    max: 7000.0,
                }),
                temperature_bounds: other.temperature_bounds.map(|x| Bounds::<f64>::from_json(x, other.version)).unwrap_or(Bounds {
                    min: 0.0,
                    max: 100.0,
                }),
            },
            _ => Self {
                version: 1,
                enable: other.enable,
                interpolate: other.interpolate,
                curve: other.curve.drain(..).map(|x| GraphPoint::from_json(x, other.version)).collect(),
                fan_bounds: Bounds {
                    min: 1.0,
                    max: 7000.0,
                },
                temperature_bounds: Bounds {
                    min: 0.0,
                    max: 100.0,
                },
            }
        };
        result.sort_curve();
        result
    }
}

impl Into<SettingsJson> for Settings {
    #[inline]
    fn into(mut self) -> SettingsJson {
        SettingsJson {
            version: self.version,
            enable: self.enable,
            interpolate: self.interpolate,
            curve: self.curve.drain(..).map(|x| x.into()).collect(),
            fan_bounds: Some(self.fan_bounds.into()),
            temperature_bounds: Some(self.temperature_bounds.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphPoint {
    pub x: f64,
    pub y: f64,
}

impl GraphPoint {
    #[inline]
    pub fn from_json(other: GraphPointJson, version: u64) -> Self {
        match version {
            0 => Self {
                x: other.x,
                y: 1.0 - other.y, // use bottom left as origin, instead of whacky old way of top left
            },
            1 => Self {
                x: other.x,
                y: other.y,
            },
            _ => Self {
                x: other.x,
                y: other.y,
            }
        }
    }
}

impl Into<GraphPointJson> for GraphPoint {
    #[inline]
    fn into(self) -> GraphPointJson {
        GraphPointJson {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bounds<T: core::fmt::Debug + Clone> {
    pub min: T,
    pub max: T,
}

/*impl Bounds<usize> {
    #[inline]
    pub fn from_json(other: BoundsJson, version: u64) -> Self {
        match version {
            0 => Self {
                min: other.min as _,
                max: other.max as _,
            },
            1 => Self {
                min: other.min as _,
                max: other.max as _,
            },
            _ => Self {
                min: other.min as _,
                max: other.max as _,
            }
        }
    }
}*/

impl Bounds<f64> {
    #[inline]
    pub fn from_json(other: BoundsJson, version: u64) -> Self {
        match version {
            0 => Self {
                min: other.min,
                max: other.max,
            },
            1 => Self {
                min: other.min,
                max: other.max,
            },
            _ => Self {
                min: other.min,
                max: other.max,
            }
        }
    }
}

/*impl Into<BoundsJson> for Bounds<usize> {
    #[inline]
    fn into(self) -> BoundsJson {
        BoundsJson {
            min: self.min as _,
            max: self.max as _,
        }
    }
}*/

impl Into<BoundsJson> for Bounds<f64> {
    #[inline]
    fn into(self) -> BoundsJson {
        BoundsJson {
            min: self.min,
            max: self.max,
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub home: PathBuf,
    pub dirty: bool,
}

impl State {
    pub fn new() -> Self {
        let def = Self::default();
        Self {
            home: usdpl_back::api::dirs::home().unwrap_or(def.home),
            dirty: true,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            home: "/home/deck".into(),
            dirty: true,
        }
    }
}
