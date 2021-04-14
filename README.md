此项目是学习rust语言时实现的一个高性能的小工具，目的将http请求的数据直接转存到redis数据库中

```bash
wrk -c40000 -t4 -d5 http://127.0.0.1:3000
```
```
Running 5s test @ http://127.0.0.1:3000
  4 threads and 40000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   466.25ms  163.08ms   1.39s    68.93%
    Req/Sec     1.52k     1.05k    4.07k    58.42%
  28982 requests in 5.09s, 2.40MB read
  Socket errors: connect 35912, read 0, write 0, timeout 157
Requests/sec:   5692.09
Transfer/sec:    483.61KB
```