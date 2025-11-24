# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.91.1 (ed61e7d7e 2025-11-07)

## Metric

```bash
cargo bench --bench metric -- --quiet
```

```text
counter(u64)::inc/prometheus
                        time:   [2.1615 ns 2.1634 ns 2.1655 ns]
counter(u64)::inc/prometheus_client
                        time:   [2.1166 ns 2.1188 ns 2.1212 ns]
counter(u64)::inc/fastmetrics
                        time:   [2.1175 ns 2.1197 ns 2.1223 ns]

counter(f64)::inc/prometheus
                        time:   [10.536 ns 10.551 ns 10.567 ns]
counter(f64)::inc/prometheus_client
                        time:   [5.5838 ns 5.5969 ns 5.6123 ns]
counter(f64)::inc/fastmetrics
                        time:   [5.6345 ns 5.6461 ns 5.6596 ns]

gauge(i64)::set/prometheus
                        time:   [691.22 ps 719.23 ps 754.92 ps]
gauge(i64)::set/prometheus_client
                        time:   [2.3370 ns 2.3589 ns 2.3793 ns]
gauge(i64)::set/fastmetrics
                        time:   [677.60 ps 690.99 ps 705.61 ps]

gauge(i64)::inc_by/prometheus
                        time:   [2.2397 ns 2.2569 ns 2.2753 ns]
gauge(i64)::inc_by/prometheus_client
                        time:   [2.5937 ns 2.6242 ns 2.6533 ns]
gauge(i64)::inc_by/fastmetrics
                        time:   [2.9189 ns 3.0247 ns 3.1626 ns]

gauge(i64)::dec_by/prometheus
                        time:   [2.4448 ns 2.5756 ns 2.7779 ns]
gauge(i64)::dec_by/prometheus_client
                        time:   [2.8179 ns 2.9059 ns 3.0266 ns]
gauge(i64)::dec_by/fastmetrics
                        time:   [2.8108 ns 2.8599 ns 2.9305 ns]

gauge(f64)::set/prometheus
                        time:   [940.22 ps 1.0051 ns 1.0839 ns]
gauge(f64)::set/prometheus_client
                        time:   [2.5071 ns 2.6039 ns 2.7399 ns]
gauge(f64)::set/fastmetrics
                        time:   [840.98 ps 859.14 ps 876.30 ps]

gauge(f64)::inc_by/prometheus
                        time:   [10.989 ns 11.652 ns 12.424 ns]
gauge(f64)::inc_by/prometheus_client
                        time:   [7.0475 ns 7.7202 ns 8.4645 ns]
gauge(f64)::inc_by/fastmetrics
                        time:   [6.4496 ns 6.5563 ns 6.7080 ns]

gauge(f64)::dec_by/prometheus
                        time:   [10.954 ns 11.071 ns 11.273 ns]
gauge(f64)::dec_by/prometheus_client
                        time:   [6.4019 ns 6.4387 ns 6.4769 ns]
gauge(f64)::dec_by/fastmetrics
                        time:   [6.4818 ns 6.5158 ns 6.5485 ns]

histogram::observe/prometheus
                        time:   [11.180 ns 11.227 ns 11.280 ns]
histogram::observe/prometheus_client
                        time:   [9.0995 ns 9.1258 ns 9.1508 ns]
histogram::observe/fastmetrics
                        time:   [5.9017 ns 5.9294 ns 5.9554 ns]
```

## Metric Family

```bash
cargo bench --bench family -- --quiet
```

```text
family without labels/prometheus
                        time:   [24.284 ns 24.318 ns 24.353 ns]
family without labels/prometheus_client
                        time:   [26.026 ns 26.106 ns 26.210 ns]
family without labels/fastmetrics
                        time:   [17.200 ns 17.311 ns 17.475 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [65.875 ns 66.180 ns 66.589 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [48.565 ns 48.744 ns 48.925 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [83.833 ns 84.236 ns 84.683 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [64.354 ns 64.576 ns 64.849 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [102.33 ns 102.88 ns 103.53 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [82.031 ns 82.355 ns 82.715 ns]

family with custom labels/prometheus
                        time:   [25.454 ns 25.537 ns 25.626 ns]
family with custom labels/prometheus_client
                        time:   [38.946 ns 39.085 ns 39.247 ns]
family with custom labels/fastmetrics
                        time:   [19.556 ns 19.597 ns 19.638 ns]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
```

```text
text::encode/prometheus: 10 metrics * 100 observe times
                        time:   [551.15 µs 554.26 µs 557.45 µs]
text::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [350.53 µs 362.60 µs 386.05 µs]
text::encode/fastmetrics: 10 metrics * 100 observe times
                        time:   [234.94 µs 236.57 µs 238.16 µs]

text::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [601.89 µs 619.22 µs 646.27 µs]
text::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [373.60 µs 374.76 µs 375.98 µs]
text::encode/fastmetrics: 10 metrics * 1000 observe times
                        time:   [256.78 µs 258.37 µs 260.38 µs]

text::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [640.03 µs 671.26 µs 716.71 µs]
text::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [380.58 µs 382.99 µs 386.01 µs]
text::encode/fastmetrics: 10 metrics * 10000 observe times
                        time:   [259.72 µs 264.00 µs 270.18 µs]

text::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [660.36 µs 680.49 µs 705.76 µs]
text::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [373.99 µs 376.30 µs 378.97 µs]
text::encode/fastmetrics: 10 metrics * 100000 observe times
                        time:   [271.11 µs 281.33 µs 296.93 µs]

text::encode/prometheus: 100 metrics * 100 observe times
                        time:   [6.0934 ms 6.2655 ms 6.5617 ms]
text::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [3.6173 ms 3.6371 ms 3.6622 ms]
text::encode/fastmetrics: 100 metrics * 100 observe times
                        time:   [2.4547 ms 2.4662 ms 2.4803 ms]

text::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [6.6667 ms 6.8684 ms 7.1428 ms]
text::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [3.8892 ms 3.9231 ms 3.9787 ms]
text::encode/fastmetrics: 100 metrics * 1000 observe times
                        time:   [2.7102 ms 2.7958 ms 2.9135 ms]

text::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [6.7495 ms 6.9321 ms 7.2535 ms]
text::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [3.9532 ms 3.9804 ms 4.0137 ms]
text::encode/fastmetrics: 100 metrics * 10000 observe times
                        time:   [2.7539 ms 2.9217 ms 3.2306 ms]

text::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [7.1597 ms 7.4924 ms 7.9327 ms]
text::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [3.8042 ms 3.8185 ms 3.8329 ms]
text::encode/fastmetrics: 100 metrics * 100000 observe times
                        time:   [2.7359 ms 2.8251 ms 2.9515 ms]
```

## Protobuf Encoding

- prometheus: use the [protobuf](https://crates.io/crates/protobuf) crate for protobuf encoding
- prometheus-client: use the [prost](https://crates.io/crates/prost) crate for protobuf encoding
- fastmetrics: use [prost](https://crates.io/crates/prost) or [protobuf](https://crates.io/crates/protobuf) crate for protobuf encoding

```bash
cargo bench --bench protobuf  -- --quiet
```

```text
protobuf::encode/prometheus: 10 metrics * 100 observe times
                        time:   [181.78 µs 182.90 µs 184.09 µs]
protobuf::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [229.22 µs 230.42 µs 231.67 µs]
protobuf::encode/fastmetrics(prost): 10 metrics * 100 observe times
                        time:   [239.86 µs 241.66 µs 243.46 µs]
protobuf::encode/fastmetrics(protobuf): 10 metrics * 100 observe times
                        time:   [237.10 µs 238.58 µs 240.03 µs]

protobuf::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [200.93 µs 204.41 µs 209.46 µs]
protobuf::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [243.32 µs 244.15 µs 245.02 µs]
protobuf::encode/fastmetrics(prost): 10 metrics * 1000 observe times
                        time:   [258.18 µs 264.10 µs 272.53 µs]
protobuf::encode/fastmetrics(protobuf): 10 metrics * 1000 observe times
                        time:   [254.93 µs 256.04 µs 257.09 µs]

protobuf::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [195.62 µs 196.40 µs 197.25 µs]
protobuf::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [250.50 µs 251.88 µs 253.55 µs]
protobuf::encode/fastmetrics(prost): 10 metrics * 10000 observe times
                        time:   [257.27 µs 258.23 µs 259.28 µs]
protobuf::encode/fastmetrics(protobuf): 10 metrics * 10000 observe times
                        time:   [259.08 µs 260.54 µs 262.38 µs]

protobuf::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [196.40 µs 197.04 µs 197.71 µs]
protobuf::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [257.44 µs 267.57 µs 280.13 µs]
protobuf::encode/fastmetrics(prost): 10 metrics * 100000 observe times
                        time:   [264.34 µs 267.95 µs 273.69 µs]
protobuf::encode/fastmetrics(protobuf): 10 metrics * 100000 observe times
                        time:   [267.02 µs 270.28 µs 274.97 µs]

protobuf::encode/prometheus: 100 metrics * 100 observe times
                        time:   [2.0137 ms 2.0264 ms 2.0390 ms]
protobuf::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [2.4012 ms 2.4107 ms 2.4204 ms]
protobuf::encode/fastmetrics(prost): 100 metrics * 100 observe times
                        time:   [2.5893 ms 2.6040 ms 2.6194 ms]
protobuf::encode/fastmetrics(protobuf): 100 metrics * 100 observe times
                        time:   [2.5524 ms 2.5611 ms 2.5702 ms]

protobuf::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [2.2243 ms 2.2862 ms 2.3888 ms]
protobuf::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [2.5749 ms 2.5859 ms 2.5968 ms]
protobuf::encode/fastmetrics(prost): 100 metrics * 1000 observe times
                        time:   [2.7564 ms 2.8330 ms 2.9463 ms]
protobuf::encode/fastmetrics(protobuf): 100 metrics * 1000 observe times
                        time:   [2.7142 ms 2.7320 ms 2.7547 ms]

protobuf::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [2.1769 ms 2.2145 ms 2.2762 ms]
protobuf::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [2.5723 ms 2.5877 ms 2.6054 ms]
protobuf::encode/fastmetrics(prost): 100 metrics * 10000 observe times
                        time:   [2.7025 ms 2.7151 ms 2.7279 ms]
protobuf::encode/fastmetrics(protobuf): 100 metrics * 10000 observe times
                        time:   [2.8068 ms 2.8329 ms 2.8688 ms]

protobuf::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [2.1755 ms 2.1928 ms 2.2116 ms]
protobuf::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [2.7004 ms 2.7599 ms 2.8587 ms]
protobuf::encode/fastmetrics(prost): 100 metrics * 100000 observe times
                        time:   [2.8570 ms 2.9685 ms 3.1246 ms]
protobuf::encode/fastmetrics(protobuf): 100 metrics * 100000 observe times
                        time:   [2.9041 ms 3.0137 ms 3.1634 ms]
```
