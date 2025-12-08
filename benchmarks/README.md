# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.91.1 (ed61e7d7e 2025-11-07)

## Metric

```bash
cargo bench --bench metric -- --quiet
# Or `just bench metric`
```

```text
counter(u64)::inc/metrics
                        time:   [7.1270 ns 7.1384 ns 7.1506 ns]
counter(u64)::inc/prometheus
                        time:   [2.1899 ns 2.1941 ns 2.1987 ns]
counter(u64)::inc/prometheus_client
                        time:   [2.1172 ns 2.1211 ns 2.1258 ns]
counter(u64)::inc/fastmetrics
                        time:   [2.1952 ns 2.2020 ns 2.2112 ns]

counter(f64)::inc/metrics
                        time:   [7.1194 ns 7.1392 ns 7.1620 ns]
counter(f64)::inc/prometheus
                        time:   [10.690 ns 10.713 ns 10.739 ns]
counter(f64)::inc/prometheus_client
                        time:   [5.5507 ns 5.5619 ns 5.5772 ns]
counter(f64)::inc/fastmetrics
                        time:   [5.5960 ns 5.6098 ns 5.6330 ns]

gauge(i64)::set/metrics time:   [6.6429 ns 6.6910 ns 6.7368 ns]
gauge(i64)::set/prometheus
                        time:   [789.78 ps 798.80 ps 807.67 ps]
gauge(i64)::set/prometheus_client
                        time:   [2.3401 ns 2.3646 ns 2.3887 ns]
gauge(i64)::set/fastmetrics
                        time:   [827.70 ps 838.31 ps 849.33 ps]

gauge(i64)::inc_by/metrics
                        time:   [7.0746 ns 7.1019 ns 7.1309 ns]
gauge(i64)::inc_by/prometheus
                        time:   [2.4369 ns 2.4532 ns 2.4704 ns]
gauge(i64)::inc_by/prometheus_client
                        time:   [2.8679 ns 2.8961 ns 2.9242 ns]
gauge(i64)::inc_by/fastmetrics
                        time:   [2.4130 ns 2.4262 ns 2.4396 ns]

gauge(i64)::dec_by/metrics
                        time:   [7.1075 ns 7.1278 ns 7.1492 ns]
gauge(i64)::dec_by/prometheus
                        time:   [2.4388 ns 2.4579 ns 2.4772 ns]
gauge(i64)::dec_by/prometheus_client
                        time:   [2.9235 ns 2.9502 ns 2.9760 ns]
gauge(i64)::dec_by/fastmetrics
                        time:   [2.4577 ns 2.4748 ns 2.4920 ns]

gauge(f64)::set/metrics time:   [6.7196 ns 6.7665 ns 6.8131 ns]
gauge(f64)::set/prometheus
                        time:   [868.07 ps 881.22 ps 894.80 ps]
gauge(f64)::set/prometheus_client
                        time:   [2.3830 ns 2.4115 ns 2.4434 ns]
gauge(f64)::set/fastmetrics
                        time:   [857.25 ps 872.51 ps 889.04 ps]

gauge(f64)::inc_by/metrics
                        time:   [7.1795 ns 7.2122 ns 7.2513 ns]
gauge(f64)::inc_by/prometheus
                        time:   [11.011 ns 11.052 ns 11.092 ns]
gauge(f64)::inc_by/prometheus_client
                        time:   [6.4218 ns 6.4595 ns 6.4992 ns]
gauge(f64)::inc_by/fastmetrics
                        time:   [6.0514 ns 6.1270 ns 6.2651 ns]

gauge(f64)::dec_by/metrics
                        time:   [7.1298 ns 7.1813 ns 7.2641 ns]
gauge(f64)::dec_by/prometheus
                        time:   [11.046 ns 11.104 ns 11.185 ns]
gauge(f64)::dec_by/prometheus_client
                        time:   [6.4431 ns 6.4836 ns 6.5221 ns]
gauge(f64)::dec_by/fastmetrics
                        time:   [6.0016 ns 6.0397 ns 6.0962 ns]

histogram::observe/metrics
                        time:   [11.025 ns 11.364 ns 11.872 ns]
histogram::observe/prometheus
                        time:   [11.184 ns 11.259 ns 11.357 ns]
histogram::observe/prometheus_client
                        time:   [9.2102 ns 9.2880 ns 9.4241 ns]
histogram::observe/fastmetrics
                        time:   [6.0025 ns 6.0282 ns 6.0566 ns]
```

## Metric Family

```bash
cargo bench --bench family -- --quiet
# Or `just bench family`
```

```text
family without labels/metrics
                        time:   [13.340 ns 13.556 ns 13.831 ns]
family without labels/prometheus
                        time:   [23.849 ns 24.166 ns 24.687 ns]
family without labels/prometheus_client
                        time:   [26.238 ns 26.318 ns 26.406 ns]
family without labels/fastmetrics
                        time:   [17.006 ns 17.043 ns 17.078 ns]

family with custom labels/metrics
                        time:   [150.20 ns 150.44 ns 150.67 ns]
family with custom labels/prometheus
                        time:   [25.489 ns 25.705 ns 25.927 ns]
family with custom labels/prometheus_client
                        time:   [39.338 ns 39.903 ns 40.608 ns]
family with custom labels/fastmetrics
                        time:   [20.509 ns 20.567 ns 20.627 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [67.077 ns 67.352 ns 67.725 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [51.074 ns 51.201 ns 51.344 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [84.203 ns 85.271 ns 86.641 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [66.479 ns 67.462 ns 68.736 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [104.44 ns 105.51 ns 106.78 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [87.777 ns 88.282 ns 88.839 ns]

family concurrent new metric creation/prometheus_client
                        time:   [833.15 µs 841.25 µs 850.05 µs]
family concurrent new metric creation/fastmetrics
                        time:   [766.11 µs 774.26 µs 783.60 µs]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
# Or `just bench text`
```

```text
text::encode/metrics_exporter_prometheus: 10 metrics * 100 observe times
                        time:   [6.8168 ms 7.2350 ms 7.6886 ms]
text::encode/prometheus: 10 metrics * 100 observe times
                        time:   [529.66 µs 533.18 µs 536.96 µs]
text::encode/prometheus_client: 10 metrics * 100 observe times
                        time:   [328.21 µs 330.03 µs 331.89 µs]
text::encode/fastmetrics: 10 metrics * 100 observe times
                        time:   [221.52 µs 223.04 µs 224.47 µs]

text::encode/metrics_exporter_prometheus: 10 metrics * 1000 observe times
                        time:   [17.855 ms 18.509 ms 19.130 ms]
text::encode/prometheus: 10 metrics * 1000 observe times
                        time:   [582.06 µs 583.74 µs 585.57 µs]
text::encode/prometheus_client: 10 metrics * 1000 observe times
                        time:   [350.96 µs 351.62 µs 352.31 µs]
text::encode/fastmetrics: 10 metrics * 1000 observe times
                        time:   [238.99 µs 239.52 µs 240.16 µs]

text::encode/metrics_exporter_prometheus: 10 metrics * 10000 observe times
                        time:   [17.998 ms 18.948 ms 20.044 ms]
text::encode/prometheus: 10 metrics * 10000 observe times
                        time:   [599.79 µs 601.34 µs 602.89 µs]
text::encode/prometheus_client: 10 metrics * 10000 observe times
                        time:   [355.59 µs 356.46 µs 357.39 µs]
text::encode/fastmetrics: 10 metrics * 10000 observe times
                        time:   [242.51 µs 243.01 µs 243.56 µs]

text::encode/metrics_exporter_prometheus: 10 metrics * 100000 observe times
                        time:   [16.585 ms 17.279 ms 17.909 ms]
text::encode/prometheus: 10 metrics * 100000 observe times
                        time:   [618.30 µs 619.60 µs 620.98 µs]
text::encode/prometheus_client: 10 metrics * 100000 observe times
                        time:   [346.88 µs 347.45 µs 348.12 µs]
text::encode/fastmetrics: 10 metrics * 100000 observe times
                        time:   [240.79 µs 241.06 µs 241.38 µs]

text::encode/metrics_exporter_prometheus: 100 metrics * 100 observe times
                        time:   [102.14 ms 115.99 ms 129.86 ms]
text::encode/prometheus: 100 metrics * 100 observe times
                        time:   [5.5308 ms 5.5530 ms 5.5769 ms]
text::encode/prometheus_client: 100 metrics * 100 observe times
                        time:   [3.2893 ms 3.2963 ms 3.3042 ms]
text::encode/fastmetrics: 100 metrics * 100 observe times
                        time:   [2.2420 ms 2.2458 ms 2.2499 ms]

text::encode/metrics_exporter_prometheus: 100 metrics * 1000 observe times
                        time:   [185.15 ms 203.35 ms 221.47 ms]
text::encode/prometheus: 100 metrics * 1000 observe times
                        time:   [6.0535 ms 6.0745 ms 6.0968 ms]
text::encode/prometheus_client: 100 metrics * 1000 observe times
                        time:   [3.5428 ms 3.5483 ms 3.5544 ms]
text::encode/fastmetrics: 100 metrics * 1000 observe times
                        time:   [2.4158 ms 2.4205 ms 2.4258 ms]

text::encode/metrics_exporter_prometheus: 100 metrics * 10000 observe times
                        time:   [201.02 ms 228.88 ms 256.94 ms]
text::encode/prometheus: 100 metrics * 10000 observe times
                        time:   [6.3015 ms 6.3326 ms 6.3645 ms]
text::encode/prometheus_client: 100 metrics * 10000 observe times
                        time:   [3.5946 ms 3.6116 ms 3.6319 ms]
text::encode/fastmetrics: 100 metrics * 10000 observe times
                        time:   [2.4583 ms 2.4639 ms 2.4698 ms]

text::encode/metrics_exporter_prometheus: 100 metrics * 100000 observe times
                        time:   [202.65 ms 216.75 ms 230.47 ms]
text::encode/prometheus: 100 metrics * 100000 observe times
                        time:   [6.5933 ms 6.6324 ms 6.6721 ms]
text::encode/prometheus_client: 100 metrics * 100000 observe times
                        time:   [3.5687 ms 3.6028 ms 3.6529 ms]
text::encode/fastmetrics: 100 metrics * 100000 observe times
                        time:   [2.4917 ms 2.5082 ms 2.5356 ms]
```

## Protobuf Encoding

- metrics-exporter-prometheus: use the [prost] crate for (prometheus) protobuf encoding
- prometheus: use the [protobuf] crate for (prometheus) protobuf encoding
- prometheus-client: use the [prost] crate for (openmetrics) protobuf encoding
- fastmetrics: use [prost] or [protobuf] crate for (openmetrics) protobuf encoding

[prost]: https://crates.io/crates/prost
[protobuf]: https://crates.io/crates/protobuf

```bash
cargo bench --bench protobuf  -- --quiet
# Or `just bench protobuf`
```

```text
protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 10 metrics * 100 observe times
                        time:   [7.3115 ms 8.0613 ms 9.0183 ms]
protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 100 observe times
                        time:   [178.65 µs 182.13 µs 186.35 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 100 observe times
                        time:   [234.95 µs 237.84 µs 242.32 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 100 observe times
                        time:   [232.26 µs 233.68 µs 235.20 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100 observe times
                        time:   [220.97 µs 222.25 µs 223.61 µs]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 10 metrics * 1000 observe times
                        time:   [15.887 ms 16.500 ms 17.067 ms]
protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 1000 observe times
                        time:   [188.92 µs 189.43 µs 190.06 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 1000 observe times
                        time:   [251.04 µs 251.68 µs 252.35 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 1000 observe times
                        time:   [246.76 µs 248.26 µs 251.03 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 1000 observe times
                        time:   [241.70 µs 242.35 µs 243.06 µs]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 10 metrics * 10000 observe times
                        time:   [18.858 ms 19.071 ms 19.337 ms]
protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 10000 observe times
                        time:   [189.07 µs 189.46 µs 189.87 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 10000 observe times
                        time:   [251.46 µs 252.02 µs 252.63 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 10000 observe times
                        time:   [248.33 µs 249.23 µs 250.20 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 10000 observe times
                        time:   [246.69 µs 248.60 µs 251.50 µs]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 10 metrics * 100000 observe times
                        time:   [18.276 ms 19.050 ms 19.783 ms]
protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 100000 observe times
                        time:   [188.17 µs 189.37 µs 191.52 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 100000 observe times
                        time:   [255.07 µs 257.92 µs 261.54 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 100000 observe times
                        time:   [250.82 µs 252.60 µs 255.62 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100000 observe times
                        time:   [248.83 µs 251.43 µs 255.54 µs]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 100 metrics * 100 observe times
                        time:   [106.51 ms 113.94 ms 121.83 ms]
protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 100 observe times
                        time:   [1.9258 ms 1.9403 ms 1.9587 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 100 observe times
                        time:   [2.4944 ms 2.5092 ms 2.5248 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 100 observe times
                        time:   [2.4464 ms 2.4668 ms 2.4974 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100 observe times
                        time:   [2.4645 ms 2.4739 ms 2.4833 ms]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 100 metrics * 1000 observe times
                        time:   [197.35 ms 207.91 ms 218.42 ms]
protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 1000 observe times
                        time:   [2.0851 ms 2.1041 ms 2.1314 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 1000 observe times
                        time:   [2.6555 ms 2.6767 ms 2.7087 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 1000 observe times
                        time:   [2.6418 ms 2.6753 ms 2.7186 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 1000 observe times
                        time:   [2.7179 ms 2.7432 ms 2.7769 ms]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 100 metrics * 10000 observe times
                        time:   [199.01 ms 209.91 ms 222.69 ms]
protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 10000 observe times
                        time:   [2.1172 ms 2.1334 ms 2.1524 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 10000 observe times
                        time:   [2.7084 ms 2.7327 ms 2.7643 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 10000 observe times
                        time:   [2.7230 ms 2.7514 ms 2.7930 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 10000 observe times
                        time:   [2.7462 ms 2.7636 ms 2.7831 ms]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 100 metrics * 100000 observe times
                        time:   [202.08 ms 211.29 ms 220.22 ms]
protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 100000 observe times
                        time:   [2.1173 ms 2.1312 ms 2.1458 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 100000 observe times
                        time:   [2.7166 ms 2.7373 ms 2.7683 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 100000 observe times
                        time:   [2.7103 ms 2.7257 ms 2.7479 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100000 observe times
                        time:   [2.6938 ms 2.7152 ms 2.7462 ms]
```
