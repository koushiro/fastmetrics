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
                        time:   [5.2419 ms 5.3558 ms 5.4493 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [411.21 µs 416.61 µs 423.28 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [529.57 µs 534.41 µs 541.45 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [338.94 µs 341.13 µs 343.33 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [170.75 µs 171.99 µs 173.25 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [160.40 µs 161.40 µs 162.45 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 10 metrics * 100 times
                        time:   [279.84 µs 282.03 µs 284.17 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [170.83 µs 172.30 µs 173.83 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [160.66 µs 163.28 µs 166.65 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 10 metrics * 100 times
                        time:   [275.80 µs 277.91 µs 280.48 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [10.387 ms 10.597 ms 10.787 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [440.71 µs 441.42 µs 442.23 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [579.37 µs 580.71 µs 582.37 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [364.92 µs 365.44 µs 365.99 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [184.61 µs 185.91 µs 188.00 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [174.34 µs 176.76 µs 179.64 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 10 metrics * 1000 times
                        time:   [300.22 µs 301.28 µs 302.66 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [185.38 µs 185.92 µs 186.53 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [172.92 µs 173.58 µs 174.38 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 10 metrics * 1000 times
                        time:   [300.74 µs 301.93 µs 303.13 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [10.867 ms 11.109 ms 11.343 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [454.06 µs 460.44 µs 469.83 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [590.20 µs 591.66 µs 593.26 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [371.05 µs 371.92 µs 372.84 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [185.77 µs 186.13 µs 186.51 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [175.39 µs 176.93 µs 179.34 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 10 metrics * 10000 times
                        time:   [301.96 µs 302.98 µs 304.41 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [185.90 µs 186.54 µs 187.21 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [174.22 µs 174.65 µs 175.11 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 10 metrics * 10000 times
                        time:   [301.73 µs 303.03 µs 304.76 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [11.479 ms 11.633 ms 11.799 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [450.95 µs 451.94 µs 452.98 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [613.24 µs 614.64 µs 616.12 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [359.61 µs 360.29 µs 361.04 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [186.50 µs 186.99 µs 187.56 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [174.64 µs 174.95 µs 175.26 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 10 metrics * 100000 times
                        time:   [301.30 µs 301.87 µs 302.46 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [186.59 µs 187.30 µs 188.21 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [174.46 µs 174.96 µs 175.56 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 10 metrics * 100000 times
                        time:   [301.43 µs 302.60 µs 304.03 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [53.482 ms 53.796 ms 54.095 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [4.1480 ms 4.1627 ms 4.1804 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [5.6357 ms 5.6597 ms 5.6849 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [3.4345 ms 3.4439 ms 3.4545 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [1.7494 ms 1.7550 ms 1.7605 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.6441 ms 1.6501 ms 1.6563 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 100 metrics * 100 times
                        time:   [2.8735 ms 2.8934 ms 2.9140 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [1.7548 ms 1.7647 ms 1.7770 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.6443 ms 1.6536 ms 1.6659 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 100 metrics * 100 times
                        time:   [2.8482 ms 2.8593 ms 2.8725 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [106.36 ms 107.18 ms 108.12 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [4.4779 ms 4.4904 ms 4.5044 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [6.2099 ms 6.2336 ms 6.2584 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [3.7248 ms 3.7611 ms 3.8257 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [1.8877 ms 1.8937 ms 1.9020 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.7666 ms 1.7748 ms 1.7841 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 100 metrics * 1000 times
                        time:   [3.0646 ms 3.0749 ms 3.0870 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [1.8956 ms 1.9047 ms 1.9151 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.7661 ms 1.7729 ms 1.7805 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 100 metrics * 1000 times
                        time:   [3.0784 ms 3.1352 ms 3.2281 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [110.62 ms 111.23 ms 111.84 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [4.5166 ms 4.5324 ms 4.5538 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [6.3067 ms 6.3345 ms 6.3686 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [3.7465 ms 3.7541 ms 3.7623 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [1.8862 ms 1.8902 ms 1.8951 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.7845 ms 1.7923 ms 1.8018 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 100 metrics * 10000 times
                        time:   [3.0689 ms 3.0788 ms 3.0882 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [1.8977 ms 1.9054 ms 1.9147 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.7809 ms 1.7845 ms 1.7887 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 100 metrics * 10000 times
                        time:   [3.0698 ms 3.0857 ms 3.1165 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [117.32 ms 119.22 ms 121.68 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [4.5694 ms 4.7392 ms 5.0348 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [6.6965 ms 6.7864 ms 6.9053 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [3.6565 ms 3.6966 ms 3.7658 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [1.9184 ms 1.9339 ms 1.9575 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.7997 ms 1.8047 ms 1.8096 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 => underscores): 100 metrics * 100000 times
                        time:   [3.0976 ms 3.1249 ms 3.1673 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [1.9100 ms 1.9167 ms 1.9253 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.7973 ms 1.8022 ms 1.8069 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 => underscores): 100 metrics * 100000 times
                        time:   [3.0875 ms 3.1004 ms 3.1151 ms]
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
