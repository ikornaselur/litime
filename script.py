# flake8: noqa
import json
import os
from collections import OrderedDict

times = {}
# folder containing XX:XX.json files
path_to_files = "/tmp/literature-clock/docs/times"

for json_file in os.listdir(path_to_files):
    with open(os.path.join(path_to_files, json_file), "r") as f:
        time_stamp = "_".join(json_file.split(".")[0].split("_"))
        times[time_stamp.replace("_", ":")] = [
            {
                "start": d["quote_first"],
                "time": d["quote_time_case"],
                "end": d["quote_last"],
                "title": d["title"],
                "author": d["author"],
            }
            for d in json.load(f)
        ]

with open("./src/times.json", "w") as f:
    json.dump(times, f)
