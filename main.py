import json
import os
import asyncio

import logging

logging.basicConfig(
    filename = "/home/deck/.fantastic.log",
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
DATA_SAVE_FOLDER = "/home/deck/.config/fantastic/"
DATA_SAVE_PATH = DATA_SAVE_FOLDER + DATA_SAVE_FILE

DEFAULT_DATA = {
    "version": 0,
    "enable": False,
    "interpolate": False,
    "curve": [],
}

class Plugin:
    settings = None
    is_changed = False

    plot_width = 1;
    plot_height = 1;

    period_s = 1.0;

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
        on_set_enable(enable)
        self.is_changed = True

    async def get_enable(self) -> bool:
        await self.wait_for_ready(self)
        return self.settings["enable"]

    async def set_plot_size(self, x, y):
        logging.debug(f"Set plot size to ({x},{y})")
        self.plot_width = x
        self.plot_height = y

    async def set_poll_period(self, period):
        self.period_s = period

    def save(self):
        if not os.path.exists(DATA_SAVE_FOLDER):
            os.mkdir(DATA_SAVE_FOLDER)
        with open(DATA_SAVE_PATH, "w") as data_file :
            json.dump(self.settings, data_file)

    async def wait_for_ready(self):
        while self.settings is None:
            await asyncio.sleep(0.01)

    def do_fan_control(self):
        index = -1
        curve = self.settings["curve"]
        temperature = (thermal_zone(0) - TEMPERATURE_MINIMUM) / TEMPERATURE_MAXIMUM
        for i in range(len(curve)-1, -1, -1):
            if curve[i]["x"] < temperature:
                index = i
                break
        if index != -1:
            target_speed = ((1 - curve[index]["y"]) * FAN_MAXIMUM) + FAN_MINIMUM
            set_fan_target(int(target_speed))
        else:
            if len(curve) == 0:
                set_fan_target(int(FAN_MAXIMUM))
            else:
                set_fan_target(int((FAN_MINIMUM + FAN_MAXIMUM) / 2))

    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded
    async def _main(self):
        # startup
        if os.path.exists(DATA_SAVE_PATH):
            with open(DATA_SAVE_PATH, "r") as data_file:
                self.settings = json.load(data_file)
        else:
            self.settings = dict(DEFAULT_DATA)
        try:
            self.settings["version"]
        except:
            self.settings = dict(DEFAULT_DATA)
        while self.settings["version"] != DEFAULT_DATA["version"]:
            # TODO specific upgrade functionality
            self.settings["version"] = DEFAULT_DATA["version"]
            self.settings["enable"] = DEFAULT_DATA["enable"]
            self.settings["interpolate"] = DEFAULT_DATA["interpolate"]
            self.settings["curve"] = DEFAULT_DATA["curve"]
            self.is_changed = True
        on_set_enable(self.settings["enable"])
        # work loop
        while True:
            if self.is_changed:
                self.save(self)
                self.is_changed = False
            if self.settings["enable"]:
                # custom fan curve is enabled
                self.do_fan_control(self)
            await asyncio.sleep(self.period_s)

def thermal_zone(index: int):
    with open(f"/sys/class/thermal/thermal_zone{index}/temp", "r") as f:
        result = float(f.read().strip()) / 1000.0
        logging.debug(f"Got {result}'C from thermal_zone{index}")
        return result

def set_fan_target(rpm: int):
    logging.debug(f"Setting fan1_target to {rpm}")
    with open("/sys/class/hwmon/hwmon5/fan1_target", "w") as f:
        f.write(str(rpm))

def on_enable():
    with open("/sys/class/hwmon/hwmon5/recalculate", "w") as f:
        f.write("1")
    # TODO disable system fan control

def on_disable():
    with open("/sys/class/hwmon/hwmon5/recalculate", "w") as f:
        f.write("0")
    # TODO restart system fan control

def on_set_enable(enable):
    if enable:
        on_enable()
    else:
        on_disable()
