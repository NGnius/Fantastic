use usdpl_back::api::files::*;

const HWMON_INDEX: usize = 5;

pub fn read_fan() -> Option<u64> {
    read_single(format!("/sys/class/hwmon/hwmon{}/fan1_input", HWMON_INDEX)).ok()
}

pub fn read_thermal_zone(index: u8) -> Option<u64> {
    read_single(format!("/sys/class/thermal/thermal_zone{}/temp", index)).ok()
}

pub fn write_fan_recalc(enabled: bool) -> Result<(), std::io::Error> {
    write_single(format!("/sys/class/hwmon/hwmon{}/recalculate", HWMON_INDEX), enabled as u8)
}

pub fn write_fan_target(rpm: u64) -> Result<(), std::io::Error> {
    write_single(format!("/sys/class/hwmon/hwmon{}/fan1_target", HWMON_INDEX), rpm)
}
