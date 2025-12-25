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
                        time:   [5.1321 ms 5.2405 ms 5.3624 ms]
text::encode/measured: 10 metrics * 100 times
                        time:   [467.63 µs 469.59 µs 471.59 µs]
text::encode/prometheus: 10 metrics * 100 times
                        time:   [534.45 µs 537.27 µs 540.12 µs]
text::encode/prometheus_client: 10 metrics * 100 times
                        time:   [338.12 µs 339.71 µs 341.36 µs]
text::encode/fastmetrics: 10 metrics * 100 times
                        time:   [185.44 µs 186.48 µs 187.59 µs]

text::encode/metrics_exporter_prometheus: 10 metrics * 1000 times
                        time:   [10.501 ms 10.631 ms 10.749 ms]
text::encode/measured: 10 metrics * 1000 times
                        time:   [503.37 µs 504.58 µs 505.94 µs]
text::encode/prometheus: 10 metrics * 1000 times
                        time:   [589.58 µs 596.55 µs 606.59 µs]
text::encode/prometheus_client: 10 metrics * 1000 times
                        time:   [363.59 µs 366.05 µs 370.49 µs]
text::encode/fastmetrics: 10 metrics * 1000 times
                        time:   [201.27 µs 201.99 µs 202.95 µs]

text::encode/metrics_exporter_prometheus: 10 metrics * 10000 times
                        time:   [10.949 ms 11.089 ms 11.235 ms]
text::encode/measured: 10 metrics * 10000 times
                        time:   [507.79 µs 508.79 µs 509.86 µs]
text::encode/prometheus: 10 metrics * 10000 times
                        time:   [596.41 µs 597.90 µs 599.54 µs]
text::encode/prometheus_client: 10 metrics * 10000 times
                        time:   [369.50 µs 370.05 µs 370.72 µs]
text::encode/fastmetrics: 10 metrics * 10000 times
                        time:   [206.41 µs 208.10 µs 210.36 µs]

text::encode/metrics_exporter_prometheus: 10 metrics * 100000 times
                        time:   [11.469 ms 11.612 ms 11.748 ms]
text::encode/measured: 10 metrics * 100000 times
                        time:   [504.93 µs 506.10 µs 507.45 µs]
text::encode/prometheus: 10 metrics * 100000 times
                        time:   [619.00 µs 620.58 µs 622.43 µs]
text::encode/prometheus_client: 10 metrics * 100000 times
                        time:   [359.47 µs 359.96 µs 360.52 µs]
text::encode/fastmetrics: 10 metrics * 100000 times
                        time:   [206.20 µs 207.55 µs 209.98 µs]

text::encode/metrics_exporter_prometheus: 100 metrics * 100 times
                        time:   [53.572 ms 53.791 ms 54.026 ms]
text::encode/measured: 100 metrics * 100 times
                        time:   [4.8093 ms 4.9357 ms 5.1121 ms]
text::encode/prometheus: 100 metrics * 100 times
                        time:   [6.5984 ms 7.3095 ms 8.2859 ms]
text::encode/prometheus_client: 100 metrics * 100 times
                        time:   [3.4611 ms 3.5793 ms 3.7610 ms]
text::encode/fastmetrics: 100 metrics * 100 times
                        time:   [1.9559 ms 2.0539 ms 2.1750 ms]

text::encode/metrics_exporter_prometheus: 100 metrics * 1000 times
                        time:   [106.65 ms 107.51 ms 108.47 ms]
text::encode/measured: 100 metrics * 1000 times
                        time:   [5.1002 ms 5.2577 ms 5.4750 ms]
text::encode/prometheus: 100 metrics * 1000 times
                        time:   [6.2978 ms 6.3709 ms 6.4850 ms]
text::encode/prometheus_client: 100 metrics * 1000 times
                        time:   [3.7598 ms 3.9089 ms 4.1262 ms]
text::encode/fastmetrics: 100 metrics * 1000 times
                        time:   [2.0917 ms 2.0981 ms 2.1051 ms]

text::encode/metrics_exporter_prometheus: 100 metrics * 10000 times
                        time:   [111.24 ms 111.99 ms 112.80 ms]
text::encode/measured: 100 metrics * 10000 times
                        time:   [5.1038 ms 5.1275 ms 5.1560 ms]
text::encode/prometheus: 100 metrics * 10000 times
                        time:   [6.3914 ms 6.4172 ms 6.4479 ms]
text::encode/prometheus_client: 100 metrics * 10000 times
                        time:   [3.7517 ms 3.7599 ms 3.7691 ms]
text::encode/fastmetrics: 100 metrics * 10000 times
                        time:   [2.1246 ms 2.1574 ms 2.2170 ms]

text::encode/metrics_exporter_prometheus: 100 metrics * 100000 times
                        time:   [122.27 ms 130.73 ms 145.04 ms]
text::encode/measured: 100 metrics * 100000 times
                        time:   [5.1042 ms 5.1525 ms 5.2361 ms]
text::encode/prometheus: 100 metrics * 100000 times
                        time:   [6.6520 ms 6.6998 ms 6.7560 ms]
text::encode/prometheus_client: 100 metrics * 100000 times
                        time:   [3.6635 ms 3.7146 ms 3.7936 ms]
text::encode/fastmetrics: 100 metrics * 100000 times
                        time:   [2.1558 ms 2.1825 ms 2.2194 ms]
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
