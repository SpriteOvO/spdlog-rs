window.BENCHMARK_DATA = {
  "lastUpdate": 1718231048821,
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
      }
    ]
  }
}