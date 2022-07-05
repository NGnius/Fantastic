use usdpl_back::api::files::*;

pub fn read_fan() -> Option<u64> {
    read_single("/sys/class/hwmon/hwmon5/fan1_input").ok()
}

pub fn read_thermal_zone(index: u8) -> Option<u64> {
    read_single(format!("/sys/class/thermal/thermal_zone{}/temp", index)).ok()
}

pub fn write_fan_recalc(enabled: bool) -> Result<(), std::io::Error> {
    write_single("/sys/class/hwmon/hwmon5/recalculate", enabled as u8)
}

pub fn write_fan_target(rpm: u64) -> Result<(), std::io::Error> {
    write_single("/sys/class/hwmon/hwmon5/fan1_target", rpm)
}
