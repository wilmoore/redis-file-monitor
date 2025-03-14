# redis-file-monitor ![License](https://img.shields.io/github/license/wilmoore/redis-file-monitor?v=1) ![Issues](https://img.shields.io/github/issues/wilmoore/redis-file-monitor)
> A CLI To Continuously Monitor the (CWD) for new and updated `*.redis` files to pipe to `| redis-cli`

### Usage

```sh
redis-file-monitor \
   --log-level debug \
   --redis-cli-path /usr/local/bin/redis-cli \
   --watch-dir /var/redis/scripts
```

###### Options

| Flag | Description | Default |
|------|-------------|---------|
| `--watch-dir` | Directory to monitor for `.redis` files | Current working directory (CWD) |
| `--redis-cli-path` | Path to `redis-cli` binary | `redis-cli` (assumes in `PATH`) |
| `--log-level` | Logging level (`info`, `debug`, `error`) | `info` |

### Features

- **Automatic Execution**: Detects new `.redis` files in `CWD` and immediately pipes them to `redis-cli`.
- **Lightweight**: Built using Rust for high performance and minimal system resource usage.
- **Cross-Platform**: Works on Linux, macOS, and Windows (via WSL or an appropriate shell environment).
- **Configurable**: Environment variables allow customization of the Redis CLI path.

### Installation

###### **From Source**
```sh
git clone https://github.com/wilmoore/redis-file-monitor.git
cd redis-file-monitor
make
```

### Contributing

We welcome contributions! Please submit pull requests and report issues via [GitHub Issues](https://github.com/wilmoore/redis-file-monitor/issues).

###### Internals

- Uses `notify` to monitor file system changes.
- Uses `tokio` for async event handling.
- Gracefully handles empty files to prevent unnecessary execution.
- Uses a channel (`tokio::sync::mpsc::channel`) to process events efficiently.
- Executes `.redis` files using a shell command (`sh -c "cat filename | redis-cli"`).

###### Development

To contribute or modify the project, clone the repository and set up your environment:

```sh
git clone https://github.com/wilmoore/redis-file-monitor.git
cd redis-file-monitor
make
```