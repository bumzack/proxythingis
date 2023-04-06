# Errors warp-proxy-v4

wenn man in Zeile 286 ein

```
info!("request uri {}", request.uri().to_string());
```

Statement einfügt, dann verschwindet der Fehler
println! is blocking, maybe this helps

oder

```
ulimit -n 10240
```

```
21:        0x104c30d00thread 'tokio-runtime-worker' panicked at 'Request failed: hyper::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: nodename nor servname provided, or not known" }))', warp-proxy-v4/src/main.rs: - 286:tokio54::
```

```
                               at /rustc/4781233a77e879e49cb5ce3c98d2abba6a6ade7a/library/core/src/result.rs:1790:5
  16:        0x104c80124 - <warp::filter::and_then::AndThenFuture<T,F> as core::future::future::Future>::poll::h7517232830b7c792
  17:        0x104ca3a0c - <warp::filter::or::EitherFuture<T,U> as core::future::future::Future>::poll::h7ba48a8f949bc4c8
  18:        0x104cdb574 - hyper::proto::h1::dispatch::Dispatcher<D,Bs,I,T>::poll_loop::h77c6f30d1565bcca
  19:        0x104cd636c - hyper::proto::h1::dispatch::Dispatcher<D,Bs,I,T>::poll_catch::hb3db68afaf170d0e
  20:        0x104d5b8c4 - <hyper::server::conn::upgrades::UpgradeableConnection<I,S,E> as core::future::future::Future>::poll::h2a95f211d68165e4
  21:        0x104c30d00thread 'tokio-runtime-worker' panicked at 'Request failed: hyper::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: nodename nor servname provided, or not known" }))', warp-proxy-v4/src/main.rs: - 286:tokio54::
loom::std::unsafe_cell::UnsafeCell<T>::with_mut::h3b8d88b3b171d981
  22:        0x104d45ca0 - tokio::runtime::task::core::Core<T,S>::poll::h69695ce2a2e80d30
  23:        0x104cf7d80 - tokio::runtime::task::harness::Harness<T,S>::poll::h22ab309e187e475e
  24:        0x104e9827c - tokio::runtime::scheduler::multi_thread::worker::Context::run_task::h01a589fefebd66b9

```

```
22:        0x104d45ca0 - tokio::runtime::task::core::Core<T,S>::poll::h69695ce2a2e80d30
23:        0x104cf7d80 - tokio::runtime::task::harness::Harness<T,S>::poll::h22ab309e187e475e
24:        0x104e9827c - tokio::runtime::scheduler::multi_thread::worker::Context::run_task::h01a589fefebd66b9
25:        0x104e97bf0 - tokio::runtime::scheduler::thread 'tokio-runtime-worker' panicked at 'Request failed: hyper::Error(Connect, ConnectError("tcp open error", Os { code: 24, kind: Uncategorized, message: "Too many open files" }))', warp-proxy-v4/src/main.rs:multi_thread::worker::Context::run::hafad847debaa8fd9
26:        0x104eaefd4 - tokio::macros::scoped_tls::ScopedKey<T>::set::h13213c7e4174fd28286
:54
27:        0x104e97834 - tokio::runtime::scheduler::multi_thread::worker::run::hfebb425ab1553c4f
28:        0x104ea0740 - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::hed45489f31cfdeb8
29:        0x104ea3430 - tokio::runtime::task::core::Core<T,S>::poll::h13317d3a3c9bbaaf
30:        0x104ea4154 - tokio::runtime::task::harness::Harness<T,S>::poll::h6678786e4cf47f6b
31'
```