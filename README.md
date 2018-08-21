# Litime

Literature time clock to use in your terminal. I personaly run it when I open a new terminal, showing me a quote for the current time.

It's very hacked together, just as fun for myself. I used to have a python script doing this, which was 10 times slower than the resulting rust binary. I want to potentially look into how to make it even faster, just to learn more about Rust.

### Example

![example](example.png)

### Benchmark with hyperfine
```
Benchmark #1: litime

  Time (mean ± σ):      12.3 ms ±   0.8 ms    [User: 5.3 ms, System: 3.8 ms]

  Range (min … max):    11.3 ms …  16.3 ms

Benchmark #2: python script.py

  Time (mean ± σ):     146.9 ms ±   8.9 ms    [User: 91.6 ms, System: 48.1 ms]

  Range (min … max):   137.1 ms … 164.3 ms
```


### Attributions
Built on top of the data from [JohannesNE/literature-clock](https://github.com/JohannesNE/literature-clock), though I think the data in this repo is out of date. Quick and dirty script I used to generate the `minutes.rs` file:

```python
from collections import OrderedDict

for json_file in os.listdir('/path/to/times'):  # folder containing XX:XX.json files
    with open(os.path.join('/Users/axel/Projects/literature-console/times', json_file), 'r') as f:
        times[json_file.split('.')[0]] = json.load(f)

times2 = OrderedDict(sorted(times.items()))

with open('/tmp/rusted', 'w') as f:
    for k,v in times2.items():
        f.write(f'minutes.insert("{k}", &[\n')
        for item in v:
            first = item['quote_first'].replace('"', '\\"')
            time = item['quote_time_case'].replace('"', '\\"')
            rest = item['quote_last'].replace('"', '\\"')
            author = item['author'].replace('"', '\\"')
            title = item['title'].replace('"', '\\"')
            f.write(f'&["{first}", "{time}", "{rest}", "{author}", "{title}"],\n')
        f.write(']);\n')
```
