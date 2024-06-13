window.BENCHMARK_DATA = {
  "lastUpdate": 1718271287564,
  "repoUrl": "https://github.com/SpriteOvO/spdlog-rs",
  "entries": {
    "spdlog-rs on Linux": [
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "8b69142c87a101d3c4815a7ea5c7f87cc2072068",
          "message": "CI run benchmarks, record results, alert if threshold is exceeded",
          "timestamp": "2024-06-13T06:19:26+08:00",
          "tree_id": "352d743f5dbe8e67dce77e5ffd040c6c1973ad10",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/8b69142c87a101d3c4815a7ea5c7f87cc2072068"
        },
        "date": 1718231048498,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 216.82,
            "range": "± 6.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 198.58,
            "range": "± 1.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 222.23,
            "range": "± 12.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 224.71,
            "range": "± 7.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.47,
            "range": "± 0.02",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "94dd5144c7bb987b7d54db8064b91ce7e869fbc7",
          "message": "CI run benchmarks, record results, alert if threshold is exceeded",
          "timestamp": "2024-06-13T06:26:48+08:00",
          "tree_id": "321b193a5f07ea71ad95c8471f59ec45da558791",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/94dd5144c7bb987b7d54db8064b91ce7e869fbc7"
        },
        "date": 1718231249598,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 218.48,
            "range": "± 3.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 200.68,
            "range": "± 5.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 224.51,
            "range": "± 47.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 227.09,
            "range": "± 171.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.47,
            "range": "± 0.01",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "963003e2af8fa33385737758408abd45ff00bb38",
          "message": "CI run benchmarks, record results, alert if threshold is exceeded",
          "timestamp": "2024-06-13T06:32:10+08:00",
          "tree_id": "49268ac4ef64ee1da3270f8f1fb6b2e063300676",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/963003e2af8fa33385737758408abd45ff00bb38"
        },
        "date": 1718231626010,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 217.42,
            "range": "± 49.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 191.74,
            "range": "± 2.92",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 226.06,
            "range": "± 123.31",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 221.48,
            "range": "± 13.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.47,
            "range": "± 0.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 127.61,
            "range": "± 55.93",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 250.34,
            "range": "± 7.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 255.28,
            "range": "± 13.18",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 83.99,
            "range": "± 5.82",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.76,
            "range": "± 8.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 98.57,
            "range": "± 25.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 99.05,
            "range": "± 1.12",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 130.82,
            "range": "± 4.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 88.68,
            "range": "± 1.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.22,
            "range": "± 0.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 39.71,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 39.84,
            "range": "± 0.84",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 126.83,
            "range": "± 19.90",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 88.15,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 87.87,
            "range": "± 0.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.93,
            "range": "± 0.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40,
            "range": "± 0.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.62,
            "range": "± 0.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 41.56,
            "range": "± 4.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 101.89,
            "range": "± 8.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 101.87,
            "range": "± 2.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 88.37,
            "range": "± 1.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 39.64,
            "range": "± 0.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 88.14,
            "range": "± 1.45",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 83.98,
            "range": "± 1.33",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 83.58,
            "range": "± 1.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 102.59,
            "range": "± 4.12",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.18,
            "range": "± 7.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 272.08,
            "range": "± 8.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 87.97,
            "range": "± 1.17",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 39.95,
            "range": "± 0.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 58.15,
            "range": "± 1.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 98.04,
            "range": "± 3.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 106.57,
            "range": "± 10.13",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 92.15,
            "range": "± 1.60",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 88.26,
            "range": "± 2.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 88.2,
            "range": "± 8.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 83.98,
            "range": "± 1.59",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.66,
            "range": "± 5.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 88.06,
            "range": "± 1.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 88.08,
            "range": "± 1.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88.11,
            "range": "± 1.26",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 42.22,
            "range": "± 1.34",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 102.11,
            "range": "± 8.87",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 102.49,
            "range": "± 8.39",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 127.97,
            "range": "± 15.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 89.01,
            "range": "± 2.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.44,
            "range": "± 1.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.56,
            "range": "± 0.81",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 43.01,
            "range": "± 3.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 121.82,
            "range": "± 15.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 89.67,
            "range": "± 2.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 89.16,
            "range": "± 2.18",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 45.45,
            "range": "± 3.27",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 42.8,
            "range": "± 1.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 41.99,
            "range": "± 0.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 45.05,
            "range": "± 1.18",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 103.77,
            "range": "± 1.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 109.02,
            "range": "± 8.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 89.49,
            "range": "± 3.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 42.55,
            "range": "± 1.08",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 89.45,
            "range": "± 2.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 88.98,
            "range": "± 2.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 89.64,
            "range": "± 1.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 109.51,
            "range": "± 5.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 45.42,
            "range": "± 4.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 280.7,
            "range": "± 5.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 89.48,
            "range": "± 12.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 42.87,
            "range": "± 1.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.51,
            "range": "± 1.59",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 102.84,
            "range": "± 4.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 104.88,
            "range": "± 9.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 96.31,
            "range": "± 1.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 89.56,
            "range": "± 2.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 88.69,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 89.3,
            "range": "± 1.39",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 89.3,
            "range": "± 6.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 88.53,
            "range": "± 3.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 89.61,
            "range": "± 2.63",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "78cd70e3424fe1737849d965c0bc3255efc1885e",
          "message": "CI run benchmarks, record results, alert if threshold is exceeded",
          "timestamp": "2024-06-13T07:02:24+08:00",
          "tree_id": "0519a9ae59fb8528653ffae29491fbc58498def6",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/78cd70e3424fe1737849d965c0bc3255efc1885e"
        },
        "date": 1718233447796,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 219.34,
            "range": "± 8.17",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 197.84,
            "range": "± 2.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 220.14,
            "range": "± 7.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 219.17,
            "range": "± 36.41",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.47,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 118.56,
            "range": "± 13.93",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 244.86,
            "range": "± 31.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 260.27,
            "range": "± 176.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 84.09,
            "range": "± 4.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.69,
            "range": "± 9.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 98.28,
            "range": "± 1.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 97.38,
            "range": "± 7.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 129.47,
            "range": "± 11.39",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 88.86,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 39.69,
            "range": "± 1.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 42.1,
            "range": "± 0.42",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 40.04,
            "range": "± 0.84",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 120.95,
            "range": "± 13.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 88.21,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 87.92,
            "range": "± 0.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.81,
            "range": "± 3.64",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.13,
            "range": "± 0.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.61,
            "range": "± 0.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 42.65,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 100.33,
            "range": "± 5.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 101.77,
            "range": "± 6.33",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 88.24,
            "range": "± 1.06",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 39.63,
            "range": "± 0.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 88.4,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 83.96,
            "range": "± 1.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 83.97,
            "range": "± 1.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 102.05,
            "range": "± 7.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.36,
            "range": "± 4.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 272.35,
            "range": "± 8.08",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 88.07,
            "range": "± 1.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 39.95,
            "range": "± 0.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 57.95,
            "range": "± 3.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 97.42,
            "range": "± 2.77",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 105.22,
            "range": "± 36.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 90.7,
            "range": "± 3.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 88.21,
            "range": "± 0.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 88.15,
            "range": "± 5.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 83.8,
            "range": "± 1.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.7,
            "range": "± 1.87",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 87.91,
            "range": "± 1.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 88.18,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 89.25,
            "range": "± 1.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 42.29,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 102.95,
            "range": "± 3.84",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 102.83,
            "range": "± 6.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 131.18,
            "range": "± 2.69",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 89.47,
            "range": "± 2.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 44.07,
            "range": "± 0.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.16,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 44.04,
            "range": "± 0.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 121.33,
            "range": "± 11.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 90.72,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 89.6,
            "range": "± 2.28",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 45.34,
            "range": "± 2.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 42.99,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 43.49,
            "range": "± 0.59",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 44.95,
            "range": "± 1.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 103.91,
            "range": "± 3.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 102.78,
            "range": "± 1.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 89.76,
            "range": "± 2.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 42.48,
            "range": "± 0.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 90.08,
            "range": "± 2.80",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 89.54,
            "range": "± 1.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 89.63,
            "range": "± 1.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 105.27,
            "range": "± 5.71",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 45.55,
            "range": "± 4.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 282.5,
            "range": "± 11.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 88.92,
            "range": "± 3.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 42.8,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.3,
            "range": "± 0.71",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 102.43,
            "range": "± 2.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 104.53,
            "range": "± 7.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 95.55,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 90.46,
            "range": "± 2.84",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 88.96,
            "range": "± 5.92",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 90.34,
            "range": "± 1.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 89.28,
            "range": "± 1.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 89.34,
            "range": "± 4.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 90.09,
            "range": "± 2.55",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "8ba4c857dd3d24ba8b80fd3941136aee2808293e",
          "message": "CI run benchmarks, record results, alert if threshold is exceeded",
          "timestamp": "2024-06-13T07:07:32+08:00",
          "tree_id": "d86e81e23e19da2d95b20b22ce0f6229edd183d5",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/8ba4c857dd3d24ba8b80fd3941136aee2808293e"
        },
        "date": 1718233951234,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 215.82,
            "range": "± 2.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 197.85,
            "range": "± 2.41",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 221.06,
            "range": "± 10.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 225.11,
            "range": "± 34.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.48,
            "range": "± 0.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 118.67,
            "range": "± 8.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 254.25,
            "range": "± 28.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 254.89,
            "range": "± 22.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 84.53,
            "range": "± 1.17",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.67,
            "range": "± 0.26",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 97.52,
            "range": "± 8.55",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 97.02,
            "range": "± 4.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 127.23,
            "range": "± 11.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 89.43,
            "range": "± 1.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.32,
            "range": "± 0.69",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 39.74,
            "range": "± 1.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 40.04,
            "range": "± 0.90",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 121.13,
            "range": "± 13.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 88.07,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 88.03,
            "range": "± 0.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.58,
            "range": "± 3.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.1,
            "range": "± 0.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.63,
            "range": "± 0.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 41.67,
            "range": "± 0.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 101.15,
            "range": "± 7.28",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 102.17,
            "range": "± 5.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 88.19,
            "range": "± 0.84",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 39.65,
            "range": "± 0.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 88.18,
            "range": "± 1.06",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 84.08,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 84.16,
            "range": "± 2.13",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 114.02,
            "range": "± 8.08",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.5,
            "range": "± 3.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 272.9,
            "range": "± 9.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 87.92,
            "range": "± 1.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 40.38,
            "range": "± 1.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 58.49,
            "range": "± 2.87",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 96.73,
            "range": "± 2.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 106.08,
            "range": "± 6.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 90.75,
            "range": "± 1.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 88.31,
            "range": "± 0.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 88.21,
            "range": "± 6.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 84.29,
            "range": "± 1.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.78,
            "range": "± 6.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 87.95,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 88.12,
            "range": "± 0.77",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88.1,
            "range": "± 1.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 41.9,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 101.96,
            "range": "± 6.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 102.38,
            "range": "± 6.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 127.89,
            "range": "± 12.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 89.42,
            "range": "± 2.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.5,
            "range": "± 1.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.59,
            "range": "± 1.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 42.95,
            "range": "± 1.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 121.88,
            "range": "± 13.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 89.82,
            "range": "± 3.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 89.26,
            "range": "± 2.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 45.41,
            "range": "± 2.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 43.16,
            "range": "± 0.81",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 42.28,
            "range": "± 0.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 45.09,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 103.94,
            "range": "± 2.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 108.67,
            "range": "± 7.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 89.91,
            "range": "± 3.45",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 42.58,
            "range": "± 1.34",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 89.59,
            "range": "± 1.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 89.52,
            "range": "± 1.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 89.05,
            "range": "± 1.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 115.11,
            "range": "± 8.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 45.58,
            "range": "± 4.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 282.01,
            "range": "± 10.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 89.11,
            "range": "± 2.80",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 42.88,
            "range": "± 0.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.49,
            "range": "± 2.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 102.76,
            "range": "± 2.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 104.95,
            "range": "± 7.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 96.33,
            "range": "± 1.81",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 89.91,
            "range": "± 2.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 89.01,
            "range": "± 5.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 89.68,
            "range": "± 1.66",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 89.18,
            "range": "± 1.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 88.27,
            "range": "± 2.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 90.12,
            "range": "± 1.98",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "a7ef97ac53bc5c82b1a72264ae97dc20f308f9f6",
          "message": "CI run benchmarks, record results, alert if threshold is exceeded",
          "timestamp": "2024-06-13T07:13:30+08:00",
          "tree_id": "36f130d583dad9b295626368ab8e15ac80cecb42",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/a7ef97ac53bc5c82b1a72264ae97dc20f308f9f6"
        },
        "date": 1718234379024,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 216.28,
            "range": "± 2.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 197.39,
            "range": "± 2.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 225.4,
            "range": "± 10.41",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 218.84,
            "range": "± 43.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.48,
            "range": "± 0.06",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 119.38,
            "range": "± 15.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 264.15,
            "range": "± 17.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 273.05,
            "range": "± 8.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 83.47,
            "range": "± 6.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.92,
            "range": "± 6.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 106.24,
            "range": "± 8.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 97.39,
            "range": "± 6.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 127.27,
            "range": "± 11.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 88.98,
            "range": "± 1.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.02,
            "range": "± 1.13",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 39.69,
            "range": "± 0.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 39.86,
            "range": "± 1.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 125.11,
            "range": "± 9.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 88.31,
            "range": "± 1.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 88.05,
            "range": "± 0.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.66,
            "range": "± 2.15",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.03,
            "range": "± 0.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.61,
            "range": "± 0.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 41.49,
            "range": "± 0.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 100.27,
            "range": "± 6.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 105.92,
            "range": "± 7.60",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 88.1,
            "range": "± 1.39",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 39.78,
            "range": "± 0.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 88.37,
            "range": "± 1.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 84.01,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 84.18,
            "range": "± 2.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 102.43,
            "range": "± 5.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.3,
            "range": "± 3.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 271.19,
            "range": "± 4.31",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 88.12,
            "range": "± 1.60",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 40.35,
            "range": "± 0.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 58.06,
            "range": "± 4.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 97.33,
            "range": "± 9.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 105.93,
            "range": "± 6.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 90.09,
            "range": "± 2.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 88.47,
            "range": "± 4.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 88.14,
            "range": "± 7.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 84.03,
            "range": "± 5.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.68,
            "range": "± 6.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 88.15,
            "range": "± 1.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 88.34,
            "range": "± 1.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88.1,
            "range": "± 1.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 42.09,
            "range": "± 1.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 111.19,
            "range": "± 9.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 102.52,
            "range": "± 5.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 127.71,
            "range": "± 10.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 89.28,
            "range": "± 2.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.39,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.57,
            "range": "± 1.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 42.81,
            "range": "± 0.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 121.15,
            "range": "± 14.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 90.13,
            "range": "± 1.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 89.3,
            "range": "± 1.15",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 45.33,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 42.95,
            "range": "± 7.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 42.05,
            "range": "± 0.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 44.99,
            "range": "± 1.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 103.77,
            "range": "± 2.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 108.69,
            "range": "± 8.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 90.5,
            "range": "± 1.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 42.77,
            "range": "± 1.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 90.67,
            "range": "± 2.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 90.2,
            "range": "± 2.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 89.11,
            "range": "± 1.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 109.63,
            "range": "± 9.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 45.41,
            "range": "± 4.93",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 278.34,
            "range": "± 9.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 89.4,
            "range": "± 3.13",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 43.07,
            "range": "± 1.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.38,
            "range": "± 1.41",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 102.52,
            "range": "± 5.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 104.55,
            "range": "± 9.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 96.03,
            "range": "± 1.31",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 90.34,
            "range": "± 5.68",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 89.01,
            "range": "± 9.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 89.73,
            "range": "± 13.11",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 89.45,
            "range": "± 5.98",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 88.67,
            "range": "± 2.71",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 89.92,
            "range": "± 2.76",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "19d207e1c213aa9dd3a4ab077738784ca943dc3e",
          "message": "CI run benchmarks, record results, alert if threshold is exceeded",
          "timestamp": "2024-06-13T07:15:01+08:00",
          "tree_id": "d0c2f533389e5654ae07e547d95f950adf567776",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/19d207e1c213aa9dd3a4ab077738784ca943dc3e"
        },
        "date": 1718234482721,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 216.75,
            "range": "± 4.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 198.58,
            "range": "± 2.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 222.03,
            "range": "± 95.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 220.61,
            "range": "± 1.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.48,
            "range": "± 1.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 125.89,
            "range": "± 17.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 253.08,
            "range": "± 25.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 255.86,
            "range": "± 20.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 84.06,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.62,
            "range": "± 0.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 97.14,
            "range": "± 8.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 97.19,
            "range": "± 5.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 130.68,
            "range": "± 4.39",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 88.89,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.36,
            "range": "± 1.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 39.86,
            "range": "± 0.60",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 40.05,
            "range": "± 0.59",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 121.97,
            "range": "± 16.11",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 88.08,
            "range": "± 1.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 88.01,
            "range": "± 0.82",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.98,
            "range": "± 1.93",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.06,
            "range": "± 1.18",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.64,
            "range": "± 0.59",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 41.52,
            "range": "± 0.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 100.34,
            "range": "± 7.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 102.34,
            "range": "± 6.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 88.08,
            "range": "± 0.80",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 39.61,
            "range": "± 0.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 88.17,
            "range": "± 6.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 84.16,
            "range": "± 1.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 84.1,
            "range": "± 5.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 102.48,
            "range": "± 6.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.94,
            "range": "± 4.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 273.65,
            "range": "± 7.87",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 87,
            "range": "± 1.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 39.95,
            "range": "± 0.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 58.24,
            "range": "± 2.77",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 97.54,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 105.08,
            "range": "± 6.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 90.23,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 88.36,
            "range": "± 5.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 88.37,
            "range": "± 38.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 83.99,
            "range": "± 8.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.66,
            "range": "± 7.18",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 87.88,
            "range": "± 1.34",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 88.26,
            "range": "± 1.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88.13,
            "range": "± 1.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 42.03,
            "range": "± 0.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 102.1,
            "range": "± 8.28",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 102.54,
            "range": "± 3.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 129.01,
            "range": "± 10.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 88.92,
            "range": "± 1.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.28,
            "range": "± 0.98",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.68,
            "range": "± 0.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 42.9,
            "range": "± 1.34",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 124.4,
            "range": "± 6.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 89.81,
            "range": "± 2.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 88.71,
            "range": "± 1.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 45.53,
            "range": "± 3.42",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 43.1,
            "range": "± 1.19",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 42.1,
            "range": "± 5.28",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 45.26,
            "range": "± 0.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 109.38,
            "range": "± 7.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 102.82,
            "range": "± 1.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 90.4,
            "range": "± 1.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 42.45,
            "range": "± 0.59",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 90.17,
            "range": "± 2.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 89.5,
            "range": "± 2.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 89.24,
            "range": "± 1.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 105.3,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 45.35,
            "range": "± 5.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 280.58,
            "range": "± 4.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 89.87,
            "range": "± 2.06",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 43.02,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.7,
            "range": "± 3.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 102.29,
            "range": "± 2.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 104.75,
            "range": "± 11.48",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 95.92,
            "range": "± 4.81",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 90.9,
            "range": "± 0.98",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 88.78,
            "range": "± 11.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 89.6,
            "range": "± 1.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 89.35,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 88.66,
            "range": "± 2.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 90.38,
            "range": "± 2.72",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "18aae28b1b8201604772b602fc0b931fce1f1333",
          "message": "CI run benchmarks, record results, alert if threshold is exceeded",
          "timestamp": "2024-06-13T07:16:00+08:00",
          "tree_id": "da6fd735bdbce42a232890e3548ecfd3c9763ea0",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/18aae28b1b8201604772b602fc0b931fce1f1333"
        },
        "date": 1718234500698,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 216.48,
            "range": "± 2.64",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 199.77,
            "range": "± 2.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 221.16,
            "range": "± 52.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 219.65,
            "range": "± 11.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.47,
            "range": "± 0.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 125.54,
            "range": "± 16.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 244.57,
            "range": "± 21.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 260.65,
            "range": "± 7.66",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 83.7,
            "range": "± 1.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.66,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 97.47,
            "range": "± 7.45",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 98.05,
            "range": "± 5.17",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 128.1,
            "range": "± 12.08",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 88.54,
            "range": "± 1.69",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 39.87,
            "range": "± 1.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 39.87,
            "range": "± 0.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 39.84,
            "range": "± 0.82",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 121.21,
            "range": "± 14.12",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 88.06,
            "range": "± 0.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 87.94,
            "range": "± 0.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.71,
            "range": "± 3.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.13,
            "range": "± 0.98",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.61,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 41.5,
            "range": "± 0.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 100.41,
            "range": "± 1.71",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 101.77,
            "range": "± 5.82",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 88.17,
            "range": "± 1.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 39.61,
            "range": "± 0.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 88.04,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 83.93,
            "range": "± 1.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 83.76,
            "range": "± 2.40",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 102.61,
            "range": "± 5.93",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.44,
            "range": "± 3.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 273.04,
            "range": "± 5.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 87.96,
            "range": "± 1.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 39.92,
            "range": "± 0.48",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 58.04,
            "range": "± 2.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 97.32,
            "range": "± 5.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 105.32,
            "range": "± 6.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 90.2,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 88.41,
            "range": "± 1.15",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 88.07,
            "range": "± 6.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 84.11,
            "range": "± 1.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.6,
            "range": "± 5.34",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 88.05,
            "range": "± 1.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 88.13,
            "range": "± 1.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 87.98,
            "range": "± 0.79",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 45.43,
            "range": "± 3.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 102.01,
            "range": "± 7.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 104.33,
            "range": "± 1.19",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 128.56,
            "range": "± 10.48",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 88.94,
            "range": "± 3.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.37,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 46.1,
            "range": "± 1.41",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 42.78,
            "range": "± 1.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 122.52,
            "range": "± 9.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 90.18,
            "range": "± 1.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 88.89,
            "range": "± 2.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 45.95,
            "range": "± 0.55",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 46.17,
            "range": "± 2.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 42.07,
            "range": "± 0.69",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 45.98,
            "range": "± 2.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 103.87,
            "range": "± 1.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 103.6,
            "range": "± 6.66",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 90.37,
            "range": "± 2.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 46.04,
            "range": "± 1.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 90.17,
            "range": "± 1.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 89.77,
            "range": "± 1.90",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 89.2,
            "range": "± 1.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 105.69,
            "range": "± 1.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 45.55,
            "range": "± 4.48",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 281.34,
            "range": "± 6.13",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 89.63,
            "range": "± 2.28",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 46,
            "range": "± 1.26",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.54,
            "range": "± 2.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 104.71,
            "range": "± 1.82",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 104.67,
            "range": "± 15.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 98.99,
            "range": "± 1.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 90.23,
            "range": "± 2.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 89.69,
            "range": "± 3.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 89.89,
            "range": "± 2.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 89.47,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 88.2,
            "range": "± 1.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 89.97,
            "range": "± 2.96",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "58e8242e9bdea1913486f5e757c7e6b194ebcb3f",
          "message": "CI run benchmarks, record results, alert if threshold is exceeded",
          "timestamp": "2024-06-13T07:17:31+08:00",
          "tree_id": "86cf337231904b973bbad76547236dd582976eba",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/58e8242e9bdea1913486f5e757c7e6b194ebcb3f"
        },
        "date": 1718234645638,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 215.84,
            "range": "± 8.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 197.62,
            "range": "± 5.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 222.66,
            "range": "± 30.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 219.17,
            "range": "± 19.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.47,
            "range": "± 1.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 121.43,
            "range": "± 25.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 248.56,
            "range": "± 11.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 254.96,
            "range": "± 22.45",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 84.01,
            "range": "± 1.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.64,
            "range": "± 0.55",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 97.28,
            "range": "± 9.19",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 98.31,
            "range": "± 1.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 127.32,
            "range": "± 9.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 88.64,
            "range": "± 1.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.2,
            "range": "± 1.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 39.63,
            "range": "± 0.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 40.07,
            "range": "± 0.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 121.27,
            "range": "± 15.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 88.29,
            "range": "± 0.80",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 87.84,
            "range": "± 0.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.77,
            "range": "± 4.80",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.09,
            "range": "± 0.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.61,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 41.49,
            "range": "± 0.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 100.32,
            "range": "± 5.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 101.75,
            "range": "± 3.48",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 88.23,
            "range": "± 0.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 39.98,
            "range": "± 0.93",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 88.22,
            "range": "± 1.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 84.2,
            "range": "± 1.18",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 83.47,
            "range": "± 1.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 102.36,
            "range": "± 6.60",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.22,
            "range": "± 3.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 272.44,
            "range": "± 7.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 96.93,
            "range": "± 8.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 39.96,
            "range": "± 0.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 58.07,
            "range": "± 1.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 97.34,
            "range": "± 2.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 105.65,
            "range": "± 7.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 91.02,
            "range": "± 2.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 88.22,
            "range": "± 1.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 88.27,
            "range": "± 8.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 83.91,
            "range": "± 1.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.66,
            "range": "± 5.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 88.03,
            "range": "± 1.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 88.21,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88.03,
            "range": "± 1.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 42.31,
            "range": "± 1.15",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 102.1,
            "range": "± 6.34",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 103.48,
            "range": "± 4.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 129.9,
            "range": "± 13.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 89.4,
            "range": "± 2.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.13,
            "range": "± 1.13",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.45,
            "range": "± 1.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 42.74,
            "range": "± 1.69",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 126.01,
            "range": "± 19.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 90.83,
            "range": "± 2.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 89.29,
            "range": "± 2.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 45.26,
            "range": "± 3.64",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 43.14,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 42.31,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 44.94,
            "range": "± 0.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 103.95,
            "range": "± 1.27",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 108.8,
            "range": "± 8.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 89.78,
            "range": "± 2.89",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 42.45,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 90.03,
            "range": "± 2.11",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 89.59,
            "range": "± 1.69",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 88.85,
            "range": "± 1.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 105.27,
            "range": "± 2.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 46.16,
            "range": "± 5.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 281.66,
            "range": "± 3.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 89.75,
            "range": "± 5.60",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 42.69,
            "range": "± 0.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.52,
            "range": "± 2.59",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 103.39,
            "range": "± 1.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 104.35,
            "range": "± 1.27",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 96.89,
            "range": "± 10.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 90.21,
            "range": "± 2.64",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 88.57,
            "range": "± 2.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 90.02,
            "range": "± 3.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 89.51,
            "range": "± 1.93",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 88.31,
            "range": "± 2.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 90.55,
            "range": "± 2.73",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "c8353b195a2a5ada3c393e2723226db545f9a823",
          "message": "Rename `CounterSink` to `TestSink`",
          "timestamp": "2024-06-13T15:02:01+08:00",
          "tree_id": "71682cab3a9e493393ebebf0de3fddd8b28d102e",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/c8353b195a2a5ada3c393e2723226db545f9a823"
        },
        "date": 1718262414369,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 212.62,
            "range": "± 4.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 196.81,
            "range": "± 17.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 217.57,
            "range": "± 4.06",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 215.21,
            "range": "± 4.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.17,
            "range": "± 0.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 121.21,
            "range": "± 18.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 248.59,
            "range": "± 37.28",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 270.14,
            "range": "± 27.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 84.39,
            "range": "± 10.84",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.68,
            "range": "± 0.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 96.91,
            "range": "± 8.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 96.86,
            "range": "± 8.41",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 127.25,
            "range": "± 9.84",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 87.65,
            "range": "± 0.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.08,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 40.02,
            "range": "± 0.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 40.11,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 121.25,
            "range": "± 21.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 87.67,
            "range": "± 1.60",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 87.65,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.84,
            "range": "± 3.80",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.13,
            "range": "± 1.79",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.91,
            "range": "± 4.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 41.5,
            "range": "± 0.82",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 100.03,
            "range": "± 17.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 99.09,
            "range": "± 2.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 87.46,
            "range": "± 0.87",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 39.86,
            "range": "± 0.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 87.58,
            "range": "± 1.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 83.57,
            "range": "± 1.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 83,
            "range": "± 1.24",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 104.88,
            "range": "± 8.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.09,
            "range": "± 4.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 283.02,
            "range": "± 3.90",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 87.57,
            "range": "± 0.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 39.98,
            "range": "± 0.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 59.68,
            "range": "± 1.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 97.11,
            "range": "± 5.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 104.21,
            "range": "± 8.45",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 89.34,
            "range": "± 1.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 87.72,
            "range": "± 5.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 87.86,
            "range": "± 7.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 83.51,
            "range": "± 1.06",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.28,
            "range": "± 6.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 87.7,
            "range": "± 0.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 87.52,
            "range": "± 0.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88.01,
            "range": "± 1.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 41.87,
            "range": "± 0.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 102.96,
            "range": "± 3.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 102.45,
            "range": "± 6.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 127.85,
            "range": "± 14.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 88.39,
            "range": "± 2.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 44.72,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.74,
            "range": "± 1.59",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 43.84,
            "range": "± 1.33",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 128.57,
            "range": "± 15.17",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 89.69,
            "range": "± 2.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 88.86,
            "range": "± 3.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 45.18,
            "range": "± 3.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 43.45,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 43.49,
            "range": "± 1.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 44.55,
            "range": "± 1.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 102.93,
            "range": "± 1.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 101.89,
            "range": "± 1.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 89.45,
            "range": "± 3.33",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 42.42,
            "range": "± 1.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 88.96,
            "range": "± 3.18",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 88.9,
            "range": "± 1.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 89.75,
            "range": "± 1.34",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 106.41,
            "range": "± 4.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 45.4,
            "range": "± 3.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 288.55,
            "range": "± 4.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 88.33,
            "range": "± 2.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 43.14,
            "range": "± 1.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.73,
            "range": "± 2.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 102.7,
            "range": "± 5.68",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 105.26,
            "range": "± 26.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 96.84,
            "range": "± 12.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 89.75,
            "range": "± 4.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 88.92,
            "range": "± 9.28",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 89.62,
            "range": "± 1.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 88.56,
            "range": "± 1.15",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 87.66,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 89.36,
            "range": "± 3.31",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "d471845cb0afa34005b23c7bece6ebc6b9c1a193",
          "message": "Box `RecordOwned` in enum `SendToChannelErrorDropped`",
          "timestamp": "2024-06-13T15:16:56+08:00",
          "tree_id": "5ad303b8978050f5028c5fd32d2d25a1ba55e8c2",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/d471845cb0afa34005b23c7bece6ebc6b9c1a193"
        },
        "date": 1718263381788,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 213.8,
            "range": "± 20.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 192.41,
            "range": "± 11.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 218.98,
            "range": "± 22.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 217.29,
            "range": "± 12.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.16,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 121.76,
            "range": "± 4.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 253.22,
            "range": "± 21.55",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 262.02,
            "range": "± 31.59",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 82.8,
            "range": "± 2.41",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.94,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 97.86,
            "range": "± 41.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 96.94,
            "range": "± 9.26",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 128.74,
            "range": "± 14.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 87,
            "range": "± 0.82",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.37,
            "range": "± 0.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 40.29,
            "range": "± 0.66",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 40.39,
            "range": "± 0.33",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 122.58,
            "range": "± 3.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 87.08,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 86.98,
            "range": "± 1.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 42.45,
            "range": "± 4.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.38,
            "range": "± 1.19",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.88,
            "range": "± 16.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 42.02,
            "range": "± 1.89",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 107.33,
            "range": "± 4.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 108.37,
            "range": "± 1.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 86.9,
            "range": "± 1.19",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 40.4,
            "range": "± 1.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 86.95,
            "range": "± 1.13",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 85.23,
            "range": "± 3.89",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 84.06,
            "range": "± 3.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 104.45,
            "range": "± 7.41",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 42.36,
            "range": "± 4.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 286.86,
            "range": "± 20.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 87.09,
            "range": "± 0.71",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 40.74,
            "range": "± 0.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 58.26,
            "range": "± 4.39",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 97.07,
            "range": "± 2.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 104.52,
            "range": "± 7.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 89.44,
            "range": "± 2.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 87.08,
            "range": "± 1.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 87.02,
            "range": "± 5.77",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 84.01,
            "range": "± 1.98",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 82.73,
            "range": "± 5.15",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 86.94,
            "range": "± 0.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 87.03,
            "range": "± 1.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88.43,
            "range": "± 2.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 41.64,
            "range": "± 0.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 102.26,
            "range": "± 7.28",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 102.32,
            "range": "± 7.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 130.83,
            "range": "± 8.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 87.5,
            "range": "± 2.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.39,
            "range": "± 0.45",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.74,
            "range": "± 0.66",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 42.22,
            "range": "± 0.89",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 127.89,
            "range": "± 7.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 87.05,
            "range": "± 1.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 87.07,
            "range": "± 1.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 44.43,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 42.53,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 41.62,
            "range": "± 0.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 44.99,
            "range": "± 0.90",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 108.45,
            "range": "± 4.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 108.1,
            "range": "± 4.31",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 86.97,
            "range": "± 1.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 42.89,
            "range": "± 0.69",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 86.94,
            "range": "± 0.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 89.38,
            "range": "± 3.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 88.24,
            "range": "± 1.81",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 105.88,
            "range": "± 2.48",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 44.98,
            "range": "± 2.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 286.91,
            "range": "± 7.12",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 87.06,
            "range": "± 1.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 42.61,
            "range": "± 0.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.54,
            "range": "± 3.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 102.15,
            "range": "± 4.46",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 105.15,
            "range": "± 7.87",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 95.52,
            "range": "± 1.71",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 86.91,
            "range": "± 1.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 86.79,
            "range": "± 4.92",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 89.35,
            "range": "± 1.55",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 89.46,
            "range": "± 2.28",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 86.92,
            "range": "± 0.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 86.91,
            "range": "± 1.20",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "8ac1eca330b969c274caaf52c28868bcb7b20c9b",
          "message": "Make method `Record:tid` public",
          "timestamp": "2024-06-13T15:18:36+08:00",
          "tree_id": "cb8d936eddd12d3ca06977e02af017661eab1bfa",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/8ac1eca330b969c274caaf52c28868bcb7b20c9b"
        },
        "date": 1718263550424,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 215.84,
            "range": "± 3.12",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 199.48,
            "range": "± 4.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 220.24,
            "range": "± 2.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 219.32,
            "range": "± 2.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.16,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 123.35,
            "range": "± 9.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 256.11,
            "range": "± 12.69",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 281.37,
            "range": "± 25.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 82.71,
            "range": "± 1.92",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.92,
            "range": "± 1.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 108.31,
            "range": "± 26.64",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 97.42,
            "range": "± 1.34",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 128.11,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 87.07,
            "range": "± 0.93",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.48,
            "range": "± 1.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 40.23,
            "range": "± 0.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 40.37,
            "range": "± 0.55",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 122.32,
            "range": "± 1.90",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 87.14,
            "range": "± 0.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 86.9,
            "range": "± 0.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 42.58,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.29,
            "range": "± 0.60",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.76,
            "range": "± 1.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 42.69,
            "range": "± 1.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 106.67,
            "range": "± 3.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 107.16,
            "range": "± 4.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 86.99,
            "range": "± 1.17",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 40.35,
            "range": "± 0.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 95.38,
            "range": "± 7.27",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 85.45,
            "range": "± 1.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 84.12,
            "range": "± 3.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 103.95,
            "range": "± 2.08",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.88,
            "range": "± 3.19",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 287.58,
            "range": "± 5.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 87.2,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 40.75,
            "range": "± 0.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 57.7,
            "range": "± 1.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 115.6,
            "range": "± 21.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 104.98,
            "range": "± 3.33",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 120.24,
            "range": "± 32.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 87.03,
            "range": "± 3.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 86.77,
            "range": "± 1.60",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 83.7,
            "range": "± 2.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.28,
            "range": "± 1.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 91.42,
            "range": "± 6.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 97.52,
            "range": "± 11.27",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88.34,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 41.62,
            "range": "± 0.64",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 108.19,
            "range": "± 9.79",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 108.22,
            "range": "± 3.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 133.91,
            "range": "± 10.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 90.29,
            "range": "± 3.19",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.41,
            "range": "± 0.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.67,
            "range": "± 1.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 42.22,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 133.55,
            "range": "± 6.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 86.89,
            "range": "± 1.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 87.2,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 44.67,
            "range": "± 2.80",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 42.4,
            "range": "± 0.79",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 41.62,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 45.07,
            "range": "± 1.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 121.16,
            "range": "± 4.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 120.63,
            "range": "± 2.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 86.97,
            "range": "± 11.78",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 43.28,
            "range": "± 1.55",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 87.08,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 92.05,
            "range": "± 3.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 88.58,
            "range": "± 2.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 120.09,
            "range": "± 9.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 44.78,
            "range": "± 0.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 283.35,
            "range": "± 4.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 86.62,
            "range": "± 1.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 42.52,
            "range": "± 0.81",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.43,
            "range": "± 1.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 105.22,
            "range": "± 5.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 105.11,
            "range": "± 6.26",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 96.8,
            "range": "± 3.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 87.13,
            "range": "± 4.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 87.16,
            "range": "± 5.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 88.65,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 91,
            "range": "± 5.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 86.97,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 86.72,
            "range": "± 1.69",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "124a6fabe33796588515a830b1a81c3f1fc3283c",
          "message": "Move module `log_crate` to `re_export::log`",
          "timestamp": "2024-06-13T16:03:05+08:00",
          "tree_id": "3fccd37dacd5895cb56b01feed1bbe5193f79639",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/124a6fabe33796588515a830b1a81c3f1fc3283c"
        },
        "date": 1718266167230,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 214.98,
            "range": "± 2.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 195.91,
            "range": "± 3.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 219.76,
            "range": "± 2.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 219.02,
            "range": "± 4.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.16,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 120.49,
            "range": "± 4.12",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 252.58,
            "range": "± 24.58",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 262.84,
            "range": "± 21.74",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 82.56,
            "range": "± 1.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.64,
            "range": "± 0.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 97.05,
            "range": "± 2.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 96.8,
            "range": "± 7.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 126.43,
            "range": "± 10.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 86.99,
            "range": "± 0.79",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.38,
            "range": "± 1.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 40.27,
            "range": "± 0.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 40.23,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 120.42,
            "range": "± 16.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 87.07,
            "range": "± 0.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 86.85,
            "range": "± 0.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.42,
            "range": "± 3.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.38,
            "range": "± 0.43",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.66,
            "range": "± 0.33",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 40.89,
            "range": "± 1.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 108.88,
            "range": "± 6.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 107.45,
            "range": "± 6.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 87.09,
            "range": "± 0.64",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 40.24,
            "range": "± 0.42",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 86.97,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 84.02,
            "range": "± 1.69",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 82.36,
            "range": "± 1.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 104.59,
            "range": "± 7.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.2,
            "range": "± 3.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 287.03,
            "range": "± 20.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 87.11,
            "range": "± 0.45",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 40.58,
            "range": "± 1.27",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 58.33,
            "range": "± 0.80",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 96.63,
            "range": "± 5.77",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 103.85,
            "range": "± 3.31",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 89.09,
            "range": "± 1.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 87.05,
            "range": "± 5.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 87.04,
            "range": "± 7.12",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 83.81,
            "range": "± 1.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.17,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 87.02,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 87.07,
            "range": "± 0.68",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88.8,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 41.62,
            "range": "± 0.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 102.16,
            "range": "± 6.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 102.13,
            "range": "± 1.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 127.44,
            "range": "± 9.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 86.92,
            "range": "± 1.83",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.44,
            "range": "± 0.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.64,
            "range": "± 0.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 42.6,
            "range": "± 1.16",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 127.33,
            "range": "± 5.03",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 86.73,
            "range": "± 0.96",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 86.91,
            "range": "± 1.72",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 44.68,
            "range": "± 2.92",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 42.85,
            "range": "± 0.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 41.56,
            "range": "± 0.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 44.86,
            "range": "± 1.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 108.03,
            "range": "± 2.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 107.87,
            "range": "± 2.91",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 86.81,
            "range": "± 0.70",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 43.08,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 87,
            "range": "± 1.17",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 89.2,
            "range": "± 3.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 88.34,
            "range": "± 1.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 105.92,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 44.49,
            "range": "± 3.09",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 282.74,
            "range": "± 6.98",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 87.05,
            "range": "± 2.02",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 42.44,
            "range": "± 0.66",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.29,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 102.22,
            "range": "± 5.95",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 104.89,
            "range": "± 8.05",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 94.97,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 87.28,
            "range": "± 4.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 86.92,
            "range": "± 8.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 89.26,
            "range": "± 1.57",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 88.14,
            "range": "± 4.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 86.85,
            "range": "± 1.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 86.71,
            "range": "± 0.46",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "committer": {
            "email": "SpriteOvO@gmail.com",
            "name": "Asuna",
            "username": "SpriteOvO"
          },
          "distinct": true,
          "id": "ae7cbd40ec53a00797b04684eaae32281247b1c3",
          "message": "Rename `LevelFilter::compare` to `LevelFilter::test`",
          "timestamp": "2024-06-13T17:29:03+08:00",
          "tree_id": "ceea47538151ede1783ac5ac79ad748464da54ae",
          "url": "https://github.com/SpriteOvO/spdlog-rs/commit/ae7cbd40ec53a00797b04684eaae32281247b1c3"
        },
        "date": 1718271287181,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_1_file",
            "value": 217.63,
            "range": "± 4.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_file_async",
            "value": 195.74,
            "range": "± 2.34",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_rotating_file_size",
            "value": 222.04,
            "range": "± 3.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_rotating_daily",
            "value": 221.05,
            "range": "± 4.67",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_level_off",
            "value": 2.16,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_1_full_formatter",
            "value": 122.66,
            "range": "± 3.10",
            "unit": "ns/iter"
          },
          {
            "name": "bench_2_full_pattern_ct",
            "value": 254.18,
            "range": "± 16.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_3_full_pattern_rt",
            "value": 264.39,
            "range": "± 19.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_am_pm",
            "value": 82.91,
            "range": "± 1.27",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_column",
            "value": 39.6,
            "range": "± 0.40",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date",
            "value": 97.67,
            "range": "± 8.54",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_date_short",
            "value": 96.99,
            "range": "± 5.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_datetime",
            "value": 127.28,
            "range": "± 15.85",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_day",
            "value": 86.99,
            "range": "± 1.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_eol",
            "value": 40.37,
            "range": "± 6.93",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file",
            "value": 40.26,
            "range": "± 0.75",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_file_name",
            "value": 40.25,
            "range": "± 0.55",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_full",
            "value": 124.14,
            "range": "± 3.38",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour",
            "value": 87.06,
            "range": "± 0.56",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_hour_12",
            "value": 86.82,
            "range": "± 1.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level",
            "value": 41.06,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_level_short",
            "value": 40.42,
            "range": "± 0.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_line",
            "value": 39.62,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_logger",
            "value": 40.83,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_microsecond",
            "value": 106.93,
            "range": "± 14.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_millsecond",
            "value": 108.35,
            "range": "± 3.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_minute",
            "value": 86.96,
            "range": "± 0.98",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_module_path",
            "value": 40.21,
            "range": "± 0.26",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month",
            "value": 86.9,
            "range": "± 0.71",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name",
            "value": 84.03,
            "range": "± 1.20",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_month_name_full",
            "value": 82.83,
            "range": "± 1.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_nanosecond",
            "value": 110.59,
            "range": "± 10.27",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_payload",
            "value": 41.22,
            "range": "± 5.49",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_pid",
            "value": 292.13,
            "range": "± 6.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_second",
            "value": 87,
            "range": "± 0.64",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_source",
            "value": 40.52,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tid",
            "value": 58.18,
            "range": "± 1.32",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time",
            "value": 96.71,
            "range": "± 6.35",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_12",
            "value": 104.34,
            "range": "± 8.53",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_time_short",
            "value": 89.61,
            "range": "± 1.80",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_tz_offset",
            "value": 87.08,
            "range": "± 1.48",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_unix_timestamp",
            "value": 86.87,
            "range": "± 6.31",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name",
            "value": 83.86,
            "range": "± 1.44",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_weekday_name_full",
            "value": 83.3,
            "range": "± 2.99",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year",
            "value": 85.67,
            "range": "± 1.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_4_ct_year_short",
            "value": 87.08,
            "range": "± 1.84",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_am_pm",
            "value": 88,
            "range": "± 1.23",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_column",
            "value": 41.87,
            "range": "± 1.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date",
            "value": 101.96,
            "range": "± 4.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_date_short",
            "value": 102.06,
            "range": "± 6.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_datetime",
            "value": 128.46,
            "range": "± 11.40",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_day",
            "value": 86.72,
            "range": "± 1.47",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_eol",
            "value": 42.43,
            "range": "± 1.13",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file",
            "value": 42.19,
            "range": "± 0.51",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_file_name",
            "value": 42.55,
            "range": "± 0.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_full",
            "value": 126.1,
            "range": "± 17.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour",
            "value": 88.6,
            "range": "± 2.40",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_hour_12",
            "value": 87.45,
            "range": "± 3.62",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level",
            "value": 45.14,
            "range": "± 1.63",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_level_short",
            "value": 42.99,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_line",
            "value": 42.12,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_logger",
            "value": 43.94,
            "range": "± 0.88",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_microsecond",
            "value": 109.16,
            "range": "± 3.89",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_millsecond",
            "value": 108.22,
            "range": "± 1.86",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_minute",
            "value": 89.14,
            "range": "± 5.61",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_module_path",
            "value": 42.09,
            "range": "± 0.76",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month",
            "value": 87.01,
            "range": "± 2.01",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name",
            "value": 89.46,
            "range": "± 1.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_month_name_full",
            "value": 88.28,
            "range": "± 1.65",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_nanosecond",
            "value": 106.35,
            "range": "± 6.06",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_payload",
            "value": 44.52,
            "range": "± 4.29",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_pid",
            "value": 284.26,
            "range": "± 3.30",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_second",
            "value": 86.72,
            "range": "± 2.08",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_source",
            "value": 42.5,
            "range": "± 0.66",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tid",
            "value": 61.44,
            "range": "± 1.36",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time",
            "value": 101.76,
            "range": "± 5.79",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_12",
            "value": 105.44,
            "range": "± 10.25",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_time_short",
            "value": 95.57,
            "range": "± 2.33",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_tz_offset",
            "value": 86.99,
            "range": "± 1.00",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_unix_timestamp",
            "value": 87.12,
            "range": "± 6.50",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name",
            "value": 89.18,
            "range": "± 1.73",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_weekday_name_full",
            "value": 88.14,
            "range": "± 5.04",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year",
            "value": 86.95,
            "range": "± 3.08",
            "unit": "ns/iter"
          },
          {
            "name": "bench_5_rt_year_short",
            "value": 87.62,
            "range": "± 2.42",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}