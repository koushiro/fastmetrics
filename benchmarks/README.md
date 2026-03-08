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
                        time:   [12.415 ns 12.549 ns 12.732 ns]
family with empty labels/metrics_dynamic
                        time:   [84.113 ns 84.422 ns 84.749 ns]
family with empty labels/measured
                        time:   [12.806 ns 13.097 ns 13.682 ns]
family with empty labels/prometheus
                        time:   [24.920 ns 25.016 ns 25.125 ns]
family with empty labels/prometheus_client
                        time:   [27.245 ns 27.323 ns 27.402 ns]
family with empty labels/fastmetrics_cached
                        time:   [5.8120 ns 5.8285 ns 5.8463 ns]
family with empty labels/fastmetrics_dynamic
                        time:   [17.332 ns 17.403 ns 17.484 ns]
family with empty labels/fastmetrics_indexed
                        time:   [5.8402 ns 5.8672 ns 5.8976 ns]

family with custom labels/metrics_cached
                        time:   [12.558 ns 12.698 ns 12.851 ns]
family with custom labels/metrics_dynamic
                        time:   [158.36 ns 159.18 ns 160.11 ns]
family with custom labels/measured
                        time:   [15.430 ns 15.484 ns 15.546 ns]
family with custom labels/prometheus
                        time:   [25.800 ns 25.865 ns 25.938 ns]
family with custom labels/prometheus_client
                        time:   [39.400 ns 39.603 ns 39.837 ns]
family with custom labels/fastmetrics_cached
                        time:   [6.1931 ns 6.2131 ns 6.2328 ns]
family with custom labels/fastmetrics_dynamic
                        time:   [20.184 ns 20.249 ns 20.323 ns]
family with custom labels/fastmetrics_indexed
                        time:   [9.3330 ns 9.5907 ns 10.101 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [67.808 ns 68.202 ns 68.624 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [50.815 ns 51.082 ns 51.453 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [88.605 ns 88.879 ns 89.144 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [66.904 ns 67.116 ns 67.341 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [106.45 ns 106.76 ns 107.07 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [83.354 ns 83.740 ns 84.160 ns]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
# Or `just bench text`
```

Each group of metrics includes a counter (u64) and a histogram.

```text
text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [5.2078 ms 5.2993 ms 5.3804 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [404.35 µs 406.34 µs 408.24 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [534.59 µs 537.27 µs 539.96 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [334.29 µs 335.83 µs 337.33 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [169.91 µs 171.24 µs 172.45 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 100 times
                        time:   [171.32 µs 172.50 µs 173.52 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [159.29 µs 160.39 µs 161.53 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 100 times
                        time:   [278.70 µs 281.64 µs 284.17 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [171.27 µs 172.55 µs 173.81 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 100 times
                        time:   [172.90 µs 174.34 µs 175.61 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [162.50 µs 163.48 µs 164.45 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 100 times
                        time:   [280.41 µs 283.50 µs 286.47 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [10.469 ms 10.613 ms 10.760 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [439.41 µs 439.90 µs 440.45 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [583.44 µs 585.41 µs 587.77 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [362.76 µs 363.85 µs 365.35 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [184.59 µs 184.94 µs 185.32 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 1000 times
                        time:   [185.08 µs 185.41 µs 185.71 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [173.53 µs 173.71 µs 173.89 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 1000 times
                        time:   [302.15 µs 302.58 µs 303.12 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [186.93 µs 187.46 µs 188.22 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 1000 times
                        time:   [187.88 µs 188.05 µs 188.22 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [175.62 µs 175.85 µs 176.08 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 1000 times
                        time:   [303.41 µs 303.90 µs 304.45 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [10.923 ms 11.107 ms 11.296 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [447.66 µs 448.11 µs 448.55 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [595.49 µs 596.17 µs 596.82 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [368.02 µs 368.46 µs 368.93 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [186.02 µs 186.28 µs 186.50 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 10000 times
                        time:   [186.26 µs 186.48 µs 186.74 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [173.97 µs 174.38 µs 174.87 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 10000 times
                        time:   [303.70 µs 304.69 µs 305.63 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [188.04 µs 188.38 µs 188.70 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 10000 times
                        time:   [188.75 µs 189.02 µs 189.30 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [176.46 µs 176.67 µs 176.90 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 10000 times
                        time:   [304.79 µs 305.15 µs 305.50 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [11.433 ms 11.640 ms 11.863 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [448.37 µs 448.92 µs 449.50 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [617.12 µs 618.03 µs 619.06 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [355.29 µs 355.77 µs 356.29 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [185.97 µs 186.53 µs 187.22 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 100000 times
                        time:   [186.49 µs 186.76 µs 187.04 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [174.54 µs 174.80 µs 175.07 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 100000 times
                        time:   [302.96 µs 303.39 µs 303.81 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [188.25 µs 188.55 µs 188.88 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 100000 times
                        time:   [188.85 µs 190.53 µs 193.31 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [176.98 µs 177.27 µs 177.64 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 100000 times
                        time:   [304.51 µs 304.76 µs 304.98 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [52.569 ms 52.984 ms 53.417 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [4.1172 ms 4.1250 ms 4.1330 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [5.6861 ms 5.7056 ms 5.7275 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [3.3903 ms 3.3960 ms 3.4018 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [1.7412 ms 1.7457 ms 1.7500 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 100 times
                        time:   [1.7485 ms 1.7517 ms 1.7554 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.6403 ms 1.6437 ms 1.6474 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 100 times
                        time:   [2.8654 ms 2.8802 ms 2.9019 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [1.7610 ms 1.7664 ms 1.7715 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 100 times
                        time:   [1.7704 ms 1.7751 ms 1.7791 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.6581 ms 1.6617 ms 1.6656 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 100 times
                        time:   [2.8823 ms 2.8897 ms 2.8959 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [105.66 ms 106.34 ms 107.18 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [4.4339 ms 4.4424 ms 4.4519 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [6.2082 ms 6.2276 ms 6.2513 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [3.6713 ms 3.6765 ms 3.6820 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [1.8830 ms 1.8860 ms 1.8890 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 1000 times
                        time:   [1.8896 ms 1.8947 ms 1.8999 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.7725 ms 1.7757 ms 1.7792 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 1000 times
                        time:   [3.0895 ms 3.0935 ms 3.0979 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [1.9177 ms 1.9233 ms 1.9313 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 1000 times
                        time:   [1.9227 ms 1.9249 ms 1.9271 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.7988 ms 1.8019 ms 1.8055 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 1000 times
                        time:   [3.1105 ms 3.1162 ms 3.1224 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [110.34 ms 111.05 ms 111.84 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [4.5264 ms 4.5391 ms 4.5556 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [6.4077 ms 6.4306 ms 6.4584 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [3.7333 ms 3.7411 ms 3.7505 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [1.9038 ms 1.9064 ms 1.9092 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 10000 times
                        time:   [1.9135 ms 1.9156 ms 1.9176 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.7959 ms 1.7996 ms 1.8035 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 10000 times
                        time:   [3.1429 ms 3.1838 ms 3.2605 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [1.9201 ms 1.9262 ms 1.9360 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 10000 times
                        time:   [1.9298 ms 1.9335 ms 1.9380 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.8108 ms 1.8153 ms 1.8210 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 10000 times
                        time:   [3.1244 ms 3.1294 ms 3.1342 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [116.80 ms 117.70 ms 118.63 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [4.5468 ms 4.5593 ms 4.5770 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [6.5607 ms 6.5752 ms 6.5903 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [3.5867 ms 3.5924 ms 3.5996 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [1.8967 ms 1.9023 ms 1.9122 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 100000 times
                        time:   [1.9007 ms 1.9034 ms 1.9060 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.7867 ms 1.7896 ms 1.7927 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 100000 times
                        time:   [3.1084 ms 3.1150 ms 3.1213 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [2.0825 ms 2.2987 ms 2.5804 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 100000 times
                        time:   [1.9935 ms 2.0292 ms 2.0789 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.8512 ms 1.8608 ms 1.8707 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 100000 times
                        time:   [3.1734 ms 3.2836 ms 3.5048 ms]
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
