# consul-rust [![Checks](https://github.com/kaylendog/consul-rust/actions/workflows/check.yml/badge.svg)](https://github.com/kaylendog/consul-rust/actions/workflows/check.yml)

`consul-rust` is a library for interacting with Consul agents via their HTTP API.
Consul is a service mesh solution providing a full featured control plane
with service discovery, configuration, and segmentation functionality. 

For more information on what Consul is, read the [Consul documentation](https://www.consul.io/docs/).

## Supported Features

The key features of Consul, and thus this crate, are:

-   Service Discovery
-   Health Checking
-   KV Store
-   Secure Service Communication
-   Multi Datacenter Support

`consul-rust` aims to support all of these to the best of its ability. Each feature is available as a compiler feature, and can be enabled by using the `discovery`, `health`, `kv`, `ssc` and `mds` features respectively. By default, all features are enabled.

## Usage

The `Client` struct provides the main entry point for the library.

```rs
let config = Config::new().unwrap();
let client = Client::new(config);
```

You can pass in custom configuration by using the `Config` datatype. By
default, it will assume the Consul agent is running on localhost, on the
default port 8500.
Requests can be made to the Consul agent by importing the relevant trait:

```rs
use consul::Agent;

let client = Client::new(Config::new().unwrap());
let agents = client.agents(false).await;
```



## Installation

Simply include the consul-rust in your Cargo dependencies.

```toml
[dependencies]
consul = "0.5"
```

## Async Support

The library is designed to be fully async compatible, and works with both
the `tokio` and `async-std` runtimes. At this time, there is no blocking API
available. As an alternative, you can use versions of this library below
`0.5.0`, as these are blocking.

## License

`consul-rust` is licensed under a combined MIT/Apache-2.0 license. See the [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE) file for more information.
