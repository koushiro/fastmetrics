# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.86.0 (05f9846f8 2025-03-31)

## Metric Family

```bash
cargo bench --bench family -- --quiet
```

```text
family without labels/prometheus
                        time:   [33.892 ns 34.000 ns 34.121 ns]
family without labels/prometheus_client
                        time:   [26.457 ns 26.760 ns 27.283 ns]
family without labels/openmetrics_client
                        time:   [18.210 ns 18.263 ns 18.316 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [67.667 ns 67.963 ns 68.304 ns]
family with [(&'static str, &'static str)] labels/openmetrics_client
                        time:   [40.364 ns 40.594 ns 40.866 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [90.230 ns 90.585 ns 90.972 ns]
family with Vec<(&'static str, &'static str)> labels/openmetrics_client
                        time:   [70.500 ns 70.838 ns 71.259 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [108.00 ns 108.44 ns 108.91 ns]
family with Vec<(String, String)> labels/openmetrics_client
                        time:   [91.864 ns 92.217 ns 92.558 ns]

family with custom labels/prometheus
                        time:   [38.708 ns 38.793 ns 38.871 ns]
family with custom labels/prometheus_client
                        time:   [39.469 ns 39.673 ns 39.924 ns]
family with custom labels/openmetrics_client
                        time:   [20.370 ns 20.474 ns 20.587 ns]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
```

```text
text::encode/prometheus: 10 metrics * 100 observe times
                        time:   [579.82 µs 587.91 µs 597.58 µs]
text::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [341.26 µs 343.34 µs 345.45 µs]
text::encode/openmetrics_client: 10 metrics * 100 observe times
                        time:   [243.95 µs 245.36 µs 246.91 µs]
                        
text::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [623.17 µs 659.48 µs 712.31 µs]
text::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [369.54 µs 371.00 µs 372.52 µs]
text::encode/openmetrics_client: 10 metrics * 1000 observe times
                        time:   [261.52 µs 262.52 µs 263.62 µs]
                        
text::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [638.57 µs 641.48 µs 645.58 µs]
text::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [369.11 µs 370.38 µs 371.90 µs]
text::encode/openmetrics_client: 10 metrics * 10000 observe times
                        time:   [261.81 µs 262.77 µs 263.81 µs]
                        
text::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [661.69 µs 664.34 µs 667.66 µs]
text::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [364.96 µs 367.57 µs 372.11 µs]
text::encode/openmetrics_client: 10 metrics * 100000 observe times
                        time:   [264.39 µs 265.76 µs 267.30 µs]
                        
text::encode/prometheus: 100 metrics * 100 observe times
                        time:   [5.9829 ms 6.0088 ms 6.0374 ms]
text::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [3.4660 ms 3.4820 ms 3.4998 ms]
text::encode/openmetrics_client: 100 metrics * 100 observe times
                        time:   [2.4744 ms 2.4821 ms 2.4901 ms]
                        
text::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [6.5339 ms 6.5579 ms 6.5839 ms]
text::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [3.7492 ms 3.7770 ms 3.8080 ms]
text::encode/openmetrics_client: 100 metrics * 1000 observe times
                        time:   [2.6679 ms 2.6772 ms 2.6868 ms]
                        
text::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [6.7451 ms 6.7659 ms 6.7882 ms]
text::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [3.7628 ms 3.7808 ms 3.8023 ms]
text::encode/openmetrics_client: 100 metrics * 10000 observe times
                        time:   [2.6993 ms 2.7102 ms 2.7212 ms]
                        
text::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [7.0661 ms 7.0939 ms 7.1245 ms]
text::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [3.7048 ms 3.7349 ms 3.7751 ms]
text::encode/openmetrics_client: 100 metrics * 100000 observe times
                        time:   [2.6799 ms 2.6898 ms 2.7005 ms]
```

## Protobuf Encoding

- prometheus: use [protobuf](https://crates.io/crates/protobuf) as protobuf encoding 
- prometheus-client: use [prost](https://crates.io/crates/prost) as protobuf encoding
- openmetrics-client: use [prost](https://crates.io/crates/prost) as protobuf encoding

```bash
cargo bench --bench protobuf  -- --quiet
```

```text
protobuf::encode/prometheus: 10 metrics * 100 observe times
                        time:   [183.60 µs 184.58 µs 185.61 µs]
protobuf::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [232.86 µs 233.94 µs 235.02 µs]
protobuf::encode/openmetrics_client: 10 metrics * 100 observe times
                        time:   [189.19 µs 190.41 µs 191.88 µs]
                        
protobuf::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [197.01 µs 197.63 µs 198.28 µs]
protobuf::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [250.92 µs 252.51 µs 254.67 µs]
protobuf::encode/openmetrics_client: 10 metrics * 1000 observe times
                        time:   [200.92 µs 201.48 µs 202.06 µs]
                        
protobuf::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [198.82 µs 199.80 µs 201.10 µs]
protobuf::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [251.24 µs 253.03 µs 255.92 µs]
protobuf::encode/openmetrics_client: 10 metrics * 10000 observe times
                        time:   [205.46 µs 205.95 µs 206.46 µs]
                        
protobuf::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [199.16 µs 199.66 µs 200.20 µs]
protobuf::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [253.58 µs 254.41 µs 255.22 µs]
protobuf::encode/openmetrics_client: 10 metrics * 100000 observe times
                        time:   [207.17 µs 207.69 µs 208.22 µs]
                        
protobuf::encode/prometheus: 100 metrics * 100 observe times
                        time:   [1.9506 ms 1.9631 ms 1.9781 ms]
protobuf::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [2.3946 ms 2.4033 ms 2.4126 ms]
protobuf::encode/openmetrics_client: 100 metrics * 100 observe times
                        time:   [2.0516 ms 2.0726 ms 2.0964 ms]
                        
protobuf::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [2.1666 ms 2.1797 ms 2.1929 ms]
protobuf::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [2.6237 ms 2.6325 ms 2.6414 ms]
protobuf::encode/openmetrics_client: 100 metrics * 1000 observe times
                        time:   [2.2170 ms 2.2317 ms 2.2470 ms]
                        
protobuf::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [2.3175 ms 2.3504 ms 2.3842 ms]
protobuf::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [2.6357 ms 2.6626 ms 2.6930 ms]
protobuf::encode/openmetrics_client: 100 metrics * 10000 observe times
                        time:   [2.2176 ms 2.2336 ms 2.2519 ms]
                        
protobuf::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [2.1776 ms 2.2021 ms 2.2312 ms]
protobuf::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [2.6523 ms 2.6668 ms 2.6822 ms]
protobuf::encode/openmetrics_client: 100 metrics * 100000 observe times
                        time:   [2.2574 ms 2.2789 ms 2.3095 ms]
```