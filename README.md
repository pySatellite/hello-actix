# Hello Actix
- https://actix.rs/docs/getting-started


- https://hello-actix.fly.dev
<img width="709" alt="image" src="https://github.com/pySatellite/hello-actix/assets/87309910/564b4f5c-2ea3-4b32-9938-4042ca3070d2">

# API TEST
```bash
$ curl https://hello-actix.fly.dev
Hello world!%

$ curl https://hello-actix.fly.dev/hey
Hey there!%

$ curl https://hello-actix.fly.dev/healthcheck
Health Check Completed!%

$ curl -X POST https://hello-actix.fly.dev/echo -d "yaho"
yaho%

$ curl -X POST https://hello-actix.fly.dev/echo -d "{"name": "yaho", "echo": "Hurray"}"
{name: yaho, echo: Hurray}%

$ date
2023년 11월  3일 금요일 12시 36분 44초 KST
```

### Deloy
```bash
$ fly deploy
```

### Run
```bash
$ cargo run
   Compiling hello-actix v0.1.0 (/Users/m2/code/study/rust/hello-actix)
    Finished dev [unoptimized + debuginfo] target(s) in 1.76s
     Running `target/debug/hello-actix`
Rust Actix-web server started at 127.0.0.1:8080
```

### Run Auto-Reloading
- [Auto-Reloading Development Server](https://actix.rs/docs/autoreload/)
```bash
$ cargo watch -x run
[Running 'cargo run']
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/hello-actix`
Rust Actix-web server started at 127.0.0.1:8080
[Running 'cargo run']
   Compiling hello-actix v0.2.1 (/Users/m2/code/study/rust/hello-actix)
    Finished dev [unoptimized + debuginfo] target(s) in 1.66s
     Running `target/debug/hello-actix`
Rust Actix-web server started at 127.0.0.1:8080
```

### Docker Build & Tag & Push
```bash
$ docker build -t pysatellite/hello-actix:0.2.1 .

$ docker tag pysatellite/hello-actix:0.2.1 pysatellite/hello-actix

$ docker push pysatellite/hello-actix

```

### Docker Run
```bash
$ docker run -dit --rm --name hello-actix -p 8021:8080 pysatellite/hello-actix:0.2.1
```

### ref
- [How can a Rust program access metadata from its Cargo package?](https://stackoverflow.com/questions/27840394/how-can-a-rust-program-access-metadata-from-its-cargo-package)

### Performance test
```
$ curl https://hello-actix.fly.dev/iambest/1000000000
duration: 782ns, limit: 1000000000, sum: 499999999500000000

$ docker run -dit --rm --name hello-actix -p 8031:8080 --cpus=".01" --memory 6m pysatellite/hello-actix:0.3.1

$ curl http://localhost:8031/iambest/1
duration: 542ns, limit: 1, sum: 0%
$ curl http://localhost:8031/iambest/10
duration: 666ns, limit: 10, sum: 45%
$ curl http://localhost:8031/iambest/100
duration: 625ns, limit: 100, sum: 4950%
$ curl http://localhost:8031/iambest/1000
duration: 666ns, limit: 1000, sum: 499500%
$ curl http://localhost:8031/iambest/10000
duration: 625ns, limit: 10000, sum: 49995000%
$ curl http://localhost:8031/iambest/100000
duration: 625ns, limit: 100000, sum: 4999950000%
$ curl http://localhost:8031/iambest/1000000
duration: 667ns, limit: 1000000, sum: 499999500000%
$ curl http://localhost:8031/iambest/10000000
duration: 792ns, limit: 10000000, sum: 49999995000000%
$ curl http://localhost:8031/iambest/100000000
duration: 791ns, limit: 100000000, sum: 4999999950000000%
$ curl http://localhost:8031/iambest/1000000000
duration: 625ns, limit: 1000000000, sum: 499999999500000000%
$ curl http://localhost:8031/iambest/10000000000
duration: 708ns, limit: 10000000000, sum: 13106511847580896768%
$ curl http://localhost:8031/iambest/100000000000
duration: 667ns, limit: 100000000000, sum: 932355974711512064%
$ curl http://localhost:8031/iambest/1000000000000
duration: 667ns, limit: 1000000000000, sum: 1001881602603448320%
$ curl http://localhost:8031/iambest/10000000000000
duration: 792ns, limit: 10000000000000, sum: 7954484891797073920%
$ curl http://localhost:8031/iambest/10000000000000
duration: 875ns, limit: 10000000000000, sum: 7954484891797073920%
$ curl http://localhost:8031/iambest/100000000000000
duration: 750ns, limit: 100000000000000, sum: 2238944010196672512%
$ curl http://localhost:8031/iambest/1000000000000000
duration: 583ns, limit: 1000000000000000, sum: 2537972135152631808%
$ curl http://localhost:8031/iambest/10000000000000000
duration: 1µs, limit: 10000000000000000, sum: 14034540557039009792%
$ curl http://localhost:8031/iambest/100000000000000000
duration: 791ns, limit: 100000000000000000, sum: 1951506101975056384%
$ curl http://localhost:8031/iambest/1000000000000000000
duration: 666ns, limit: 1000000000000000000, sum: 15183169460410122240%
$ curl http://localhost:8031/iambest/10000000000000000000
duration: 708ns, limit: 10000000000000000000, sum: 13790443849409888256%
$ curl http://localhost:8031/iambest/100000000000000000000
curl: (52) Empty reply from server
```

### vs FastApi
```bash
$ curl https://dj-api.fly.dev/iambest/100000000
{"duration":6.793297529220581,"limit":100000000,"sum":4999999950000000}%
$ curl https://dj-api.fly.dev/iambest/10000000
{"duration":0.9222512245178223,"limit":10000000,"sum":49999995000000}%
$ curl https://dj-api.fly.dev/iambest/1000000
{"duration":0.06862711906433105,"limit":1000000,"sum":499999500000}%
$ curl https://dj-api.fly.dev/iambest/100000
{"duration":0.012793540954589844,"limit":100000,"sum":4999950000}%
$ curl https://dj-api.fly.dev/iambest/10000
{"duration":0.0005216598510742188,"limit":10000,"sum":49995000}%
$ curl https://dj-api.fly.dev/iambest/1000
{"duration":4.38690185546875e-05,"limit":1000,"sum":499500}%
```