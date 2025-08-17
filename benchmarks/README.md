# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.89.0 (29483883e 2025-08-04)

## Metric

```bash
cargo bench --bench metric -- --quiet
```

```text
counter(u64)::inc/prometheus
                        time:   [2.1833 ns 2.2041 ns 2.2264 ns]
counter(u64)::inc/prometheus_client
                        time:   [2.1664 ns 2.1712 ns 2.1770 ns]
counter(u64)::inc/fastmetrics
                        time:   [2.1612 ns 2.1675 ns 2.1747 ns]

counter(f64)::inc/prometheus
                        time:   [10.658 ns 10.672 ns 10.689 ns]
counter(f64)::inc/prometheus_client
                        time:   [5.6210 ns 5.6285 ns 5.6360 ns]
counter(f64)::inc/fastmetrics
                        time:   [5.8563 ns 6.1546 ns 6.5252 ns]

gauge(i64)::set/prometheus
                        time:   [969.88 ps 986.92 ps 1.0057 ns]
gauge(i64)::set/prometheus_client
                        time:   [2.8733 ns 2.9181 ns 2.9750 ns]
gauge(i64)::set/fastmetrics
                        time:   [907.27 ps 923.35 ps 939.66 ps]

gauge(i64)::inc_by/prometheus
                        time:   [2.4215 ns 2.4440 ns 2.4664 ns]
gauge(i64)::inc_by/prometheus_client
                        time:   [2.9821 ns 3.0105 ns 3.0461 ns]
gauge(i64)::inc_by/fastmetrics
                        time:   [2.9549 ns 2.9787 ns 3.0031 ns]

gauge(i64)::dec_by/prometheus
                        time:   [2.4392 ns 2.4535 ns 2.4685 ns]
gauge(i64)::dec_by/prometheus_client
                        time:   [3.0007 ns 3.0507 ns 3.1134 ns]
gauge(i64)::dec_by/fastmetrics
                        time:   [2.9178 ns 2.9371 ns 2.9552 ns]

gauge(f64)::set/prometheus
                        time:   [1.1049 ns 1.1663 ns 1.2446 ns]
gauge(f64)::set/prometheus_client
                        time:   [2.8167 ns 2.8414 ns 2.8650 ns]
gauge(f64)::set/fastmetrics
                        time:   [1.0540 ns 1.0671 ns 1.0799 ns]

gauge(f64)::inc_by/prometheus
                        time:   [11.029 ns 11.064 ns 11.096 ns]
gauge(f64)::inc_by/prometheus_client
                        time:   [6.4439 ns 6.4709 ns 6.4976 ns]
gauge(f64)::inc_by/fastmetrics
                        time:   [6.3718 ns 6.4056 ns 6.4381 ns]

gauge(f64)::dec_by/prometheus
                        time:   [10.923 ns 10.965 ns 11.006 ns]
gauge(f64)::dec_by/prometheus_client
                        time:   [6.4156 ns 6.5225 ns 6.7109 ns]
gauge(f64)::dec_by/fastmetrics
                        time:   [6.3611 ns 6.3892 ns 6.4158 ns]

histogram::observe/prometheus
                        time:   [11.298 ns 11.840 ns 12.629 ns]
histogram::observe/prometheus_client
                        time:   [9.1247 ns 9.1670 ns 9.2179 ns]
histogram::observe/fastmetrics
                        time:   [9.0561 ns 9.4978 ns 10.165 ns]
```

## Metric Family

```bash
cargo bench --bench family -- --quiet
```

```text
family without labels/prometheus
                        time:   [25.740 ns 27.433 ns 29.600 ns]
family without labels/prometheus_client
                        time:   [27.214 ns 27.365 ns 27.533 ns]
family without labels/fastmetrics
                        time:   [18.407 ns 18.495 ns 18.589 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [70.673 ns 73.303 ns 76.868 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [42.143 ns 42.416 ns 42.739 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [94.057 ns 102.96 ns 116.73 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [73.438 ns 74.104 ns 74.832 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [113.35 ns 118.93 ns 126.26 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [94.405 ns 94.900 ns 95.413 ns]

family with custom labels/prometheus
                        time:   [26.815 ns 26.984 ns 27.159 ns]
family with custom labels/prometheus_client
                        time:   [40.202 ns 40.366 ns 40.518 ns]
family with custom labels/fastmetrics
                        time:   [20.406 ns 20.529 ns 20.660 ns]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
```

```text
text::encode/prometheus: 10 metrics * 100 observe times
                        time:   [608.34 µs 677.85 µs 768.24 µs]
text::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [388.58 µs 403.26 µs 422.01 µs]
text::encode/fastmetrics: 10 metrics * 100 observe times
                        time:   [278.89 µs 294.66 µs 317.45 µs]

text::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [767.41 µs 821.03 µs 895.43 µs]
text::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [389.65 µs 398.24 µs 411.38 µs]
text::encode/fastmetrics: 10 metrics * 1000 observe times
                        time:   [270.39 µs 280.27 µs 296.75 µs]

text::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [665.02 µs 694.49 µs 744.22 µs]
text::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [387.14 µs 389.95 µs 393.69 µs]
text::encode/fastmetrics: 10 metrics * 10000 observe times
                        time:   [270.54 µs 272.07 µs 273.77 µs]

text::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [663.88 µs 693.48 µs 755.06 µs]
text::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [381.29 µs 383.04 µs 384.92 µs]
text::encode/fastmetrics: 10 metrics * 100000 observe times
                        time:   [271.79 µs 273.79 µs 276.28 µs]

text::encode/prometheus: 100 metrics * 100 observe times
                        time:   [6.1232 ms 6.3659 ms 6.7912 ms]
text::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [3.6384 ms 3.6561 ms 3.6739 ms]
text::encode/fastmetrics: 100 metrics * 100 observe times
                        time:   [2.5756 ms 2.6860 ms 2.8759 ms]

text::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [6.6056 ms 6.6443 ms 6.6846 ms]
text::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [3.8864 ms 4.0139 ms 4.1965 ms]
text::encode/fastmetrics: 100 metrics * 1000 observe times
                        time:   [2.7320 ms 2.7455 ms 2.7594 ms]

text::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [6.8943 ms 7.0203 ms 7.1839 ms]
text::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [3.9389 ms 3.9851 ms 4.0566 ms]
text::encode/fastmetrics: 100 metrics * 10000 observe times
                        time:   [2.7906 ms 2.8613 ms 2.9870 ms]

text::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [7.2137 ms 7.3618 ms 7.5774 ms]
text::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [3.8908 ms 3.9168 ms 3.9435 ms]
text::encode/fastmetrics: 100 metrics * 100000 observe times
                        time:   [2.7882 ms 2.8191 ms 2.8671 ms]
```

## Protobuf Encoding

- prometheus: use the [protobuf](https://crates.io/crates/protobuf) crate for protobuf encoding 
- prometheus-client: use the [prost](https://crates.io/crates/prost) crate for protobuf encoding
- fastmetrics: use the [prost](https://crates.io/crates/prost) crate for protobuf encoding

```bash
cargo bench --bench protobuf  -- --quiet
```

```text
protobuf::encode/prometheus: 10 metrics * 100 observe times
                        time:   [194.25 µs 200.49 µs 210.41 µs]
protobuf::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [254.68 µs 256.65 µs 258.79 µs]
protobuf::encode/fastmetrics: 10 metrics * 100 observe times
                        time:   [241.37 µs 246.07 µs 254.66 µs]

protobuf::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [207.33 µs 209.87 µs 213.19 µs]
protobuf::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [275.51 µs 276.64 µs 277.77 µs]
protobuf::encode/fastmetrics: 10 metrics * 1000 observe times
                        time:   [258.48 µs 259.95 µs 261.57 µs]

protobuf::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [205.94 µs 206.93 µs 208.01 µs]
protobuf::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [281.20 µs 284.21 µs 288.19 µs]
protobuf::encode/fastmetrics: 10 metrics * 10000 observe times
                        time:   [264.02 µs 271.38 µs 285.94 µs]

protobuf::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [209.31 µs 214.65 µs 224.31 µs]
protobuf::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [283.38 µs 287.29 µs 293.00 µs]
protobuf::encode/fastmetrics: 10 metrics * 100000 observe times
                        time:   [267.91 µs 271.03 µs 275.49 µs]

protobuf::encode/prometheus: 100 metrics * 100 observe times
                        time:   [2.1532 ms 2.1721 ms 2.1918 ms]
protobuf::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [2.6858 ms 2.7050 ms 2.7262 ms]
protobuf::encode/fastmetrics: 100 metrics * 100 observe times
                        time:   [2.5773 ms 2.5923 ms 2.6074 ms]

protobuf::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [2.3533 ms 2.4045 ms 2.4791 ms]
protobuf::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [2.8818 ms 2.8970 ms 2.9125 ms]
protobuf::encode/fastmetrics: 100 metrics * 1000 observe times
                        time:   [2.7454 ms 2.7610 ms 2.7771 ms]

protobuf::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [2.3346 ms 2.3669 ms 2.4140 ms]
protobuf::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [2.9340 ms 2.9546 ms 2.9808 ms]
protobuf::encode/fastmetrics: 100 metrics * 10000 observe times
                        time:   [2.8051 ms 2.8570 ms 2.9432 ms]

protobuf::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [2.3481 ms 2.3666 ms 2.3856 ms]
protobuf::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [2.9807 ms 2.9971 ms 3.0142 ms]
protobuf::encode/fastmetrics: 100 metrics * 100000 observe times
                        time:   [2.8576 ms 2.8956 ms 2.9445 ms]
```
