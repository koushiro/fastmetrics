# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.93.0 (254b59607 2026-01-19)

## Metric

```bash
cargo bench --bench metric -- --quiet
# Or `just bench metric`
```

```text
counter(u64)::inc/metrics
                        time:   [7.1701 ns 7.1929 ns 7.2269 ns]
counter(u64)::inc/measured
                        time:   [2.1799 ns 2.2218 ns 2.3099 ns]
counter(u64)::inc/prometheus
                        time:   [2.2067 ns 2.2344 ns 2.2827 ns]
counter(u64)::inc/prometheus_client
                        time:   [2.1296 ns 2.1533 ns 2.1980 ns]
counter(u64)::inc/fastmetrics
                        time:   [2.2060 ns 2.2085 ns 2.2112 ns]

counter(u64)::saturating_inc/fastmetrics
                        time:   [2.4990 ns 2.5028 ns 2.5066 ns]

counter(f64)::inc/metrics
                        time:   [7.1656 ns 7.1736 ns 7.1822 ns]
counter(f64)::inc/prometheus
                        time:   [10.758 ns 10.961 ns 11.381 ns]
counter(f64)::inc/prometheus_client
                        time:   [5.6103 ns 5.6167 ns 5.6232 ns]
counter(f64)::inc/fastmetrics
                        time:   [5.6669 ns 5.7429 ns 5.8925 ns]

gauge(i64)::set/metrics time:   [6.4548 ns 6.5041 ns 6.5539 ns]
gauge(i64)::set/measured
                        time:   [987.40 ps 992.07 ps 997.34 ps]
gauge(i64)::set/prometheus
                        time:   [582.40 ps 586.00 ps 589.92 ps]
gauge(i64)::set/prometheus_client
                        time:   [1.6853 ns 1.6986 ns 1.7138 ns]
gauge(i64)::set/fastmetrics
                        time:   [582.01 ps 585.49 ps 589.64 ps]

gauge(i64)::inc_by/metrics
                        time:   [6.8636 ns 6.9125 ns 7.0066 ns]
gauge(i64)::inc_by/measured
                        time:   [2.1678 ns 2.1752 ns 2.1825 ns]
gauge(i64)::inc_by/prometheus
                        time:   [2.1674 ns 2.1747 ns 2.1821 ns]
gauge(i64)::inc_by/prometheus_client
                        time:   [2.2545 ns 2.2651 ns 2.2755 ns]
gauge(i64)::inc_by/fastmetrics
                        time:   [2.1702 ns 2.1917 ns 2.2227 ns]

gauge(i64)::saturating_inc_by/fastmetrics
                        time:   [3.1359 ns 3.1726 ns 3.2193 ns]

gauge(i64)::dec_by/metrics
                        time:   [6.8622 ns 6.8784 ns 6.8966 ns]
gauge(i64)::dec_by/measured
                        time:   [2.1534 ns 2.1618 ns 2.1719 ns]
gauge(i64)::dec_by/prometheus
                        time:   [2.1767 ns 2.1990 ns 2.2294 ns]
gauge(i64)::dec_by/prometheus_client
                        time:   [2.2337 ns 2.2838 ns 2.3661 ns]
gauge(i64)::dec_by/fastmetrics
                        time:   [2.1700 ns 2.2096 ns 2.2679 ns]

gauge(i64)::saturating_dec_by/fastmetrics
                        time:   [3.1386 ns 3.1577 ns 3.1844 ns]

gauge(f64)::set/metrics time:   [6.4696 ns 6.5152 ns 6.5631 ns]
gauge(f64)::set/measured
                        time:   [1.0005 ns 1.0271 ns 1.0726 ns]
gauge(f64)::set/prometheus
                        time:   [590.97 ps 604.58 ps 629.78 ps]
gauge(f64)::set/prometheus_client
                        time:   [1.6941 ns 1.7136 ns 1.7357 ns]
gauge(f64)::set/fastmetrics
                        time:   [586.80 ps 609.50 ps 658.87 ps]

gauge(f64)::inc_by/metrics
                        time:   [6.8960 ns 6.9484 ns 7.0249 ns]
gauge(f64)::inc_by/measured
                        time:   [10.818 ns 10.918 ns 11.098 ns]
gauge(f64)::inc_by/prometheus
                        time:   [10.888 ns 10.981 ns 11.108 ns]
gauge(f64)::inc_by/prometheus_client
                        time:   [5.8338 ns 5.8601 ns 5.8900 ns]
gauge(f64)::inc_by/fastmetrics
                        time:   [5.7928 ns 5.8059 ns 5.8189 ns]

gauge(f64)::dec_by/metrics
                        time:   [6.9126 ns 6.9316 ns 6.9507 ns]
gauge(f64)::dec_by/measured
                        time:   [10.855 ns 10.890 ns 10.931 ns]
gauge(f64)::dec_by/prometheus
                        time:   [10.851 ns 10.880 ns 10.909 ns]
gauge(f64)::dec_by/prometheus_client
                        time:   [5.7687 ns 5.8194 ns 5.9032 ns]
gauge(f64)::dec_by/fastmetrics
                        time:   [5.8120 ns 5.8299 ns 5.8484 ns]

histogram::observe/metrics
                        time:   [10.552 ns 10.644 ns 10.751 ns]
histogram::observe/measured
                        time:   [11.623 ns 11.656 ns 11.691 ns]
histogram::observe/prometheus
                        time:   [10.928 ns 10.968 ns 11.013 ns]
histogram::observe/prometheus_client
                        time:   [8.9747 ns 9.0023 ns 9.0369 ns]
histogram::observe/fastmetrics
                        time:   [5.7546 ns 5.7678 ns 5.7813 ns]
```

## Metric Family

```bash
cargo bench --bench family -- --quiet
# Or `just bench family`
```

Each group of families includes a counter (u64) family and a histogram family.

```text
family with empty labels/metrics_cached
                        time:   [12.247 ns 12.294 ns 12.344 ns]
family with empty labels/metrics_dynamic
                        time:   [84.531 ns 84.784 ns 85.054 ns]
family with empty labels/measured
                        time:   [12.758 ns 12.797 ns 12.843 ns]
family with empty labels/prometheus
                        time:   [24.863 ns 24.942 ns 25.046 ns]
family with empty labels/prometheus_client
                        time:   [27.119 ns 27.181 ns 27.247 ns]
family with empty labels/fastmetrics_cached
                        time:   [5.8060 ns 5.8214 ns 5.8389 ns]
family with empty labels/fastmetrics_dynamic
                        time:   [17.144 ns 17.184 ns 17.227 ns]

family with custom labels/metrics_cached
                        time:   [12.474 ns 12.551 ns 12.640 ns]
family with custom labels/metrics_dynamic
                        time:   [158.01 ns 158.35 ns 158.72 ns]
family with custom labels/measured
                        time:   [15.481 ns 15.528 ns 15.581 ns]
family with custom labels/prometheus
                        time:   [25.950 ns 26.034 ns 26.133 ns]
family with custom labels/prometheus_client
                        time:   [39.168 ns 39.287 ns 39.416 ns]
family with custom labels/fastmetrics_cached
                        time:   [6.2012 ns 6.2307 ns 6.2638 ns]
family with custom labels/fastmetrics_dynamic
                        time:   [20.316 ns 20.406 ns 20.506 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [67.111 ns 67.294 ns 67.484 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [50.827 ns 50.924 ns 51.027 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [89.712 ns 89.958 ns 90.188 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [65.653 ns 65.816 ns 65.991 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [106.18 ns 106.57 ns 107.05 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [82.871 ns 83.151 ns 83.447 ns]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
# Or `just bench text`
```

Each group of metrics includes a counter (u64) and a histogram.

```text
text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [5.2281 ms 5.3138 ms 5.4316 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [410.74 µs 413.58 µs 416.77 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [532.96 µs 535.41 µs 537.98 µs]
text::encode/prometheus_client (openmetrics 1): 10 metrics * 100 times
                        time:   [336.83 µs 338.54 µs 340.24 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [162.63 µs 163.74 µs 164.95 µs]
text::encode/fastmetrics (openmetrics 1): 10 metrics * 100 times
                        time:   [163.04 µs 164.15 µs 165.18 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [10.485 ms 10.594 ms 10.693 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [440.27 µs 444.89 µs 453.71 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [584.82 µs 587.41 µs 590.73 µs]
text::encode/prometheus_client (openmetrics 1): 10 metrics * 1000 times
                        time:   [363.03 µs 364.92 µs 368.37 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [176.74 µs 177.73 µs 179.50 µs]
text::encode/fastmetrics (openmetrics 1): 10 metrics * 1000 times
                        time:   [176.05 µs 176.87 µs 177.77 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [10.928 ms 11.084 ms 11.253 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [448.42 µs 450.78 µs 454.85 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [595.20 µs 596.69 µs 598.30 µs]
text::encode/prometheus_client (openmetrics 1): 10 metrics * 10000 times
                        time:   [367.54 µs 368.21 µs 368.90 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [177.78 µs 179.02 µs 181.56 µs]
text::encode/fastmetrics (openmetrics 1): 10 metrics * 10000 times
                        time:   [177.97 µs 178.30 µs 178.65 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [11.267 ms 11.413 ms 11.554 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [452.88 µs 457.01 µs 464.36 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [618.22 µs 622.05 µs 628.64 µs]
text::encode/prometheus_client (openmetrics 1): 10 metrics * 100000 times
                        time:   [357.63 µs 361.21 µs 367.90 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [180.42 µs 181.87 µs 184.35 µs]
text::encode/fastmetrics (openmetrics 1): 10 metrics * 100000 times
                        time:   [179.44 µs 179.88 µs 180.38 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [53.436 ms 53.805 ms 54.174 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [4.1551 ms 4.1929 ms 4.2600 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [5.7268 ms 5.8118 ms 5.9317 ms]
text::encode/prometheus_client (openmetrics 1): 100 metrics * 100 times
                        time:   [3.4162 ms 3.4748 ms 3.5614 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [1.6750 ms 1.6795 ms 1.6845 ms]
text::encode/fastmetrics (openmetrics 1): 100 metrics * 100 times
                        time:   [1.6712 ms 1.6780 ms 1.6850 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [105.19 ms 105.94 ms 106.81 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [4.4668 ms 4.5146 ms 4.5990 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [6.2045 ms 6.2317 ms 6.2619 ms]
text::encode/prometheus_client (openmetrics 1): 100 metrics * 1000 times
                        time:   [3.7008 ms 3.7406 ms 3.8115 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [1.8035 ms 1.8086 ms 1.8140 ms]
text::encode/fastmetrics (openmetrics 1): 100 metrics * 1000 times
                        time:   [1.7914 ms 1.7976 ms 1.8041 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [110.35 ms 110.81 ms 111.29 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [4.5048 ms 4.5137 ms 4.5230 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [6.2554 ms 6.2713 ms 6.2878 ms]
text::encode/prometheus_client (openmetrics 1): 100 metrics * 10000 times
                        time:   [3.7042 ms 3.7109 ms 3.7180 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [1.8087 ms 1.8133 ms 1.8174 ms]
text::encode/fastmetrics (openmetrics 1): 100 metrics * 10000 times
                        time:   [1.8033 ms 1.8079 ms 1.8125 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [115.58 ms 117.14 ms 119.37 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [4.5890 ms 4.7871 ms 5.1389 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [6.5787 ms 6.5976 ms 6.6171 ms]
text::encode/prometheus_client (openmetrics 1): 100 metrics * 100000 times
                        time:   [3.6126 ms 3.6331 ms 3.6691 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [1.8395 ms 1.8438 ms 1.8478 ms]
text::encode/fastmetrics (openmetrics 1): 100 metrics * 100000 times
                        time:   [1.8409 ms 1.8570 ms 1.8825 ms]
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
