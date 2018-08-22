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
Built on top of the data from [JohannesNE/literature-clock](https://github.com/JohannesNE/literature-clock). See `script.py` for quick and dirty script to update.
