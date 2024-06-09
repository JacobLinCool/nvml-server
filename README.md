# nvml-server

This is a simple server that uses the [NVIDIA Management Library (NVML)](https://developer.nvidia.com/nvidia-management-library-nvml) to expose GPU metrics over a REST API.

The pre-built binary is available in the [releases](https://github.com/JacobLinCool/nvml-server/releases) page.

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

## Responses

The JSON response will be like the following:

```json
{
    "gpus": [
        {
            "uuid": "GPU-ba25eb9d-1a6f-ff8f-8ae8-a0b6a5ba04a2",
            "name": "NVIDIA GeForce GTX 1080 Ti",
            "power": 107238,
            "memory": { "free": 923140096, "total": 11811160064, "used": 10888019968 },
            "utilization": { "gpu": 26, "memory": 19 }
        },
        {
            "uuid": "GPU-cd974359-b75a-82b0-cdbc-1e1352743ea3",
            "name": "NVIDIA GeForce GTX 1080 Ti",
            "power": 214884,
            "memory": { "free": 770965504, "total": 11811160064, "used": 11040194560 },
            "utilization": { "gpu": 95, "memory": 44 }
        }
    ],
    "processes": [
        {
            "pid": 2049806,
            "name": "python",
            "user": "alice",
            "run_time": 4424,
            "cpu_usage": 0.46462402,
            "gpus": [{ "uuid": "GPU-ba25eb9d-1a6f-ff8f-8ae8-a0b6a5ba04a2", "memory": 8069840896 }]
        },
        {
            "pid": 2053760,
            "name": "ollama_llama_se",
            "user": "alice",
            "run_time": 33,
            "cpu_usage": 26.731369,
            "gpus": [
                { "uuid": "GPU-ba25eb9d-1a6f-ff8f-8ae8-a0b6a5ba04a2", "memory": 2701131776 },
                { "uuid": "GPU-cd974359-b75a-82b0-cdbc-1e1352743ea3", "memory": 2973761536 }
            ]
        },
        {
            "pid": 1326474,
            "name": "python",
            "user": "bob",
            "run_time": 104886,
            "cpu_usage": 99.2437,
            "gpus": [{ "uuid": "GPU-cd974359-b75a-82b0-cdbc-1e1352743ea3", "memory": 5333057536 }]
        },
        {
            "pid": 1636239,
            "name": "python",
            "user": "alice",
            "run_time": 69947,
            "cpu_usage": 0.30974934,
            "gpus": [{ "uuid": "GPU-cd974359-b75a-82b0-cdbc-1e1352743ea3", "memory": 2627731456 }]
        }
    ]
}
```
