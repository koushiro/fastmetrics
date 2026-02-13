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
                        time:   [5.2107 ms 5.2848 ms 5.3495 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [410.42 µs 419.13 µs 432.04 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [531.88 µs 534.44 µs 537.25 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [340.17 µs 343.40 µs 347.01 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [169.88 µs 171.12 µs 172.42 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [159.13 µs 160.59 µs 162.07 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 10 metrics * 100 times
                        time:   [277.06 µs 279.66 µs 281.96 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [170.31 µs 171.66 µs 172.95 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [159.88 µs 165.65 µs 177.20 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 10 metrics * 100 times
                        time:   [280.15 µs 282.72 µs 285.02 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [10.399 ms 10.545 ms 10.698 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [443.30 µs 444.10 µs 444.90 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [586.38 µs 588.27 µs 590.36 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [365.90 µs 366.62 µs 367.36 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [185.11 µs 185.48 µs 185.87 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [173.64 µs 173.93 µs 174.24 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 10 metrics * 1000 times
                        time:   [300.83 µs 301.40 µs 301.98 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [185.26 µs 188.11 µs 193.66 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [173.31 µs 173.69 µs 174.10 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 10 metrics * 1000 times
                        time:   [301.99 µs 302.68 µs 303.38 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [10.812 ms 11.484 ms 12.465 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [450.75 µs 451.56 µs 452.44 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [596.24 µs 597.42 µs 598.73 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [372.95 µs 386.86 µs 412.30 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [187.25 µs 187.65 µs 188.09 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [175.27 µs 175.62 µs 175.98 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 10 metrics * 10000 times
                        time:   [303.11 µs 303.67 µs 304.22 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [186.49 µs 186.83 µs 187.19 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [174.93 µs 180.02 µs 191.43 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 10 metrics * 10000 times
                        time:   [302.64 µs 303.36 µs 304.03 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [11.339 ms 11.482 ms 11.605 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [453.53 µs 454.39 µs 455.31 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [617.31 µs 618.72 µs 620.34 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [359.69 µs 361.50 µs 363.96 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [187.34 µs 187.68 µs 188.03 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [176.20 µs 176.49 µs 176.78 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 10 metrics * 100000 times
                        time:   [303.30 µs 303.66 µs 304.06 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [187.83 µs 188.35 µs 188.95 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [176.09 µs 176.45 µs 176.79 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 10 metrics * 100000 times
                        time:   [304.22 µs 305.03 µs 305.90 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [53.807 ms 54.066 ms 54.326 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [4.1668 ms 4.1818 ms 4.1982 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [5.8428 ms 6.0025 ms 6.2576 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [3.4333 ms 3.4440 ms 3.4554 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [1.8005 ms 1.8104 ms 1.8203 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.6896 ms 1.6977 ms 1.7067 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 100 metrics * 100 times
                        time:   [2.9293 ms 2.9414 ms 2.9546 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [1.7749 ms 1.7841 ms 1.7950 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.6770 ms 1.6853 ms 1.6946 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 100 metrics * 100 times
                        time:   [2.8898 ms 2.8972 ms 2.9065 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [107.41 ms 108.12 ms 108.89 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [4.4948 ms 4.5075 ms 4.5219 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [6.4141 ms 6.4600 ms 6.5110 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [3.7319 ms 3.8255 ms 3.9881 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [1.9115 ms 1.9171 ms 1.9228 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.7938 ms 1.7986 ms 1.8036 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 100 metrics * 1000 times
                        time:   [3.1013 ms 3.1142 ms 3.1281 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [1.9047 ms 1.9103 ms 1.9158 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.7943 ms 1.7994 ms 1.8047 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 100 metrics * 1000 times
                        time:   [3.1024 ms 3.1092 ms 3.1158 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [111.19 ms 112.01 ms 112.86 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [4.5447 ms 4.5526 ms 4.5607 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [6.4666 ms 6.6367 ms 6.9339 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [3.8359 ms 3.8521 ms 3.8693 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [1.9523 ms 1.9844 ms 2.0351 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.8281 ms 1.8360 ms 1.8440 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 100 metrics * 10000 times
                        time:   [3.1439 ms 3.1674 ms 3.1982 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [1.9595 ms 2.0620 ms 2.2631 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.8161 ms 1.8237 ms 1.8323 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 100 metrics * 10000 times
                        time:   [3.1776 ms 3.2566 ms 3.3497 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [118.47 ms 119.46 ms 120.50 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [4.6520 ms 4.7137 ms 4.7895 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [6.7197 ms 6.8553 ms 7.0196 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [3.6384 ms 3.6459 ms 3.6541 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [1.9263 ms 1.9314 ms 1.9375 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.8056 ms 1.8116 ms 1.8182 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 100 metrics * 100000 times
                        time:   [3.0984 ms 3.1087 ms 3.1209 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [1.9211 ms 1.9327 ms 1.9553 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.8035 ms 1.8104 ms 1.8176 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 100 metrics * 100000 times
                        time:   [3.1020 ms 3.1110 ms 3.1205 ms]
```

## Protobuf Encoding

- metrics-exporter-prometheus: use the [prost] crate for (prometheus) protobuf encoding
- prometheus: use the [protobuf] crate for (prometheus) protobuf encoding
- prometheus-client: use the [prost] crate for (openmetrics) protobuf encoding
- fastmetrics: use [prost] or [protobuf] crate for both (prometheus) and (openmetrics) protobuf encoding

[prost]: https://crates.io/crates/prost
[protobuf]: https://crates.io/crates/protobuf

```bash
cargo bench --bench protobuf  -- --quiet
# Or `just bench protobuf`
```

Each group of metrics includes a counter (u64) and a histogram.

```text
protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 100 times
                        time:   [4.6235 ms 4.7349 ms 4.8421 ms]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 100 times
                        time:   [175.74 µs 180.18 µs 186.11 µs]
protobuf::encode/prometheus_client (prost/openmetrics): 10 metrics * 100 times
                        time:   [234.90 µs 240.97 µs 249.13 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 100 times
                        time:   [222.26 µs 225.36 µs 229.08 µs]
protobuf::encode/fastmetrics (prost/openmetrics): 10 metrics * 100 times
                        time:   [240.63 µs 242.53 µs 244.38 µs]
protobuf::encode/fastmetrics (protobuf/promtheus): 10 metrics * 100 times
                        time:   [223.52 µs 225.11 µs 226.80 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100 times
                        time:   [224.16 µs 225.84 µs 227.51 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 1000 times
                        time:   [9.6647 ms 9.8419 ms 10.105 ms]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 1000 times
                        time:   [185.59 µs 186.87 µs 188.23 µs]
protobuf::encode/prometheus_client (prost/openmetrics): 10 metrics * 1000 times
                        time:   [247.93 µs 254.83 µs 266.34 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 1000 times
                        time:   [234.74 µs 241.66 µs 251.75 µs]
protobuf::encode/fastmetrics (prost/openmetrics): 10 metrics * 1000 times
                        time:   [257.85 µs 259.76 µs 261.71 µs]
protobuf::encode/fastmetrics (protobuf/promtheus): 10 metrics * 1000 times
                        time:   [235.76 µs 237.62 µs 240.08 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 1000 times
                        time:   [242.68 µs 244.17 µs 245.76 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 10000 times
                        time:   [10.047 ms 10.298 ms 10.552 ms]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 10000 times
                        time:   [182.65 µs 183.58 µs 184.53 µs]
protobuf::encode/prometheus_client (prost/openmetrics): 10 metrics * 10000 times
                        time:   [250.34 µs 252.02 µs 253.79 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 10000 times
                        time:   [232.11 µs 233.78 µs 235.62 µs]
protobuf::encode/fastmetrics (prost/openmetrics): 10 metrics * 10000 times
                        time:   [262.30 µs 268.87 µs 278.57 µs]
protobuf::encode/fastmetrics (protobuf/promtheus): 10 metrics * 10000 times
                        time:   [237.52 µs 238.94 µs 240.46 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 10000 times
                        time:   [246.57 µs 247.85 µs 249.35 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 100000 times
                        time:   [10.420 ms 10.549 ms 10.710 ms]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 100000 times
                        time:   [181.85 µs 182.91 µs 184.14 µs]
protobuf::encode/prometheus_client (prost/openmetrics): 10 metrics * 100000 times
                        time:   [252.93 µs 259.58 µs 272.46 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 100000 times
                        time:   [231.65 µs 232.61 µs 233.63 µs]
protobuf::encode/fastmetrics (prost/openmetrics): 10 metrics * 100000 times
                        time:   [260.83 µs 261.42 µs 262.08 µs]
protobuf::encode/fastmetrics (protobuf/promtheus): 10 metrics * 100000 times
                        time:   [235.09 µs 237.56 µs 241.70 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100000 times
                        time:   [248.19 µs 248.96 µs 249.82 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 100 times
                        time:   [45.869 ms 46.226 ms 46.732 ms]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 100 times
                        time:   [1.9234 ms 1.9666 ms 2.0298 ms]
protobuf::encode/prometheus_client (prost/openmetrics): 100 metrics * 100 times
                        time:   [2.5320 ms 2.5617 ms 2.5951 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 100 times
                        time:   [2.3612 ms 2.3714 ms 2.3818 ms]
protobuf::encode/fastmetrics (prost/openmetrics): 100 metrics * 100 times
                        time:   [2.5417 ms 2.6119 ms 2.7109 ms]
protobuf::encode/fastmetrics (protobuf/promtheus): 100 metrics * 100 times
                        time:   [2.3428 ms 2.3572 ms 2.3745 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100 times
                        time:   [2.3570 ms 2.3658 ms 2.3756 ms]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 1000 times
                        time:   [95.946 ms 96.777 ms 97.607 ms]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 1000 times
                        time:   [1.9511 ms 1.9753 ms 2.0018 ms]
protobuf::encode/prometheus_client (prost/openmetrics): 100 metrics * 1000 times
                        time:   [2.5819 ms 2.6049 ms 2.6293 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 1000 times
                        time:   [2.4610 ms 2.4877 ms 2.5149 ms]
protobuf::encode/fastmetrics (prost/openmetrics): 100 metrics * 1000 times
                        time:   [2.6556 ms 2.6789 ms 2.7040 ms]
protobuf::encode/fastmetrics (protobuf/promtheus): 100 metrics * 1000 times
                        time:   [2.4373 ms 2.4573 ms 2.4788 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 1000 times
                        time:   [2.4683 ms 2.4815 ms 2.4953 ms]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 10000 times
                        time:   [103.00 ms 103.67 ms 104.44 ms]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 10000 times
                        time:   [2.0913 ms 2.1022 ms 2.1138 ms]
protobuf::encode/prometheus_client (prost/openmetrics): 100 metrics * 10000 times
                        time:   [2.6739 ms 2.6843 ms 2.6967 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 10000 times
                        time:   [2.5919 ms 2.6588 ms 2.7788 ms]
protobuf::encode/fastmetrics (prost/openmetrics): 100 metrics * 10000 times
                        time:   [2.7735 ms 2.7853 ms 2.7980 ms]
protobuf::encode/fastmetrics (protobuf/promtheus): 100 metrics * 10000 times
                        time:   [2.5129 ms 2.5234 ms 2.5345 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 10000 times
                        time:   [2.6079 ms 2.6210 ms 2.6354 ms]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 100000 times
                        time:   [108.24 ms 109.38 ms 110.65 ms]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 100000 times
                        time:   [2.1033 ms 2.1224 ms 2.1463 ms]
protobuf::encode/prometheus_client (prost/openmetrics): 100 metrics * 100000 times
                        time:   [2.7185 ms 2.7340 ms 2.7545 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 100000 times
                        time:   [2.5878 ms 2.6103 ms 2.6450 ms]
protobuf::encode/fastmetrics (prost/openmetrics): 100 metrics * 100000 times
                        time:   [2.8419 ms 2.8522 ms 2.8631 ms]
protobuf::encode/fastmetrics (protobuf/promtheus): 100 metrics * 100000 times
                        time:   [2.5172 ms 2.5315 ms 2.5509 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100000 times
                        time:   [2.6373 ms 2.6481 ms 2.6593 ms]
```
