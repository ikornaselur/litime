import json
import os
from collections import OrderedDict

times = {}
# folder containing XX:XX.json files
path_to_files = "/tmp/literature-clock/docs/times"

for json_file in os.listdir(path_to_files):
    with open(os.path.join(path_to_files, json_file), "r") as f:
        time_stamp = "_".join(json_file.split(".")[0].split("_"))
        times[time_stamp] = json.load(f)

times2 = OrderedDict(sorted(times.items()))

with open("/tmp/rusted", "w") as f:
    for k, v in times2.items():
        f.write(f'minutes.insert("{k}", &[\n')
        for item in v:
            first = item["quote_first"].replace('"', '\\"')
            time = item["quote_time_case"].replace('"', '\\"')
            rest = item["quote_last"].replace('"', '\\"')
            author = item["author"].replace('"', '\\"')
            title = item["title"].replace('"', '\\"')
            f.write(f'&["{first}", "{time}", "{rest}", "{author}", "{title}"],\n')
        f.write("]);\n")
