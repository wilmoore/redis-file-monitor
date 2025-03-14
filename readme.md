# redis-file-monitor ![License](https://img.shields.io/github/license/wilmoore/redis-file-monitor?v=1) ![Issues](https://img.shields.io/github/issues/wilmoore/redis-file-monitor)
> A CLI To Continuously Monitor the (CWD) for new and updated `*.redis` files to `|` to `redis-cli`

### Usage

```sh
redis-file-monitor --log-level debug --redis-cli-path /usr/local/bin/redis-cli --watch-dir /var/redis/scripts
```

###### Options

| Flag | Description | Default |
|------|-------------|---------|
| `--watch-dir` | Directory to monitor for `.redis` files | Current working directory (CWD) |
| `--redis-cli-path` | Path to `redis-cli` binary | `redis-cli` (assumes in `PATH`) |
| `--log-level` | Logging level (`info`, `debug`, `error`) | `info` |

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