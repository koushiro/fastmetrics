# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.87.0 (17067e9ac 2025-05-09)

## Metric

```bash
cargo bench --bench metric -- --quiet
```

```text
counter(u64)::inc/prometheus
                        time:   [2.1586 ns 2.1749 ns 2.1979 ns]
counter(u64)::inc/prometheus_client
                        time:   [2.1523 ns 2.1762 ns 2.2195 ns]
counter(u64)::inc/fastmetrics
                        time:   [2.1427 ns 2.1710 ns 2.2251 ns]

counter(f64)::inc/prometheus
                        time:   [10.696 ns 10.820 ns 11.079 ns]
counter(f64)::inc/prometheus_client
                        time:   [5.6320 ns 5.6725 ns 5.7500 ns]
counter(f64)::inc/fastmetrics
                        time:   [5.7029 ns 5.7812 ns 5.9195 ns]

gauge(i64)::set/prometheus
                        time:   [936.06 ps 950.45 ps 965.44 ps]
gauge(i64)::set/prometheus_client
                        time:   [2.7662 ns 2.8079 ns 2.8586 ns]
gauge(i64)::set/fastmetrics
                        time:   [949.19 ps 970.00 ps 999.50 ps]

gauge(i64)::inc_by/prometheus
                        time:   [2.3797 ns 2.4204 ns 2.4765 ns]
gauge(i64)::inc_by/prometheus_client
                        time:   [2.8480 ns 2.8708 ns 2.8941 ns]
gauge(i64)::inc_by/fastmetrics
                        time:   [2.8654 ns 2.9029 ns 2.9545 ns]

gauge(i64)::dec_by/prometheus
                        time:   [2.3859 ns 2.4058 ns 2.4253 ns]
gauge(i64)::dec_by/prometheus_client
                        time:   [2.9074 ns 2.9344 ns 2.9696 ns]
gauge(i64)::dec_by/fastmetrics
                        time:   [2.9154 ns 2.9569 ns 3.0128 ns]

gauge(f64)::set/prometheus
                        time:   [1.0872 ns 1.0999 ns 1.1130 ns]
gauge(f64)::set/prometheus_client
                        time:   [2.7899 ns 2.8072 ns 2.8232 ns]
gauge(f64)::set/fastmetrics
                        time:   [1.0949 ns 1.1111 ns 1.1286 ns]

gauge(f64)::inc_by/prometheus
                        time:   [10.945 ns 10.989 ns 11.035 ns]
gauge(f64)::inc_by/prometheus_client
                        time:   [6.3071 ns 6.3973 ns 6.5513 ns]
gauge(f64)::inc_by/fastmetrics
                        time:   [6.3271 ns 6.3752 ns 6.4350 ns]

gauge(f64)::dec_by/prometheus
                        time:   [10.958 ns 11.032 ns 11.138 ns]
gauge(f64)::dec_by/prometheus_client
                        time:   [6.3737 ns 6.4471 ns 6.5645 ns]
gauge(f64)::dec_by/fastmetrics
                        time:   [6.3128 ns 6.4021 ns 6.5562 ns]

histogram::observe/prometheus
                        time:   [11.258 ns 11.419 ns 11.717 ns]
histogram::observe/prometheus_client
                        time:   [9.1316 ns 9.2092 ns 9.3083 ns]
histogram::observe/fastmetrics
                        time:   [9.0976 ns 9.2077 ns 9.3906 ns]
```

## Metric Family

```bash
cargo bench --bench family -- --quiet
```

```text
family without labels/prometheus
                        time:   [24.775 ns 24.825 ns 24.882 ns]
family without labels/prometheus_client
                        time:   [26.341 ns 26.396 ns 26.451 ns]
family without labels/fastmetrics
                        time:   [18.111 ns 18.379 ns 18.670 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [66.728 ns 66.941 ns 67.163 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [40.825 ns 41.172 ns 41.552 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [87.573 ns 87.755 ns 87.933 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [70.534 ns 70.853 ns 71.183 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [106.77 ns 107.37 ns 108.07 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [91.430 ns 91.864 ns 92.387 ns]

family with custom labels/prometheus
                        time:   [25.859 ns 25.952 ns 26.077 ns]
family with custom labels/prometheus_client
                        time:   [39.069 ns 39.176 ns 39.280 ns]
family with custom labels/fastmetrics
                        time:   [19.821 ns 21.654 ns 24.334 ns]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
```

```text
text::encode/prometheus: 10 metrics * 100 observe times
                        time:   [589.44 µs 595.47 µs 602.40 µs]
text::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [349.90 µs 352.39 µs 355.27 µs]
text::encode/fastmetrics: 10 metrics * 100 observe times
                        time:   [263.61 µs 266.75 µs 271.21 µs]

text::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [627.68 µs 632.70 µs 641.40 µs]
text::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [371.86 µs 374.60 µs 378.16 µs]
text::encode/fastmetrics: 10 metrics * 1000 observe times
                        time:   [282.01 µs 283.51 µs 285.23 µs]

text::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [646.32 µs 654.37 µs 667.53 µs]
text::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [375.40 µs 379.50 µs 385.62 µs]
text::encode/fastmetrics: 10 metrics * 10000 observe times
                        time:   [283.11 µs 286.25 µs 291.97 µs]

text::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [672.15 µs 680.24 µs 692.40 µs]
text::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [369.38 µs 371.26 µs 373.32 µs]
text::encode/fastmetrics: 10 metrics * 100000 observe times
                        time:   [282.10 µs 286.21 µs 292.38 µs]

text::encode/prometheus: 100 metrics * 100 observe times
                        time:   [6.1584 ms 6.2121 ms 6.2939 ms]
text::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [3.5421 ms 3.5731 ms 3.6180 ms]
text::encode/fastmetrics: 100 metrics * 100 observe times
                        time:   [2.7111 ms 2.7363 ms 2.7760 ms]

text::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [6.7785 ms 6.8406 ms 6.9305 ms]
text::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [3.7931 ms 3.8346 ms 3.9054 ms]
text::encode/fastmetrics: 100 metrics * 1000 observe times
                        time:   [2.9063 ms 2.9299 ms 2.9601 ms]

text::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [6.9645 ms 6.9917 ms 7.0184 ms]
text::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [3.8259 ms 3.8473 ms 3.8728 ms]
text::encode/fastmetrics: 100 metrics * 10000 observe times
                        time:   [2.9342 ms 2.9637 ms 3.0084 ms]

text::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [7.2893 ms 7.3160 ms 7.3427 ms]
text::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [3.8268 ms 3.8457 ms 3.8656 ms]
text::encode/fastmetrics: 100 metrics * 100000 observe times
                        time:   [2.9398 ms 2.9534 ms 2.9681 ms]
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
                        time:   [184.27 µs 186.90 µs 190.82 µs]
protobuf::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [240.15 µs 244.05 µs 251.04 µs]
protobuf::encode/fastmetrics: 10 metrics * 100 observe times
                        time:   [231.21 µs 235.20 µs 241.64 µs]

protobuf::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [202.20 µs 205.42 µs 209.75 µs]
protobuf::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [255.53 µs 258.27 µs 262.80 µs]
protobuf::encode/fastmetrics: 10 metrics * 1000 observe times
                        time:   [245.18 µs 246.64 µs 248.27 µs]

protobuf::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [197.81 µs 201.75 µs 207.13 µs]
protobuf::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [259.60 µs 262.07 µs 265.87 µs]
protobuf::encode/fastmetrics: 10 metrics * 10000 observe times
                        time:   [246.02 µs 248.51 µs 252.56 µs]

protobuf::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [198.14 µs 200.74 µs 205.23 µs]
protobuf::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [264.48 µs 267.91 µs 273.96 µs]
protobuf::encode/fastmetrics: 10 metrics * 100000 observe times
                        time:   [258.09 µs 262.83 µs 269.90 µs]

protobuf::encode/prometheus: 100 metrics * 100 observe times
                        time:   [1.9939 ms 2.0084 ms 2.0236 ms]
protobuf::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [2.5247 ms 2.5526 ms 2.5964 ms]
protobuf::encode/fastmetrics: 100 metrics * 100 observe times
                        time:   [2.4333 ms 2.4641 ms 2.5120 ms]

protobuf::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [2.1970 ms 2.2274 ms 2.2721 ms]
protobuf::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [2.7262 ms 2.7539 ms 2.8000 ms]
protobuf::encode/fastmetrics: 100 metrics * 1000 observe times
                        time:   [2.6236 ms 2.6499 ms 2.6891 ms]

protobuf::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [2.2148 ms 2.2442 ms 2.2920 ms]
protobuf::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [2.6654 ms 2.7130 ms 2.7803 ms]
protobuf::encode/fastmetrics: 100 metrics * 10000 observe times
                        time:   [2.7044 ms 2.7367 ms 2.7817 ms]

protobuf::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [2.2316 ms 2.2725 ms 2.3400 ms]
protobuf::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [2.8279 ms 2.8721 ms 2.9304 ms]
protobuf::encode/fastmetrics: 100 metrics * 100000 observe times
                        time:   [2.7238 ms 2.7607 ms 2.8228 ms]
```