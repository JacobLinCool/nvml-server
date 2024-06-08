# nvml-server

This is a simple server that uses the [NVIDIA Management Library (NVML)](https://developer.nvidia.com/nvidia-management-library-nvml) to expose GPU metrics over a REST API.

## Building

```bash
cargo build --release
```

`target/release/nvml-server` will be the binary.

## Running

```bash
./target/release/nvml-server
```

`PORT` environment variable can be used to specify the port to listen on. Default is `21005`.
