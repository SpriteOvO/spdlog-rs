window.BENCHMARK_DATA = {
  "lastUpdate": 1718233448634,
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
      }
    ]
  }
}