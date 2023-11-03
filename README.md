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