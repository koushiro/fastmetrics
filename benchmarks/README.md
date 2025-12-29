# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.92.0 (ded5c06cf 2025-12-08)

## Metric

```bash
cargo bench --bench metric -- --quiet
# Or `just bench metric`
```

```text
counter(u64)::inc/metrics
                        time:   [7.1525 ns 7.1636 ns 7.1761 ns]
counter(u64)::inc/measured
                        time:   [2.1671 ns 2.1705 ns 2.1747 ns]
counter(u64)::inc/prometheus
                        time:   [2.2137 ns 2.2297 ns 2.2514 ns]
counter(u64)::inc/prometheus_client
                        time:   [2.1529 ns 2.2310 ns 2.3373 ns]
counter(u64)::inc/fastmetrics
                        time:   [2.2050 ns 2.2301 ns 2.2692 ns]

counter(f64)::inc/metrics
                        time:   [7.1328 ns 7.1433 ns 7.1555 ns]
counter(f64)::inc/prometheus
                        time:   [10.693 ns 10.707 ns 10.724 ns]
counter(f64)::inc/prometheus_client
                        time:   [5.5991 ns 5.6093 ns 5.6208 ns]
counter(f64)::inc/fastmetrics
                        time:   [5.6798 ns 5.6973 ns 5.7214 ns]

gauge(i64)::set/metrics time:   [6.4639 ns 6.5164 ns 6.5738 ns]
gauge(i64)::set/measured
                        time:   [985.63 ps 988.45 ps 991.40 ps]
gauge(i64)::set/prometheus
                        time:   [579.69 ps 581.63 ps 583.73 ps]
gauge(i64)::set/prometheus_client
                        time:   [1.7251 ns 1.7365 ns 1.7474 ns]
gauge(i64)::set/fastmetrics
                        time:   [579.76 ps 582.81 ps 587.01 ps]

gauge(i64)::inc_by/metrics
                        time:   [6.8638 ns 6.8886 ns 6.9161 ns]
gauge(i64)::inc_by/measured
                        time:   [2.1527 ns 2.1631 ns 2.1774 ns]
gauge(i64)::inc_by/prometheus
                        time:   [2.1728 ns 2.1846 ns 2.2042 ns]
gauge(i64)::inc_by/prometheus_client
                        time:   [2.2467 ns 2.2529 ns 2.2593 ns]
gauge(i64)::inc_by/fastmetrics
                        time:   [2.1697 ns 2.1850 ns 2.2115 ns]

gauge(i64)::dec_by/metrics
                        time:   [6.8571 ns 6.8826 ns 6.9249 ns]
gauge(i64)::dec_by/measured
                        time:   [2.1567 ns 2.1696 ns 2.1916 ns]
gauge(i64)::dec_by/prometheus
                        time:   [2.1658 ns 2.1721 ns 2.1790 ns]
gauge(i64)::dec_by/prometheus_client
                        time:   [2.2243 ns 2.2288 ns 2.2336 ns]
gauge(i64)::dec_by/fastmetrics
                        time:   [2.1691 ns 2.1848 ns 2.2094 ns]

gauge(f64)::set/metrics time:   [6.4346 ns 6.4778 ns 6.5189 ns]
gauge(f64)::set/measured
                        time:   [986.55 ps 995.76 ps 1.0111 ns]
gauge(f64)::set/prometheus
                        time:   [586.60 ps 588.40 ps 590.24 ps]
gauge(f64)::set/prometheus_client
                        time:   [1.7002 ns 1.7190 ns 1.7435 ns]
gauge(f64)::set/fastmetrics
                        time:   [585.92 ps 588.42 ps 591.39 ps]

gauge(f64)::inc_by/metrics
                        time:   [6.8456 ns 6.8582 ns 6.8711 ns]
gauge(f64)::inc_by/measured
                        time:   [10.773 ns 10.796 ns 10.821 ns]
gauge(f64)::inc_by/prometheus
                        time:   [10.834 ns 10.984 ns 11.176 ns]
gauge(f64)::inc_by/prometheus_client
                        time:   [5.7706 ns 5.8330 ns 5.9264 ns]
gauge(f64)::inc_by/fastmetrics
                        time:   [5.7656 ns 5.7802 ns 5.7985 ns]

gauge(f64)::dec_by/metrics
                        time:   [6.8898 ns 6.9835 ns 7.1097 ns]
gauge(f64)::dec_by/measured
                        time:   [10.790 ns 10.844 ns 10.922 ns]
gauge(f64)::dec_by/prometheus
                        time:   [10.739 ns 10.773 ns 10.810 ns]
gauge(f64)::dec_by/prometheus_client
                        time:   [5.7367 ns 5.7462 ns 5.7555 ns]
gauge(f64)::dec_by/fastmetrics
                        time:   [5.7269 ns 5.7698 ns 5.8260 ns]

histogram::observe/metrics
                        time:   [10.391 ns 10.489 ns 10.614 ns]
histogram::observe/measured
                        time:   [11.680 ns 11.969 ns 12.393 ns]
histogram::observe/prometheus
                        time:   [10.902 ns 10.973 ns 11.078 ns]
histogram::observe/prometheus_client
                        time:   [8.8961 ns 8.9094 ns 8.9232 ns]
histogram::observe/fastmetrics
                        time:   [5.7252 ns 5.7737 ns 5.8664 ns]
```

## Metric Family

```bash
cargo bench --bench family -- --quiet
# Or `just bench family`
```

```text
family without labels/metrics
                        time:   [12.027 ns 12.372 ns 12.960 ns]
family without labels/prometheus
                        time:   [24.856 ns 24.917 ns 25.005 ns]
family without labels/prometheus_client
                        time:   [27.056 ns 27.140 ns 27.247 ns]
family without labels/fastmetrics
                        time:   [17.179 ns 17.594 ns 18.379 ns]

family with custom labels/metrics
                        time:   [160.59 ns 161.23 ns 161.97 ns]
family with custom labels/measured
                        time:   [11.328 ns 11.381 ns 11.437 ns]
family with custom labels/prometheus
                        time:   [25.758 ns 25.878 ns 26.009 ns]
family with custom labels/prometheus_client
                        time:   [38.976 ns 39.085 ns 39.205 ns]
family with custom labels/fastmetrics
                        time:   [20.186 ns 20.239 ns 20.297 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [68.399 ns 69.215 ns 70.472 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [50.470 ns 50.593 ns 50.736 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [88.559 ns 88.926 ns 89.329 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [63.767 ns 63.909 ns 64.067 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [106.13 ns 106.61 ns 107.24 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [82.139 ns 82.617 ns 83.184 ns]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
# Or `just bench text`
```

Each group of metrics includes a counter (u64) and a histogram.

```text
text::encode/metrics_exporter_prometheus: 10 metrics * 100 times
                        time:   [5.1128 ms 5.2469 ms 5.3891 ms]
text::encode/measured: 10 metrics * 100 times
                        time:   [459.25 µs 461.88 µs 464.57 µs]
text::encode/prometheus: 10 metrics * 100 times
                        time:   [534.58 µs 537.33 µs 539.96 µs]
text::encode/prometheus_client: 10 metrics * 100 times
                        time:   [335.32 µs 336.66 µs 338.03 µs]
text::encode/fastmetrics: 10 metrics * 100 times
                        time:   [177.21 µs 180.86 µs 185.50 µs]

text::encode/metrics_exporter_prometheus: 10 metrics * 1000 times
                        time:   [10.296 ms 10.467 ms 10.647 ms]
text::encode/measured: 10 metrics * 1000 times
                        time:   [491.37 µs 497.02 µs 505.83 µs]
text::encode/prometheus: 10 metrics * 1000 times
                        time:   [585.53 µs 586.64 µs 588.00 µs]
text::encode/prometheus_client: 10 metrics * 1000 times
                        time:   [362.98 µs 363.60 µs 364.33 µs]
text::encode/fastmetrics: 10 metrics * 1000 times
                        time:   [191.28 µs 191.83 µs 192.35 µs]

text::encode/metrics_exporter_prometheus: 10 metrics * 10000 times
                        time:   [10.957 ms 11.138 ms 11.300 ms]
text::encode/measured: 10 metrics * 10000 times
                        time:   [496.12 µs 496.90 µs 497.74 µs]
text::encode/prometheus: 10 metrics * 10000 times
                        time:   [597.48 µs 598.49 µs 599.56 µs]
text::encode/prometheus_client: 10 metrics * 10000 times
                        time:   [368.70 µs 375.18 µs 388.22 µs]
text::encode/fastmetrics: 10 metrics * 10000 times
                        time:   [192.25 µs 192.72 µs 193.16 µs]

text::encode/metrics_exporter_prometheus: 10 metrics * 100000 times
                        time:   [11.269 ms 11.436 ms 11.593 ms]
text::encode/measured: 10 metrics * 100000 times
                        time:   [500.74 µs 502.88 µs 506.18 µs]
text::encode/prometheus: 10 metrics * 100000 times
                        time:   [620.20 µs 621.12 µs 622.06 µs]
text::encode/prometheus_client: 10 metrics * 100000 times
                        time:   [358.24 µs 358.76 µs 359.32 µs]
text::encode/fastmetrics: 10 metrics * 100000 times
                        time:   [193.74 µs 196.06 µs 199.59 µs]

text::encode/metrics_exporter_prometheus: 100 metrics * 100 times
                        time:   [53.606 ms 53.976 ms 54.340 ms]
text::encode/measured: 100 metrics * 100 times
                        time:   [4.6519 ms 4.7210 ms 4.8494 ms]
text::encode/prometheus: 100 metrics * 100 times
                        time:   [5.7552 ms 5.7719 ms 5.7888 ms]
text::encode/prometheus_client: 100 metrics * 100 times
                        time:   [3.4358 ms 3.4425 ms 3.4493 ms]
text::encode/fastmetrics: 100 metrics * 100 times
                        time:   [1.8225 ms 1.8294 ms 1.8359 ms]

text::encode/metrics_exporter_prometheus: 100 metrics * 1000 times
                        time:   [106.77 ms 109.50 ms 112.90 ms]
text::encode/measured: 100 metrics * 1000 times
                        time:   [4.9424 ms 4.9516 ms 4.9612 ms]
text::encode/prometheus: 100 metrics * 1000 times
                        time:   [6.3005 ms 6.3829 ms 6.5066 ms]
text::encode/prometheus_client: 100 metrics * 1000 times
                        time:   [3.7142 ms 3.7192 ms 3.7243 ms]
text::encode/fastmetrics: 100 metrics * 1000 times
                        time:   [1.9628 ms 1.9679 ms 1.9724 ms]

text::encode/metrics_exporter_prometheus: 100 metrics * 10000 times
                        time:   [110.85 ms 111.41 ms 112.11 ms]
text::encode/measured: 100 metrics * 10000 times
                        time:   [5.0009 ms 5.0130 ms 5.0280 ms]
text::encode/prometheus: 100 metrics * 10000 times
                        time:   [6.4255 ms 6.4802 ms 6.5739 ms]
text::encode/prometheus_client: 100 metrics * 10000 times
                        time:   [3.7569 ms 3.7627 ms 3.7686 ms]
text::encode/fastmetrics: 100 metrics * 10000 times
                        time:   [1.9814 ms 1.9909 ms 2.0029 ms]

text::encode/metrics_exporter_prometheus: 100 metrics * 100000 times
                        time:   [117.01 ms 118.69 ms 121.21 ms]
text::encode/measured: 100 metrics * 100000 times
                        time:   [5.0311 ms 5.0409 ms 5.0506 ms]
text::encode/prometheus: 100 metrics * 100000 times
                        time:   [6.6319 ms 6.6503 ms 6.6687 ms]
text::encode/prometheus_client: 100 metrics * 100000 times
                        time:   [3.6289 ms 3.6342 ms 3.6396 ms]
text::encode/fastmetrics: 100 metrics * 100000 times
                        time:   [1.9816 ms 2.0197 ms 2.0963 ms]
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

Each group of metrics includes a counter (u64) and a histogram.

```text
protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 10 metrics * 100 times
                        time:   [7.3115 ms 8.0613 ms 9.0183 ms]
protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 100 times
                        time:   [178.65 µs 182.13 µs 186.35 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 100 times
                        time:   [234.95 µs 237.84 µs 242.32 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 100 times
                        time:   [232.26 µs 233.68 µs 235.20 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100 times
                        time:   [220.97 µs 222.25 µs 223.61 µs]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 10 metrics * 1000 times
                        time:   [15.887 ms 16.500 ms 17.067 ms]
protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 1000 times
                        time:   [188.92 µs 189.43 µs 190.06 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 1000 times
                        time:   [251.04 µs 251.68 µs 252.35 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 1000 times
                        time:   [246.76 µs 248.26 µs 251.03 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 1000 times
                        time:   [241.70 µs 242.35 µs 243.06 µs]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 10 metrics * 10000 times
                        time:   [18.858 ms 19.071 ms 19.337 ms]
protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 10000 times
                        time:   [189.07 µs 189.46 µs 189.87 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 10000 times
                        time:   [251.46 µs 252.02 µs 252.63 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 10000 times
                        time:   [248.33 µs 249.23 µs 250.20 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 10000 times
                        time:   [246.69 µs 248.60 µs 251.50 µs]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 10 metrics * 100000 times
                        time:   [18.276 ms 19.050 ms 19.783 ms]
protobuf::encode/prometheus(protobuf/prometheus): 10 metrics * 100000 times
                        time:   [188.17 µs 189.37 µs 191.52 µs]
protobuf::encode/prometheus_client(prost/openmetrics): 10 metrics * 100000 times
                        time:   [255.07 µs 257.92 µs 261.54 µs]
protobuf::encode/fastmetrics(prost/openmetrics): 10 metrics * 100000 times
                        time:   [250.82 µs 252.60 µs 255.62 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100000 times
                        time:   [248.83 µs 251.43 µs 255.54 µs]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 100 metrics * 100 times
                        time:   [106.51 ms 113.94 ms 121.83 ms]
protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 100 times
                        time:   [1.9258 ms 1.9403 ms 1.9587 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 100 times
                        time:   [2.4944 ms 2.5092 ms 2.5248 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 100 times
                        time:   [2.4464 ms 2.4668 ms 2.4974 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100 times
                        time:   [2.4645 ms 2.4739 ms 2.4833 ms]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 100 metrics * 1000 times
                        time:   [197.35 ms 207.91 ms 218.42 ms]
protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 1000 times
                        time:   [2.0851 ms 2.1041 ms 2.1314 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 1000 times
                        time:   [2.6555 ms 2.6767 ms 2.7087 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 1000 times
                        time:   [2.6418 ms 2.6753 ms 2.7186 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 1000 times
                        time:   [2.7179 ms 2.7432 ms 2.7769 ms]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 100 metrics * 10000 times
                        time:   [199.01 ms 209.91 ms 222.69 ms]
protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 10000 times
                        time:   [2.1172 ms 2.1334 ms 2.1524 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 10000 times
                        time:   [2.7084 ms 2.7327 ms 2.7643 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 10000 times
                        time:   [2.7230 ms 2.7514 ms 2.7930 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 10000 times
                        time:   [2.7462 ms 2.7636 ms 2.7831 ms]

protobuf::encode/metrics_exporter_prometheus(prost/prometheus): 100 metrics * 100000 times
                        time:   [202.08 ms 211.29 ms 220.22 ms]
protobuf::encode/prometheus(protobuf/prometheus): 100 metrics * 100000 times
                        time:   [2.1173 ms 2.1312 ms 2.1458 ms]
protobuf::encode/prometheus_client(prost/openmetrics): 100 metrics * 100000 times
                        time:   [2.7166 ms 2.7373 ms 2.7683 ms]
protobuf::encode/fastmetrics(prost/openmetrics): 100 metrics * 100000 times
                        time:   [2.7103 ms 2.7257 ms 2.7479 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100000 times
                        time:   [2.6938 ms 2.7152 ms 2.7462 ms]
```
