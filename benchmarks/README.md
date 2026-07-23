# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.96.0 (ac68faa20 2026-05-25)

## Metric

```bash
cargo bench --bench metric -- --quiet
# Or `just bench metric`
```

```text
counter(u64)::inc/metrics
                        time:   [7.2057 ns 7.2389 ns 7.2780 ns]
counter(u64)::inc/measured
                        time:   [2.1861 ns 2.1941 ns 2.2030 ns]
counter(u64)::inc/prometheus
                        time:   [2.2097 ns 2.2183 ns 2.2287 ns]
counter(u64)::inc/prometheus_client
                        time:   [2.1290 ns 2.1405 ns 2.1553 ns]
counter(u64)::inc/fastmetrics
                        time:   [2.2102 ns 2.2646 ns 2.3664 ns]

counter(u64)::saturating_inc/fastmetrics
                        time:   [2.5093 ns 2.5701 ns 2.6888 ns]

counter(f64)::inc/metrics
                        time:   [7.1811 ns 7.2114 ns 7.2500 ns]
counter(f64)::inc/prometheus
                        time:   [10.805 ns 10.844 ns 10.889 ns]
counter(f64)::inc/prometheus_client
                        time:   [5.6485 ns 5.7108 ns 5.8075 ns]
counter(f64)::inc/fastmetrics
                        time:   [5.6875 ns 5.7057 ns 5.7268 ns]

gauge(i64)::set/metrics time:   [7.2178 ns 7.2475 ns 7.2801 ns]
gauge(i64)::set/measured
                        time:   [987.69 ps 997.00 ps 1.0086 ns]
gauge(i64)::set/prometheus
                        time:   [643.65 ps 654.74 ps 668.56 ps]
gauge(i64)::set/prometheus_client
                        time:   [1.8110 ns 1.8363 ns 1.8686 ns]
gauge(i64)::set/fastmetrics
                        time:   [641.58 ps 658.12 ps 690.02 ps]

gauge(i64)::inc_by/metrics
                        time:   [6.9247 ns 7.0160 ns 7.1372 ns]
gauge(i64)::inc_by/measured
                        time:   [2.1592 ns 2.1698 ns 2.1822 ns]
gauge(i64)::inc_by/prometheus
                        time:   [2.1685 ns 2.1849 ns 2.2047 ns]
gauge(i64)::inc_by/prometheus_client
                        time:   [2.2277 ns 2.2403 ns 2.2543 ns]
gauge(i64)::inc_by/fastmetrics
                        time:   [2.1522 ns 2.1594 ns 2.1679 ns]

gauge(i64)::saturating_inc_by/fastmetrics
                        time:   [3.1406 ns 3.1624 ns 3.1881 ns]

gauge(i64)::dec_by/metrics
                        time:   [6.8885 ns 6.9254 ns 6.9692 ns]
gauge(i64)::dec_by/measured
                        time:   [2.1505 ns 2.1663 ns 2.1903 ns]
gauge(i64)::dec_by/prometheus
                        time:   [2.1583 ns 2.1710 ns 2.1869 ns]
gauge(i64)::dec_by/prometheus_client
                        time:   [2.2247 ns 2.2414 ns 2.2608 ns]
gauge(i64)::dec_by/fastmetrics
                        time:   [2.1500 ns 2.1637 ns 2.1815 ns]

gauge(i64)::saturating_dec_by/fastmetrics
                        time:   [3.1345 ns 3.1491 ns 3.1662 ns]

gauge(f64)::set/metrics time:   [7.1997 ns 7.2312 ns 7.2650 ns]
gauge(f64)::set/measured
                        time:   [999.03 ps 1.0093 ns 1.0204 ns]
gauge(f64)::set/prometheus
                        time:   [706.31 ps 716.87 ps 733.92 ps]
gauge(f64)::set/prometheus_client
                        time:   [1.8064 ns 1.8238 ns 1.8442 ns]
gauge(f64)::set/fastmetrics
                        time:   [705.40 ps 715.62 ps 730.85 ps]

gauge(f64)::inc_by/metrics
                        time:   [6.8623 ns 6.8823 ns 6.9053 ns]
gauge(f64)::inc_by/measured
                        time:   [10.791 ns 10.821 ns 10.853 ns]
gauge(f64)::inc_by/prometheus
                        time:   [10.770 ns 10.801 ns 10.834 ns]
gauge(f64)::inc_by/prometheus_client
                        time:   [5.7813 ns 5.8100 ns 5.8452 ns]
gauge(f64)::inc_by/fastmetrics
                        time:   [5.8073 ns 5.8402 ns 5.8807 ns]

gauge(f64)::dec_by/metrics
                        time:   [6.9446 ns 7.0602 ns 7.2061 ns]
gauge(f64)::dec_by/measured
                        time:   [10.803 ns 10.846 ns 10.896 ns]
gauge(f64)::dec_by/prometheus
                        time:   [10.775 ns 10.817 ns 10.865 ns]
gauge(f64)::dec_by/prometheus_client
                        time:   [5.7247 ns 5.7466 ns 5.7736 ns]
gauge(f64)::dec_by/fastmetrics
                        time:   [5.7868 ns 5.8715 ns 5.9976 ns]

histogram::observe/metrics
                        time:   [10.224 ns 10.275 ns 10.332 ns]
histogram::observe/measured
                        time:   [11.431 ns 11.468 ns 11.509 ns]
histogram::observe/prometheus
                        time:   [10.953 ns 10.990 ns 11.033 ns]
histogram::observe/prometheus_client
                        time:   [8.9743 ns 9.0113 ns 9.0548 ns]
histogram::observe/fastmetrics
                        time:   [5.7308 ns 5.7600 ns 5.7982 ns]
```

## Metric Family

```bash
cargo bench --bench family -- --quiet
# Or `just bench family`
```

Each group of families includes a counter (u64) family and a histogram family.

```text
family with empty labels/metrics_cached
                        time:   [12.796 ns 12.922 ns 13.062 ns]
family with empty labels/metrics_dynamic
                        time:   [54.766 ns 60.859 ns 72.999 ns]
family with empty labels/measured
                        time:   [13.950 ns 15.270 ns 17.311 ns]
family with empty labels/prometheus
                        time:   [25.090 ns 25.309 ns 25.559 ns]
family with empty labels/prometheus_client
                        time:   [29.548 ns 30.099 ns 30.974 ns]
family with empty labels/fastmetrics_cached
                        time:   [5.8494 ns 5.9743 ns 6.1358 ns]
family with empty labels/fastmetrics_dynamic
                        time:   [17.845 ns 18.014 ns 18.223 ns]

family with custom labels/metrics_cached
                        time:   [12.930 ns 13.319 ns 13.867 ns]
family with custom labels/metrics_dynamic
                        time:   [105.61 ns 106.10 ns 106.64 ns]
family with custom labels/measured
                        time:   [14.858 ns 14.971 ns 15.125 ns]
family with custom labels/prometheus
                        time:   [26.006 ns 26.116 ns 26.243 ns]
family with custom labels/prometheus_client
                        time:   [40.175 ns 40.492 ns 41.012 ns]
family with custom labels/fastmetrics_cached
                        time:   [6.2220 ns 6.3452 ns 6.5367 ns]
family with custom labels/fastmetrics_dynamic
                        time:   [19.570 ns 19.648 ns 19.733 ns]

family with [(&'static str, &'static str)] labels/prometheus_client
                        time:   [72.144 ns 72.467 ns 72.821 ns]
family with [(&'static str, &'static str)] labels/fastmetrics
                        time:   [51.094 ns 51.382 ns 51.689 ns]

family with Vec<(&'static str, &'static str)> labels/prometheus_client
                        time:   [91.926 ns 92.526 ns 93.188 ns]
family with Vec<(&'static str, &'static str)> labels/fastmetrics
                        time:   [64.116 ns 64.372 ns 64.669 ns]

family with Vec<(String, String)> labels/prometheus_client
                        time:   [111.22 ns 113.54 ns 117.09 ns]
family with Vec<(String, String)> labels/fastmetrics
                        time:   [84.306 ns 87.024 ns 92.185 ns]
```

## Text Encoding

```bash
cargo bench --bench text  -- --quiet
# Or `just bench text`
```

Each group of metrics includes a counter (u64) and a histogram.

```text
text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [5.0255 ms 5.1340 ms 5.2247 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [433.28 µs 441.31 µs 451.11 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [555.06 µs 559.42 µs 564.15 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [370.93 µs 373.16 µs 375.50 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100 times
                        time:   [178.26 µs 182.25 µs 187.51 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 100 times
                        time:   [173.40 µs 175.46 µs 178.04 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [168.58 µs 170.16 µs 171.99 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 100 times
                        time:   [293.16 µs 295.60 µs 298.43 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100 times
                        time:   [182.00 µs 183.97 µs 186.17 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 100 times
                        time:   [179.66 µs 181.52 µs 183.57 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100 times
                        time:   [174.87 µs 180.60 µs 190.27 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 100 times
                        time:   [298.76 µs 301.90 µs 305.46 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [10.192 ms 10.414 ms 10.739 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [466.03 µs 467.68 µs 469.43 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [612.43 µs 616.06 µs 620.66 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [403.18 µs 405.35 µs 408.13 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 1000 times
                        time:   [193.68 µs 196.12 µs 199.35 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 1000 times
                        time:   [189.86 µs 197.32 µs 210.50 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [181.46 µs 182.81 µs 184.44 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 1000 times
                        time:   [314.69 µs 316.40 µs 318.51 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 1000 times
                        time:   [193.27 µs 194.75 µs 196.48 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 1000 times
                        time:   [190.70 µs 191.76 µs 193.17 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 1000 times
                        time:   [183.26 µs 184.62 µs 186.30 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 1000 times
                        time:   [316.94 µs 318.30 µs 319.94 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [10.568 ms 10.767 ms 10.955 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [473.88 µs 475.30 µs 476.80 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [625.68 µs 634.34 µs 648.19 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [405.93 µs 407.53 µs 409.30 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 10000 times
                        time:   [192.05 µs 193.79 µs 195.90 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 10000 times
                        time:   [190.12 µs 191.12 µs 192.45 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [183.84 µs 187.58 µs 194.08 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 10000 times
                        time:   [316.15 µs 317.47 µs 319.08 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 10000 times
                        time:   [194.71 µs 195.85 µs 197.05 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 10000 times
                        time:   [193.51 µs 194.76 µs 196.61 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 10000 times
                        time:   [185.93 µs 187.63 µs 189.71 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 10000 times
                        time:   [319.25 µs 325.51 µs 335.89 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [10.971 ms 11.316 ms 11.814 ms]
text::encode/measured (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [473.17 µs 479.86 µs 491.42 µs]
text::encode/prometheus (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [645.73 µs 648.44 µs 651.46 µs]
text::encode/prometheus_client (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [390.20 µs 393.32 µs 398.30 µs]
text::encode/fastmetrics (prometheus 0.0.4): 10 metrics * 100000 times
                        time:   [192.85 µs 194.32 µs 195.95 µs]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 10 metrics * 100000 times
                        time:   [190.61 µs 191.98 µs 194.22 µs]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [184.31 µs 191.02 µs 200.94 µs]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 10 metrics * 100000 times
                        time:   [321.41 µs 327.22 µs 333.51 µs]
text::encode/fastmetrics (openmetrics 0.0.1): 10 metrics * 100000 times
                        time:   [195.98 µs 197.86 µs 200.13 µs]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 10 metrics * 100000 times
                        time:   [192.75 µs 193.72 µs 195.10 µs]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 10 metrics * 100000 times
                        time:   [186.96 µs 193.12 µs 202.69 µs]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 10 metrics * 100000 times
                        time:   [318.48 µs 327.51 µs 343.64 µs]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [50.745 ms 51.001 ms 51.243 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [4.3702 ms 4.4362 ms 4.5254 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [5.8957 ms 5.9290 ms 5.9648 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [3.7974 ms 3.8560 ms 3.9474 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100 times
                        time:   [1.8064 ms 1.8176 ms 1.8292 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 100 times
                        time:   [1.7961 ms 1.8071 ms 1.8206 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.7373 ms 1.7630 ms 1.7991 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 100 times
                        time:   [3.0260 ms 3.0697 ms 3.1334 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100 times
                        time:   [1.8302 ms 1.8459 ms 1.8624 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 100 times
                        time:   [1.8158 ms 1.8243 ms 1.8362 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100 times
                        time:   [1.7467 ms 1.7609 ms 1.7781 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 100 times
                        time:   [3.0003 ms 3.0110 ms 3.0257 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [101.91 ms 102.50 ms 103.17 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [4.6855 ms 4.6990 ms 4.7132 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [6.4434 ms 6.4770 ms 6.5137 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [4.0944 ms 4.1104 ms 4.1291 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 1000 times
                        time:   [1.9453 ms 1.9533 ms 1.9607 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 1000 times
                        time:   [1.9176 ms 1.9261 ms 1.9380 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.8517 ms 1.8804 ms 1.9248 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 1000 times
                        time:   [3.1966 ms 3.2062 ms 3.2203 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 1000 times
                        time:   [1.9590 ms 1.9725 ms 1.9866 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 1000 times
                        time:   [1.9601 ms 2.0276 ms 2.1202 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 1000 times
                        time:   [1.8822 ms 1.9142 ms 1.9583 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 1000 times
                        time:   [3.2238 ms 3.2666 ms 3.3454 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [106.59 ms 107.13 ms 107.71 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [4.8084 ms 4.8319 ms 4.8617 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [6.5871 ms 6.6253 ms 6.6698 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [4.1169 ms 4.1319 ms 4.1487 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 10000 times
                        time:   [1.9717 ms 1.9896 ms 2.0132 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 10000 times
                        time:   [1.9390 ms 1.9504 ms 1.9665 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.8835 ms 1.9104 ms 1.9552 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 10000 times
                        time:   [3.2146 ms 3.2287 ms 3.2471 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 10000 times
                        time:   [1.9862 ms 2.0369 ms 2.1250 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 10000 times
                        time:   [1.9678 ms 1.9812 ms 1.9989 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 10000 times
                        time:   [1.8924 ms 1.9055 ms 1.9218 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 10000 times
                        time:   [3.2397 ms 3.2498 ms 3.2618 ms]

text::encode/metrics_exporter_prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [113.25 ms 114.34 ms 115.42 ms]
text::encode/measured (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [4.7565 ms 4.7707 ms 4.7865 ms]
text::encode/prometheus (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [6.7843 ms 6.8096 ms 6.8371 ms]
text::encode/prometheus_client (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [3.9316 ms 3.9451 ms 3.9602 ms]
text::encode/fastmetrics (prometheus 0.0.4): 100 metrics * 100000 times
                        time:   [1.9711 ms 2.0052 ms 2.0699 ms]
text::encode/fastmetrics (prometheus 1.0.0, legacy + underscores): 100 metrics * 100000 times
                        time:   [1.9495 ms 1.9594 ms 1.9715 ms]
text::encode/fastmetrics (prometheus 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.8581 ms 1.8705 ms 1.8863 ms]
text::encode/fastmetrics (prometheus 1.0.0, utf8 + underscores): 100 metrics * 100000 times
                        time:   [3.2014 ms 3.2192 ms 3.2528 ms]
text::encode/fastmetrics (openmetrics 0.0.1): 100 metrics * 100000 times
                        time:   [1.9776 ms 1.9876 ms 1.9975 ms]
text::encode/fastmetrics (openmetrics 1.0.0, legacy + underscores): 100 metrics * 100000 times
                        time:   [1.9922 ms 2.0061 ms 2.0237 ms]
text::encode/fastmetrics (openmetrics 1.0.0, allow utf8): 100 metrics * 100000 times
                        time:   [1.9079 ms 1.9228 ms 1.9414 ms]
text::encode/fastmetrics (openmetrics 1.0.0, utf8 + underscores): 100 metrics * 100000 times
                        time:   [3.2507 ms 3.2788 ms 3.3320 ms]
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
                        time:   [4.2375 ms 4.3280 ms 4.4137 ms]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 100 times
                        time:   [167.62 µs 168.54 µs 169.46 µs]
protobuf::encode/prometheus_client (prost/prometheus): 10 metrics * 100 times
                        time:   [240.95 µs 242.21 µs 243.48 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 100 times
                        time:   [213.08 µs 214.12 µs 215.19 µs]
protobuf::encode/fastmetrics (prost/openmetrics): 10 metrics * 100 times
                        time:   [233.41 µs 234.69 µs 236.07 µs]
protobuf::encode/fastmetrics (protobuf/promtheus): 10 metrics * 100 times
                        time:   [219.40 µs 221.33 µs 223.41 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100 times
                        time:   [220.06 µs 222.48 µs 226.06 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 1000 times
                        time:   [9.1086 ms 9.3386 ms 9.5198 ms]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 1000 times
                        time:   [182.04 µs 182.42 µs 182.86 µs]
protobuf::encode/prometheus_client (prost/prometheus): 10 metrics * 1000 times
                        time:   [256.02 µs 256.55 µs 257.13 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 1000 times
                        time:   [226.37 µs 227.12 µs 228.08 µs]
protobuf::encode/fastmetrics (prost/openmetrics): 10 metrics * 1000 times
                        time:   [248.59 µs 249.51 µs 251.00 µs]
protobuf::encode/fastmetrics (protobuf/promtheus): 10 metrics * 1000 times
                        time:   [234.12 µs 235.39 µs 236.75 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 1000 times
                        time:   [236.41 µs 237.57 µs 239.15 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 10000 times
                        time:   [9.7219 ms 9.8431 ms 9.9660 ms]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 10000 times
                        time:   [182.72 µs 185.75 µs 190.75 µs]
protobuf::encode/prometheus_client (prost/prometheus): 10 metrics * 10000 times
                        time:   [258.51 µs 258.99 µs 259.53 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 10000 times
                        time:   [226.45 µs 227.05 µs 227.82 µs]
protobuf::encode/fastmetrics (prost/openmetrics): 10 metrics * 10000 times
                        time:   [252.19 µs 253.24 µs 254.50 µs]
protobuf::encode/fastmetrics (protobuf/promtheus): 10 metrics * 10000 times
                        time:   [234.35 µs 236.23 µs 238.58 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 10000 times
                        time:   [239.85 µs 240.48 µs 241.16 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 10 metrics * 100000 times
                        time:   [10.218 ms 10.662 ms 11.207 ms]
protobuf::encode/prometheus (protobuf/prometheus): 10 metrics * 100000 times
                        time:   [186.76 µs 187.87 µs 189.29 µs]
protobuf::encode/prometheus_client (prost/prometheus): 10 metrics * 100000 times
                        time:   [268.47 µs 269.74 µs 271.13 µs]
protobuf::encode/fastmetrics (prost/prometheus): 10 metrics * 100000 times
                        time:   [232.00 µs 234.87 µs 239.60 µs]
protobuf::encode/fastmetrics (prost/openmetrics): 10 metrics * 100000 times
                        time:   [259.01 µs 259.77 µs 260.65 µs]
protobuf::encode/fastmetrics (protobuf/promtheus): 10 metrics * 100000 times
                        time:   [240.38 µs 244.13 µs 249.22 µs]
protobuf::encode/fastmetrics(protobuf/openmetrics): 10 metrics * 100000 times
                        time:   [248.92 µs 250.55 µs 252.29 µs]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 100 times
                        time:   [44.076 ms 44.415 ms 44.720 ms]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 100 times
                        time:   [1.9181 ms 1.9455 ms 1.9831 ms]
protobuf::encode/prometheus_client (prost/prometheus): 100 metrics * 100 times
                        time:   [2.6513 ms 2.7080 ms 2.7831 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 100 times
                        time:   [2.3391 ms 2.3544 ms 2.3712 ms]
protobuf::encode/fastmetrics (prost/openmetrics): 100 metrics * 100 times
                        time:   [2.5220 ms 2.5635 ms 2.6252 ms]
protobuf::encode/fastmetrics (protobuf/promtheus): 100 metrics * 100 times
                        time:   [2.3438 ms 2.3573 ms 2.3718 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100 times
                        time:   [2.3263 ms 2.3387 ms 2.3528 ms]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 1000 times
                        time:   [95.294 ms 96.045 ms 96.852 ms]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 1000 times
                        time:   [2.0508 ms 2.0821 ms 2.1220 ms]
protobuf::encode/prometheus_client (prost/prometheus): 100 metrics * 1000 times
                        time:   [2.8333 ms 2.8502 ms 2.8686 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 1000 times
                        time:   [2.5024 ms 2.5191 ms 2.5372 ms]
protobuf::encode/fastmetrics (prost/openmetrics): 100 metrics * 1000 times
                        time:   [2.7073 ms 2.7219 ms 2.7392 ms]
protobuf::encode/fastmetrics (protobuf/promtheus): 100 metrics * 1000 times
                        time:   [2.5075 ms 2.5225 ms 2.5384 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 1000 times
                        time:   [2.5142 ms 2.5249 ms 2.5364 ms]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 10000 times
                        time:   [98.763 ms 99.284 ms 99.700 ms]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 10000 times
                        time:   [2.0751 ms 2.1146 ms 2.1708 ms]
protobuf::encode/prometheus_client (prost/prometheus): 100 metrics * 10000 times
                        time:   [2.8567 ms 2.8717 ms 2.8875 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 10000 times
                        time:   [2.5111 ms 2.5360 ms 2.5694 ms]
protobuf::encode/fastmetrics (prost/openmetrics): 100 metrics * 10000 times
                        time:   [2.7513 ms 2.8282 ms 2.9458 ms]
protobuf::encode/fastmetrics (protobuf/promtheus): 100 metrics * 10000 times
                        time:   [2.5226 ms 2.5765 ms 2.6665 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 10000 times
                        time:   [2.5543 ms 2.5676 ms 2.5822 ms]

protobuf::encode/metrics_exporter_prometheus (prost/prometheus): 100 metrics * 100000 times
                        time:   [105.88 ms 107.76 ms 110.18 ms]
protobuf::encode/prometheus (protobuf/prometheus): 100 metrics * 100000 times
                        time:   [2.0718 ms 2.1173 ms 2.1930 ms]
protobuf::encode/prometheus_client (prost/prometheus): 100 metrics * 100000 times
                        time:   [2.9076 ms 2.9244 ms 2.9426 ms]
protobuf::encode/fastmetrics (prost/prometheus): 100 metrics * 100000 times
                        time:   [2.5290 ms 2.5426 ms 2.5569 ms]
protobuf::encode/fastmetrics (prost/openmetrics): 100 metrics * 100000 times
                        time:   [2.7976 ms 2.8143 ms 2.8323 ms]
protobuf::encode/fastmetrics (protobuf/promtheus): 100 metrics * 100000 times
                        time:   [2.5240 ms 2.5373 ms 2.5513 ms]
protobuf::encode/fastmetrics(protobuf/openmetrics): 100 metrics * 100000 times
                        time:   [2.6354 ms 2.6940 ms 2.7961 ms]
```
