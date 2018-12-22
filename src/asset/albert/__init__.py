# -*- coding: utf-8 -*-

import os
import subprocess

from albertv0 import *

__iid__ = "PythonInterface/v0.2"
__prettyname__ = "{{prettyname}}"
__version__ = "{{version}}"
__trigger__ = "{{trigger}}"
__author__ = "{{author}}"
__dependencies__ = []

iconPath = os.path.join(os.path.dirname(__file__), 'icon.png')
pathlist = ["/usr/local/bin", "~/.local/bin", "~/.cargo/bin"]

def handleQuery(query):
    if not query.isTriggered:
        return None

    if len(query.string) <= 1:
        return None

    os.environ["PATH"] += os.pathsep + os.pathsep.join(pathlist)
    cmd = ["url"] + query.string.split()
    pipes = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    items = []
    if pipes.returncode != 0:
        err_msg = "%s. exit code: %s" % (pipes.stderr.strip().decode("utf-8"), pipes.returncode)
        items.append(Item(
            id = __prettyname__,
            icon = iconPath,
            text = err_msg,
            subtext = "Failed",
            actions = []
        ))
    else:
        out = pipes.stdout.decode("utf-8")
        items.append(Item(
            id = __prettyname__,
            icon = iconPath,
            text = out,
            subtext = "Success",
            actions = [
                ClipAction("Added to Clipboard", out)
            ]
        ))
    return items

