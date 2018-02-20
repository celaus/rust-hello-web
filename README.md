# A Rust example of a web server


... using the [rouille](https://github.com/tomaka/rouille) framework. 

## Quick start

This is a simple program for testing purposes, it should be easy to build, run, and deploy. It's usefulness also ends there :)

```bash
$ cargo test
Compiling ... 
warning: unused variable: `request`
  --> src/main.rs:32:62
   |
32 |     rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
   |                                                              ^^^^^^^
   |
   = note: #[warn(unused_variables)] on by default
   = note: to avoid this warning, consider using `_request` instead

    Finished dev [unoptimized + debuginfo] target(s) in 3.65 secs
     Running target/debug/deps/hello_web-03b450ff35276cf9

running 2 tests
test tests::default_response_is_hello_world ... ok
test tests::port_is_8080 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```bash
$ cargo run
Compiling ... 

Now listening on 0.0.0.0:8080 
```


# License

[MIT](LICENSE)