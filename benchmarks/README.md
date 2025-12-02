# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.91.1 (ed61e7d7e 2025-11-07)

## Metric

```bash
cargo bench --bench metric -- --quiet
```

```text
counter(u64)::inc/prometheus
                        time:   [2.1470 ns 2.1498 ns 2.1530 ns]
counter(u64)::inc/prometheus_client
                        time:   [2.0752 ns 2.0962 ns 2.1339 ns]
counter(u64)::inc/fastmetrics
                        time:   [2.1464 ns 2.1489 ns 2.1516 ns]

counter(f64)::inc/prometheus
                        time:   [10.483 ns 10.581 ns 10.724 ns]
counter(f64)::inc/prometheus_client
                        time:   [5.4776 ns 5.4827 ns 5.4882 ns]
counter(f64)::inc/fastmetrics
                        time:   [5.5307 ns 5.5382 ns 5.5468 ns]

gauge(i64)::set/prometheus
                        time:   [740.88 ps 754.32 ps 767.06 ps]
gauge(i64)::set/prometheus_client
                        time:   [2.2434 ns 2.2714 ns 2.2990 ns]
gauge(i64)::set/fastmetrics
                        time:   [766.40 ps 783.85 ps 801.44 ps]

gauge(i64)::inc_by/prometheus
                        time:   [2.3247 ns 2.3401 ns 2.3555 ns]
gauge(i64)::inc_by/prometheus_client
                        time:   [2.7579 ns 2.7817 ns 2.8040 ns]
gauge(i64)::inc_by/fastmetrics
                        time:   [2.3400 ns 2.3732 ns 2.4166 ns]

gauge(i64)::dec_by/prometheus
                        time:   [2.3347 ns 2.3486 ns 2.3620 ns]
gauge(i64)::dec_by/prometheus_client
                        time:   [2.7919 ns 2.8191 ns 2.8463 ns]
gauge(i64)::dec_by/fastmetrics
                        time:   [2.3577 ns 2.3811 ns 2.4119 ns]

gauge(f64)::set/prometheus
                        time:   [772.76 ps 786.77 ps 800.70 ps]
gauge(f64)::set/prometheus_client
                        time:   [2.2504 ns 2.2719 ns 2.2924 ns]
gauge(f64)::set/fastmetrics
                        time:   [777.71 ps 792.27 ps 806.47 ps]

gauge(f64)::inc_by/prometheus
                        time:   [10.865 ns 10.906 ns 10.949 ns]
gauge(f64)::inc_by/prometheus_client
                        time:   [6.2856 ns 6.3665 ns 6.4913 ns]
gauge(f64)::inc_by/fastmetrics
                        time:   [5.9085 ns 5.9319 ns 5.9550 ns]

gauge(f64)::dec_by/prometheus
                        time:   [10.798 ns 10.835 ns 10.872 ns]
gauge(f64)::dec_by/prometheus_client
                        time:   [6.2904 ns 6.3257 ns 6.3593 ns]
gauge(f64)::dec_by/fastmetrics
                        time:   [5.8670 ns 5.8893 ns 5.9123 ns]

histogram::observe/prometheus
                        time:   [11.001 ns 11.029 ns 11.056 ns]
histogram::observe/prometheus_client
                        time:   [9.0512 ns 9.0744 ns 9.0970 ns]
histogram::observe/fastmetrics
                        time:   [5.9090 ns 5.9338 ns 5.9599 ns]
```

## Metric Family

```bash
cargo bench --bench family -- --quiet
```

```text
family without labels/prometheus
                        time:   [24.566 ns 24.649 ns 24.725 ns]
family without labels/prometheus_client
                        time:   [26.510 ns 26.590 ns 26.668 ns]
family without labels/fastmetrics
                        time:   [17.176 ns 17.225 ns 17.274 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [67.419 ns 67.706 ns 68.037 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [49.989 ns 50.064 ns 50.141 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [83.403 ns 83.547 ns 83.699 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [64.944 ns 65.878 ns 67.129 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [101.72 ns 101.97 ns 102.21 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [83.405 ns 83.821 ns 84.261 ns]

family with custom labels/prometheus
                        time:   [25.347 ns 25.401 ns 25.459 ns]
family with custom labels/prometheus_client
                        time:   [39.049 ns 39.182 ns 39.311 ns]
family with custom labels/fastmetrics
                        time:   [19.921 ns 19.955 ns 19.992 ns]

family concurrent new metric creation/prometheus_client
                        time:   [803.34 µs 813.33 µs 824.71 µs]
family concurrent new metric creation/fastmetrics
                        time:   [725.61 µs 735.99 µs 746.70 µs]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
```

```text
text::encode/prometheus: 10 metrics * 100 observe times
                        time:   [539.63 µs 542.47 µs 545.28 µs]
text::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [344.92 µs 346.74 µs 348.65 µs]
text::encode/fastmetrics: 10 metrics * 100 observe times
                        time:   [234.02 µs 235.28 µs 236.59 µs]

text::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [589.50 µs 591.26 µs 593.13 µs]
text::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [372.05 µs 372.85 µs 373.76 µs]
text::encode/fastmetrics: 10 metrics * 1000 observe times
                        time:   [251.82 µs 252.56 µs 253.40 µs]

text::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [601.50 µs 602.94 µs 604.48 µs]
text::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [376.64 µs 377.57 µs 378.55 µs]
text::encode/fastmetrics: 10 metrics * 10000 observe times
                        time:   [256.90 µs 257.90 µs 258.98 µs]

text::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [625.15 µs 626.96 µs 628.97 µs]
text::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [364.22 µs 364.70 µs 365.22 µs]
text::encode/fastmetrics: 10 metrics * 100000 observe times
                        time:   [256.23 µs 256.78 µs 257.37 µs]

text::encode/prometheus: 100 metrics * 100 observe times
                        time:   [5.7002 ms 5.7273 ms 5.7561 ms]
text::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [3.5223 ms 3.5349 ms 3.5480 ms]
text::encode/fastmetrics: 100 metrics * 100 observe times
                        time:   [2.4216 ms 2.4314 ms 2.4415 ms]

text::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [6.2682 ms 6.3134 ms 6.3615 ms]
text::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [3.7883 ms 3.7998 ms 3.8122 ms]
text::encode/fastmetrics: 100 metrics * 1000 observe times
                        time:   [2.5867 ms 2.5971 ms 2.6082 ms]

text::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [6.3810 ms 6.4036 ms 6.4287 ms]
text::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [3.7966 ms 3.8056 ms 3.8153 ms]
text::encode/fastmetrics: 100 metrics * 10000 observe times
                        time:   [2.6038 ms 2.6099 ms 2.6162 ms]

text::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [6.6692 ms 6.6986 ms 6.7311 ms]
text::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [3.7118 ms 3.7242 ms 3.7378 ms]
text::encode/fastmetrics: 100 metrics * 100000 observe times
                        time:   [2.6248 ms 2.6319 ms 2.6393 ms]
```

## Protobuf Encoding

- prometheus: use the [protobuf](https://crates.io/crates/protobuf) crate for (prometheus) protobuf encoding
- prometheus-client: use the [prost](https://crates.io/crates/prost) crate for (openmetrics) protobuf encoding
- fastmetrics: use [prost](https://crates.io/crates/prost) or [protobuf](https://crates.io/crates/protobuf) crate for (openmetrics) protobuf encoding

```bash
cargo bench --bench protobuf  -- --quiet
```

```text
protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 100 observe times
                        time:   [178.66 µs 179.73 µs 180.85 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 100 observe times
                        time:   [225.94 µs 227.38 µs 228.89 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 100 observe times
                        time:   [238.31 µs 239.93 µs 241.57 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100 observe times
                        time:   [228.02 µs 229.17 µs 230.40 µs]

protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 1000 observe times
                        time:   [192.83 µs 193.27 µs 193.74 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 1000 observe times
                        time:   [244.18 µs 244.80 µs 245.47 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 1000 observe times
                        time:   [251.27 µs 252.40 µs 253.51 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 1000 observe times
                        time:   [244.33 µs 245.05 µs 245.85 µs]

protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 10000 observe times
                        time:   [192.62 µs 192.96 µs 193.33 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 10000 observe times
                        time:   [246.08 µs 246.95 µs 247.79 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 10000 observe times
                        time:   [252.07 µs 252.89 µs 253.84 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 10000 observe times
                        time:   [250.27 µs 250.93 µs 251.67 µs]

protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 100000 observe times
                        time:   [192.45 µs 192.89 µs 193.38 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 100000 observe times
                        time:   [248.75 µs 249.59 µs 250.41 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 100000 observe times
                        time:   [257.14 µs 258.25 µs 259.38 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100000 observe times
                        time:   [254.91 µs 255.85 µs 256.87 µs]

protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 100 observe times
                        time:   [1.8924 ms 1.9000 ms 1.9084 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 100 observe times
                        time:   [2.3253 ms 2.3322 ms 2.3392 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 100 observe times
                        time:   [2.4704 ms 2.4853 ms 2.5003 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100 observe times
                        time:   [2.4028 ms 2.4102 ms 2.4180 ms]

protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 1000 observe times
                        time:   [2.3162 ms 2.3519 ms 2.3950 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 1000 observe times
                        time:   [2.5066 ms 2.5159 ms 2.5264 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 1000 observe times
                        time:   [2.6151 ms 2.6269 ms 2.6393 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 1000 observe times
                        time:   [2.6491 ms 2.6606 ms 2.6731 ms]

protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 10000 observe times
                        time:   [2.0620 ms 2.0707 ms 2.0801 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 10000 observe times
                        time:   [2.5355 ms 2.5423 ms 2.5493 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 10000 observe times
                        time:   [2.6353 ms 2.6593 ms 2.6972 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 10000 observe times
                        time:   [2.7152 ms 2.7250 ms 2.7354 ms]

protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 100000 observe times
                        time:   [2.0846 ms 2.0945 ms 2.1056 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 100000 observe times
                        time:   [2.5791 ms 2.5879 ms 2.5972 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 100000 observe times
                        time:   [2.6614 ms 2.6702 ms 2.6796 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100000 observe times
                        time:   [2.7425 ms 2.7514 ms 2.7611 ms]
```
