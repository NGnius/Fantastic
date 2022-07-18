import pathlib
import subprocess
import asyncio
import os

HOME_DIR = str(pathlib.Path(os.getcwd()).parent.parent.resolve())
PARENT_DIR = str(pathlib.Path(__file__).parent.resolve())

LOG_LOCATION = "/tmp/fantastic.py.log"

import logging

logging.basicConfig(
    filename = LOG_LOCATION,
    format = '%(asctime)s %(levelname)s %(message)s',
    filemode = 'w',
    force = True)

logger = logging.getLogger()
logger.setLevel(logging.DEBUG)
logging.info(f"Fantastic main.py https://github.com/NGnius/Fantastic")

class Plugin:
    backend_proc = None
    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded
    async def _main(self):
        # startup
        self.backend_proc = subprocess.Popen([PARENT_DIR + "/bin/backend"])
        while True:
            asyncio.sleep(1)
