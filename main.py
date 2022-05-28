import json
import os
import asyncio
import time
import pathlib
import subprocess

HOME_DIR = str(pathlib.Path(os.getcwd()).parent.parent.resolve())

import logging

logging.basicConfig(
    filename = "/tmp/fantastic.log",
    format = '%(asctime)s %(levelname)s %(message)s',
    filemode = 'w',
    force = True)

logger = logging.getLogger()
logger.setLevel(logging.DEBUG)

FAN_MINIMUM = 0
FAN_MAXIMUM = 7000 # max is more around 7100

TEMPERATURE_MINIMUM = 0.0
TEMPERATURE_MAXIMUM = 100.0

DATA_SAVE_FILE = "fantastic.json"
DATA_SAVE_FOLDER = HOME_DIR + "/.config/fantastic/"
DATA_SAVE_PATH = DATA_SAVE_FOLDER + DATA_SAVE_FILE

DEFAULT_DATA = {
    "version": 0,
    "enable": False,
    "interpolate": True,
    "curve": [], # items are {x: int (distance from left), y: int (distance from top, NOT bottom)}
}

class Plugin:
    settings = None
    is_changed = False

    plot_width = 1
    plot_height = 1

    period_s = 1.0

    jupiter_fan_control_was_disabled = False

    async def set_curve(self, curve):
        await self.wait_for_ready(self)
        self.settings["curve"] = curve
        self.is_changed = True

    async def get_curve(self):
        await self.wait_for_ready(self)
        return self.settings["curve"]

    async def get_curve_point(self, index):
        await self.wait_for_ready(self)
        return self.settings["curve"][index]

    async def set_curve_point(self, index, point):
        await self.wait_for_ready(self)
        self.settings["curve"][index] = point
        self.is_changed = True

    async def add_curve_point(self, point):
        await self.wait_for_ready(self)
        self.settings["curve"].append(point)
        self.settings["curve"].sort(key=lambda p: p["x"])
        self.is_changed = True
        x = point["x"]
        y = point["y"]
        logger.debug(f"Added point (Temp:{100*x},PWM%:{100*y}) ~= ({x*self.plot_width},{y*self.plot_height})")

    async def remove_curve_point(self, index):
        await self.wait_for_ready(self)
        del(self.settings["curve"][index])
        self.is_changed = True

    async def set_enable(self, enable: bool):
        await self.wait_for_ready(self)
        self.settings["enable"] = enable
        self.on_set_enable(self)
        self.is_changed = True

    async def get_enable(self) -> bool:
        await self.wait_for_ready(self)
        return self.settings["enable"]

    async def set_interpol(self, interpolate: bool):
        await self.wait_for_ready(self)
        self.settings["interpolate"] = interpolate
        self.is_changed = True

    async def get_interpol(self) -> bool:
        await self.wait_for_ready(self)
        return self.settings["interpolate"]

    async def set_plot_size(self, x, y):
        logging.debug(f"Set plot size to ({x},{y})")
        self.plot_width = x
        self.plot_height = y

    async def get_fan_rpm(self) -> int:
        return get_fan_input()

    async def get_temperature(self) -> int:
        return int(thermal_zone(0))

    async def set_poll_period(self, period):
        self.period_s = period

    def on_set_enable(self):
        if self.settings["enable"]:
            self.disable_jupiter_fan_control(self)
            on_enable()
        else:
            self.enable_jupiter_fan_control(self)
            on_disable()

    def disable_jupiter_fan_control(self):
        active = subprocess.Popen(["systemctl", "is-active", "jupiter-fan-control.service"]).wait() == 0
        if active:
            logging.info("Stopping jupiter-fan-control.service so it doesn't interfere")
            # only disable if currently active
            self.jupiter_fan_control_was_disabled = True
            stop_p = subprocess.Popen(["systemctl", "stop", "jupiter-fan-control.service"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            stop_p.wait()
            logging.debug("systemctl stop jupiter-fan-control.service stdout:\n" + stop_p.stdout.read().decode())
            logging.debug("systemctl stop jupiter-fan-control.service stderr:\n" + stop_p.stderr.read().decode())

    def enable_jupiter_fan_control(self):
        if self.jupiter_fan_control_was_disabled:
            logging.info("Starting jupiter-fan-control.service so it doesn't interfere")
            # only re-enable if I disabled it
            self.jupiter_fan_control_was_disabled = False
            start_p = subprocess.Popen(["systemctl", "start", "jupiter-fan-control.service"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            start_p.wait()
            logging.debug("systemctl start jupiter-fan-control.service stdout:\n" + start_p.stdout.read().decode())
            logging.debug("systemctl start jupiter-fan-control.service stderr:\n" + start_p.stderr.read().decode())

    def save(self):
        if not os.path.exists(DATA_SAVE_FOLDER):
            os.mkdir(DATA_SAVE_FOLDER)
        with open(DATA_SAVE_PATH, "w") as data_file:
            json.dump(self.settings, data_file)

    async def wait_for_ready(self):
        while self.settings is None:
            await asyncio.sleep(0.01)

    def do_fan_control(self):
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


    def interpolate_fan(self, index, temperature_ratio):
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

    def step_fan(self, index, temperature_ratio):
        curve = self.settings["curve"]
        if index != -1:
            return 1 - curve[index]["y"]
        else:
            if len(curve) == 0:
                return 1
            else:
                return 0.5

    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded
    async def _main(self):
        # startup
        if os.path.exists(DATA_SAVE_PATH):
            with open(DATA_SAVE_PATH, "r") as data_file:
                self.settings = json.load(data_file)
        else:
            self.settings = dict(DEFAULT_DATA)
        try:
            str(self.settings["version"])
        except:
            self.settings = dict(DEFAULT_DATA)
        while self.settings["version"] != DEFAULT_DATA["version"]:
            # TODO specific upgrade functionality
            self.settings["version"] = DEFAULT_DATA["version"]
            self.settings["enable"] = DEFAULT_DATA["enable"]
            self.settings["interpolate"] = DEFAULT_DATA["interpolate"]
            self.settings["curve"] = DEFAULT_DATA["curve"]
            self.is_changed = True
        self.on_set_enable(self)
        last_time = time.time()
        # work loop
        while True:
            if (time.time() - last_time) * 0.9 > self.period_s:
                # detect sleep
                logging.debug("Detected resume from sleep, overriding fan again")
                self.on_set_enable(self)
            last_time = time.time()
            if self.is_changed:
                self.save(self)
                self.is_changed = False
            if self.settings["enable"]:
                # custom fan curve is enabled
                self.do_fan_control(self)
            await asyncio.sleep(self.period_s)

def thermal_zone(index: int) -> float:
    with open(f"/sys/class/thermal/thermal_zone{index}/temp", "r") as f:
        result = float(f.read().strip()) / 1000.0
        logging.debug(f"Got {result}'C from thermal_zone{index}")
        return result

def set_fan_target(rpm: int):
    logging.debug(f"Setting fan1_target to {rpm}")
    with open("/sys/class/hwmon/hwmon5/fan1_target", "w") as f:
        f.write(str(rpm))

def get_fan_input() -> int:
    with open("/sys/class/hwmon/hwmon5/fan1_input", "r") as f:
        rpm = int(f.read().strip())
        #logging.debug(f"Got {rpm} from fan1_input") # this is too spammy; runs every 0.5s
        return rpm

def on_enable():
    with open("/sys/class/hwmon/hwmon5/recalculate", "w") as f:
        f.write("1")
    # TODO stop system fan control

def on_disable():
    with open("/sys/class/hwmon/hwmon5/recalculate", "w") as f:
        f.write("0")
    # TODO restart system fan control
