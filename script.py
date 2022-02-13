import json
import html
import os

times = {}
# folder containing XX:XX.json files
path_to_files = "/tmp/literature-clock/docs/times"

for json_file in os.listdir(path_to_files):
    with open(os.path.join(path_to_files, json_file), "r") as f:
        timestamp = json_file.split(".")[0].replace("_", ":")
        times[timestamp] = [
            {
                "start": html.unescape(d["quote_first"]),
                "time": html.unescape(d["quote_time_case"]),
                "end": html.unescape(d["quote_last"]),
                "title": d["title"],
                "author": d["author"],
            }
            for d in json.load(f)
        ]

with open("./src/times.json", "w") as f:
    json.dump(times, f, indent=2, sort_keys=True)
