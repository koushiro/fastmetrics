# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.99.1 (58cf8f9c3 2026-10-13)

## Metric

```bash
cargo bench --bench metric -- --quiet
# Or `just bench metric`
```

```text
counter(u64)::inc/metrics
                        time:   [7.1040 ns 7.1405 ns 7.1822 ns]
counter(u64)::inc/measured
                        time:   [2.1493 ns 2.1641 ns 2.1921 ns]
counter(u64)::inc/prometheus
                        time:   [2.1816 ns 2.1845 ns 2.1876 ns]
counter(u64)::inc/prometheus_client
                        time:   [2.1174 ns 2.1255 ns 2.1369 ns]
counter(u64)::inc/fastmetrics
                        time:   [2.1871 ns 2.2158 ns 2.2675 ns]

counter(u64)::saturating_inc/fastmetrics
                        time:   [2.4821 ns 2.4860 ns 2.4902 ns]

counter(f64)::inc/metrics
                        time:   [7.1153 ns 7.1247 ns 7.1351 ns]
counter(f64)::inc/prometheus
                        time:   [10.660 ns 10.675 ns 10.691 ns]
counter(f64)::inc/prometheus_client
                        time:   [5.5899 ns 5.6449 ns 5.7427 ns]
counter(f64)::inc/fastmetrics
                        time:   [5.6395 ns 5.6614 ns 5.6916 ns]

gauge(i64)::set/metrics time:   [7.1102 ns 7.1248 ns 7.1402 ns]
gauge(i64)::set/measured
                        time:   [975.85 ps 982.95 ps 993.20 ps]
gauge(i64)::set/prometheus
                        time:   [635.28 ps 637.13 ps 639.16 ps]
gauge(i64)::set/prometheus_client
                        time:   [1.8361 ns 1.8548 ns 1.8789 ns]
gauge(i64)::set/fastmetrics
                        time:   [689.74 ps 693.03 ps 696.83 ps]

gauge(i64)::inc_by/metrics
                        time:   [6.8251 ns 6.8397 ns 6.8576 ns]
gauge(i64)::inc_by/measured
                        time:   [2.1433 ns 2.1503 ns 2.1580 ns]
gauge(i64)::inc_by/prometheus
                        time:   [2.1291 ns 2.1458 ns 2.1751 ns]
gauge(i64)::inc_by/prometheus_client
                        time:   [2.1963 ns 2.2057 ns 2.2190 ns]
gauge(i64)::inc_by/fastmetrics
                        time:   [2.1380 ns 2.1525 ns 2.1751 ns]

gauge(i64)::saturating_inc_by/fastmetrics
                        time:   [3.1347 ns 3.2467 ns 3.4190 ns]

gauge(i64)::dec_by/metrics
                        time:   [6.8128 ns 6.8701 ns 6.9485 ns]
gauge(i64)::dec_by/measured
                        time:   [2.1509 ns 2.2112 ns 2.3383 ns]
gauge(i64)::dec_by/prometheus
                        time:   [2.1493 ns 2.1613 ns 2.1773 ns]
gauge(i64)::dec_by/prometheus_client
                        time:   [2.2049 ns 2.2278 ns 2.2665 ns]
gauge(i64)::dec_by/fastmetrics
                        time:   [2.1309 ns 2.1376 ns 2.1454 ns]

gauge(i64)::saturating_dec_by/fastmetrics
                        time:   [3.0990 ns 3.1067 ns 3.1159 ns]

gauge(f64)::set/metrics time:   [7.1187 ns 7.1390 ns 7.1623 ns]
gauge(f64)::set/measured
                        time:   [978.17 ps 982.94 ps 988.35 ps]
gauge(f64)::set/prometheus
                        time:   [708.02 ps 717.64 ps 730.92 ps]
gauge(f64)::set/prometheus_client
                        time:   [1.8107 ns 1.8289 ns 1.8527 ns]
gauge(f64)::set/fastmetrics
                        time:   [723.56 ps 728.30 ps 733.55 ps]

gauge(f64)::inc_by/metrics
                        time:   [6.8028 ns 6.8301 ns 6.8753 ns]
gauge(f64)::inc_by/measured
                        time:   [10.693 ns 10.710 ns 10.728 ns]
gauge(f64)::inc_by/prometheus
                        time:   [10.666 ns 10.687 ns 10.711 ns]
gauge(f64)::inc_by/prometheus_client
                        time:   [5.6866 ns 5.7017 ns 5.7189 ns]
gauge(f64)::inc_by/fastmetrics
                        time:   [5.6642 ns 5.6906 ns 5.7242 ns]

gauge(f64)::dec_by/metrics
                        time:   [6.8077 ns 6.9138 ns 7.1253 ns]
gauge(f64)::dec_by/measured
                        time:   [10.695 ns 10.714 ns 10.733 ns]
gauge(f64)::dec_by/prometheus
                        time:   [10.670 ns 10.698 ns 10.733 ns]
gauge(f64)::dec_by/prometheus_client
                        time:   [5.7155 ns 5.7661 ns 5.8331 ns]
gauge(f64)::dec_by/fastmetrics
                        time:   [5.6580 ns 5.6960 ns 5.7521 ns]

histogram::observe/metrics
                        time:   [10.178 ns 10.231 ns 10.289 ns]
histogram::observe/measured
                        time:   [11.661 ns 11.695 ns 11.735 ns]
histogram::observe/prometheus
                        time:   [10.950 ns 10.985 ns 11.034 ns]
histogram::observe/prometheus_client
                        time:   [8.8740 ns 8.9017 ns 8.9316 ns]
histogram::observe/fastmetrics
                        time:   [5.7231 ns 5.7598 ns 5.8130 ns]
```

## Metric Family

```bash
cargo bench --bench family -- --quiet
# Or `just bench family`
```

Each group of families includes a counter (u64) family and a histogram family.

```text
family with empty labels/metrics_cached
                        time:   [12.481 ns 12.580 ns 12.702 ns]
family with empty labels/metrics_dynamic
                        time:   [51.250 ns 51.434 ns 51.640 ns]
family with empty labels/measured
                        time:   [11.629 ns 11.708 ns 11.825 ns]
family with empty labels/prometheus
                        time:   [23.857 ns 23.931 ns 24.015 ns]
family with empty labels/prometheus_client
                        time:   [27.535 ns 27.594 ns 27.664 ns]
family with empty labels/fastmetrics_cached
                        time:   [5.7443 ns 5.7763 ns 5.8256 ns]
family with empty labels/fastmetrics_dynamic
                        time:   [17.179 ns 17.246 ns 17.335 ns]

family with custom labels/metrics_cached
                        time:   [12.576 ns 12.695 ns 12.846 ns]
family with custom labels/metrics_dynamic
                        time:   [115.20 ns 115.86 ns 116.66 ns]
family with custom labels/measured
                        time:   [11.087 ns 11.126 ns 11.166 ns]
family with custom labels/prometheus
                        time:   [24.763 ns 24.812 ns 24.866 ns]
family with custom labels/prometheus_client
                        time:   [37.568 ns 37.926 ns 38.504 ns]
family with custom labels/fastmetrics_cached
                        time:   [6.1070 ns 6.2002 ns 6.3578 ns]
family with custom labels/fastmetrics_dynamic
                        time:   [19.263 ns 19.295 ns 19.329 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [67.823 ns 68.039 ns 68.279 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [49.826 ns 49.950 ns 50.086 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [86.223 ns 87.096 ns 88.156 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [61.603 ns 61.708 ns 61.820 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [102.73 ns 104.09 ns 106.46 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [83.755 ns 83.927 ns 84.119 ns]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
# Or `just bench text`
```

Each group of metrics includes a counter (u64) and a histogram.

```text
text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [4.4268 ms 4.5278 ms 4.6642 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [354.22 µs 356.26 µs 358.55 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [569.28 µs 571.42 µs 573.51 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [411.15 µs 413.48 µs 415.66 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [179.45 µs 180.91 µs 183.16 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 100 times
                        time:   [178.81 µs 180.65 µs 182.99 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [171.48 µs 172.59 µs 173.85 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 100 times
                        time:   [316.79 µs 321.81 µs 328.37 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [181.23 µs 182.37 µs 183.44 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 100 times
                        time:   [181.96 µs 182.85 µs 183.88 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [174.28 µs 176.81 µs 181.24 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 100 times
                        time:   [314.63 µs 318.39 µs 322.58 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [8.7182 ms 8.8241 ms 8.9441 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [382.10 µs 384.80 µs 389.36 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [623.40 µs 626.02 µs 629.10 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [444.99 µs 446.09 µs 447.32 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [192.79 µs 194.26 µs 197.10 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 1000 times
                        time:   [193.44 µs 194.09 µs 194.86 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [184.82 µs 185.23 µs 185.70 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 1000 times
                        time:   [339.53 µs 343.31 µs 350.27 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [195.38 µs 195.82 µs 196.29 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 1000 times
                        time:   [197.04 µs 197.69 µs 198.45 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [187.47 µs 187.81 µs 188.21 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 1000 times
                        time:   [340.97 µs 341.54 µs 342.12 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [9.0896 ms 9.3057 ms 9.5989 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [385.01 µs 386.55 µs 388.25 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [634.08 µs 638.79 µs 647.42 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [451.29 µs 452.17 µs 453.18 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [194.64 µs 195.09 µs 195.58 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 10000 times
                        time:   [195.74 µs 196.22 µs 196.72 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [190.23 µs 190.59 µs 190.94 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 10000 times
                        time:   [346.07 µs 347.68 µs 350.09 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [198.02 µs 198.58 µs 199.28 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 10000 times
                        time:   [198.74 µs 201.48 µs 206.54 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [189.01 µs 189.46 µs 189.93 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 10000 times
                        time:   [341.71 µs 342.55 µs 343.73 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [9.5702 ms 9.7405 ms 9.9031 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [390.28 µs 391.31 µs 392.36 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [655.89 µs 657.25 µs 658.68 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [454.33 µs 456.77 µs 459.77 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [194.70 µs 195.05 µs 195.40 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 100000 times
                        time:   [195.33 µs 195.57 µs 195.87 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [187.43 µs 187.94 µs 188.61 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 100000 times
                        time:   [341.47 µs 342.65 µs 343.86 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [197.63 µs 200.28 µs 204.93 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 100000 times
                        time:   [197.78 µs 199.26 µs 201.74 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [188.59 µs 188.88 µs 189.20 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 100000 times
                        time:   [341.95 µs 342.56 µs 343.25 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [46.562 ms 46.804 ms 47.041 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [3.6253 ms 3.6372 ms 3.6494 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [6.0263 ms 6.0932 ms 6.1831 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [4.1835 ms 4.1999 ms 4.2251 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [1.8257 ms 1.8312 ms 1.8370 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 100 times
                        time:   [1.8341 ms 1.8461 ms 1.8586 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.7494 ms 1.7551 ms 1.7608 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 100 times
                        time:   [3.2274 ms 3.2768 ms 3.3520 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [1.8524 ms 1.8679 ms 1.8948 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 100 times
                        time:   [1.8584 ms 1.8695 ms 1.8828 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.7791 ms 1.7864 ms 1.7939 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 100 times
                        time:   [3.2511 ms 3.2631 ms 3.2748 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [89.361 ms 90.512 ms 92.448 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [3.8921 ms 3.9299 ms 3.9898 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [6.5682 ms 6.5906 ms 6.6137 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [4.5267 ms 4.5357 ms 4.5456 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [1.9731 ms 1.9975 ms 2.0413 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 1000 times
                        time:   [1.9752 ms 1.9814 ms 1.9893 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.8995 ms 1.9197 ms 1.9455 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 1000 times
                        time:   [3.4722 ms 3.4891 ms 3.5113 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [1.9904 ms 1.9966 ms 2.0034 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 1000 times
                        time:   [2.0076 ms 2.0154 ms 2.0240 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.9215 ms 1.9335 ms 1.9527 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 1000 times
                        time:   [3.4715 ms 3.4842 ms 3.4995 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [93.727 ms 94.708 ms 96.112 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [3.9415 ms 3.9576 ms 3.9750 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [6.6874 ms 6.7643 ms 6.8901 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [4.6187 ms 4.6353 ms 4.6542 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [1.9879 ms 2.0090 ms 2.0470 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 10000 times
                        time:   [1.9940 ms 2.0024 ms 2.0128 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.9240 ms 1.9448 ms 1.9743 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 10000 times
                        time:   [3.4701 ms 3.4875 ms 3.5051 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [2.0163 ms 2.0310 ms 2.0544 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 10000 times
                        time:   [2.0300 ms 2.0376 ms 2.0460 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.9492 ms 1.9792 ms 2.0403 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 10000 times
                        time:   [3.4961 ms 3.5271 ms 3.5770 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [98.934 ms 99.781 ms 100.69 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [3.9924 ms 4.0066 ms 4.0207 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [6.9634 ms 7.1431 ms 7.4431 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [4.6385 ms 4.6724 ms 4.7299 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [2.0075 ms 2.0190 ms 2.0338 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 100000 times
                        time:   [1.9943 ms 2.0130 ms 2.0449 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.9042 ms 1.9215 ms 1.9466 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 100000 times
                        time:   [3.4536 ms 3.4640 ms 3.4741 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [2.0080 ms 2.0179 ms 2.0292 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 100000 times
                        time:   [2.0817 ms 2.1600 ms 2.2395 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.9328 ms 1.9418 ms 1.9536 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 100000 times
                        time:   [3.4602 ms 3.4865 ms 3.5280 ms]
```

## Protobuf Encoding

- metrics-exporter-prometheus: use the [prost] crate for (prometheus) protobuf encoding
- measured: use [measured-prometheus-protobuf] for (prometheus) protobuf encoding
- prometheus: use the [protobuf] crate for (prometheus) protobuf encoding
- prometheus-client: use the [prost] crate for (prometheus) protobuf encoding
- fastmetrics: use the [prost] crate for (prometheus) protobuf encoding

[prost]: https://crates.io/crates/prost
[protobuf]: https://crates.io/crates/protobuf
[measured-prometheus-protobuf]: https://crates.io/crates/measured-prometheus-protobuf

```bash
cargo bench --bench protobuf  -- --quiet
# Or `just bench protobuf`
```

Each group of metrics includes a counter (u64) and a histogram.

```text
protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 100 times
                        time:   [3.8789 ms 4.2672 ms 4.6656 ms]
protobuf::encode/measured (self/prometheus): 10 metrics * 100 times
                        time:   [47.757 µs 48.221 µs 48.836 µs]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 100 times
                        time:   [180.58 µs 182.13 µs 183.88 µs]
protobuf::encode/prometheus_client (prost/prometheus): 10 metrics * 100 times
                        time:   [251.50 µs 254.45 µs 258.61 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 100 times
                        time:   [66.610 µs 68.485 µs 71.392 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 1000 times
                        time:   [8.9071 ms 9.6011 ms 11.266 ms]
protobuf::encode/measured (self/prometheus): 10 metrics * 1000 times
                        time:   [50.795 µs 51.022 µs 51.357 µs]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 1000 times
                        time:   [194.11 µs 195.20 µs 196.36 µs]
protobuf::encode/prometheus_client (prost/prometheus): 10 metrics * 1000 times
                        time:   [269.73 µs 270.59 µs 271.47 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 1000 times
                        time:   [71.118 µs 71.418 µs 71.741 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 10000 times
                        time:   [9.5258 ms 9.6746 ms 9.8450 ms]
protobuf::encode/measured (self/prometheus): 10 metrics * 10000 times
                        time:   [51.009 µs 51.134 µs 51.279 µs]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 10000 times
                        time:   [194.87 µs 196.10 µs 197.51 µs]
protobuf::encode/prometheus_client (prost/prometheus): 10 metrics * 10000 times
                        time:   [273.32 µs 274.31 µs 275.50 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 10000 times
                        time:   [71.472 µs 75.529 µs 80.747 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 100000 times
                        time:   [9.5757 ms 9.9116 ms 10.221 ms]
protobuf::encode/measured (self/prometheus): 10 metrics * 100000 times
                        time:   [51.367 µs 51.761 µs 52.221 µs]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 100000 times
                        time:   [199.54 µs 205.92 µs 214.57 µs]
protobuf::encode/prometheus_client (prost/prometheus): 10 metrics * 100000 times
                        time:   [276.59 µs 277.76 µs 279.07 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 100000 times
                        time:   [73.293 µs 74.666 µs 76.339 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 100 times
                        time:   [42.819 ms 43.355 ms 43.824 ms]
protobuf::encode/measured (self/prometheus): 100 metrics * 100 times
                        time:   [484.72 µs 487.51 µs 490.46 µs]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 100 times
                        time:   [1.9580 ms 1.9821 ms 2.0150 ms]
protobuf::encode/prometheus_client (prost/prometheus): 100 metrics * 100 times
                        time:   [2.7693 ms 2.7891 ms 2.8123 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 100 times
                        time:   [641.69 µs 644.80 µs 648.35 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 1000 times
                        time:   [92.273 ms 106.19 ms 126.09 ms]
protobuf::encode/measured (self/prometheus): 100 metrics * 1000 times
                        time:   [516.71 µs 519.11 µs 522.09 µs]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 1000 times
                        time:   [2.1547 ms 2.1948 ms 2.2457 ms]
protobuf::encode/prometheus_client (prost/prometheus): 100 metrics * 1000 times
                        time:   [2.9070 ms 2.9255 ms 2.9454 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 1000 times
                        time:   [686.45 µs 688.81 µs 691.24 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 10000 times
                        time:   [97.430 ms 101.28 ms 107.80 ms]
protobuf::encode/measured (self/prometheus): 100 metrics * 10000 times
                        time:   [513.72 µs 516.06 µs 519.13 µs]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 10000 times
                        time:   [2.0334 ms 2.0630 ms 2.1063 ms]
protobuf::encode/prometheus_client (prost/prometheus): 100 metrics * 10000 times
                        time:   [3.0177 ms 3.1230 ms 3.2813 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 10000 times
                        time:   [715.42 µs 718.57 µs 721.90 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 100000 times
                        time:   [104.60 ms 107.39 ms 110.87 ms]
protobuf::encode/measured (self/prometheus): 100 metrics * 100000 times
                        time:   [514.76 µs 516.61 µs 519.12 µs]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 100000 times
                        time:   [2.1381 ms 2.2218 ms 2.3449 ms]
protobuf::encode/prometheus_client (prost/prometheus): 100 metrics * 100000 times
                        time:   [2.9824 ms 3.0190 ms 3.0678 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 100000 times
                        time:   [683.64 µs 685.19 µs 687.14 µs]
```
