# redis-file-monitor ![License](https://img.shields.io/github/license/wilmoore/redis-file-monitor?v=1) ![Issues](https://img.shields.io/github/issues/wilmoore/redis-file-monitor)
> A CLI To Continuously Monitor the (CWD) for new and updated `*.redis` files to pipe to `| redis-cli`

## Features

- **Automatic Execution**: Detects new `.redis` files in `CWD` and immediately pipes them to `redis-cli`.
- **Lightweight**: Built using Rust for high performance and minimal system resource usage.
- **Cross-Platform**: Works on Linux, macOS, and Windows (via WSL or an appropriate shell environment).
- **Configurable**: Environment variables allow customization of the Redis CLI path.

## Installation
> TBD...

## Usage

Run `redis-file-monitor` in any directory where `.redis` files may be created:

```sh
redis-file-monitor
```

Whenever a new `.redis` file appears in the directory, the tool will automatically execute:

```sh
cat filename.redis | redis-cli
```

## Example Workflow

1. Start `redis-file-monitor` in a terminal.
2. Create a `.redis` file with Redis commands:
   ```sh
   echo "SET foo bar" > example.redis
   ```
3. The command inside `example.redis` is automatically sent to `redis-cli`.
4. Check the result in Redis:
   ```sh
   redis-cli GET foo
   # Output: "bar"
   ```

## Configuration

You can customize behavior using environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `REDIS_CLI_PATH` | Path to `redis-cli` binary | `redis-cli` (assumes in `PATH`) |

Example:

```sh
export REDIS_CLI_PATH=/usr/local/bin/redis-cli
redis-file-monitor
```

## Internals

- Uses `notify` to monitor file system changes.
- Uses `tokio` for async event handling.
- Gracefully handles empty files to prevent unnecessary execution.
- Uses a channel (`tokio::sync::mpsc::channel`) to process events efficiently.
- Executes `.redis` files using a shell command (`sh -c "cat filename | redis-cli"`).

## Contributing

We welcome contributions! Please submit pull requests and report issues via [GitHub Issues](https://github.com/wilmoore/redis-file-monitor/issues).

## Development

To contribute or modify the project, clone the repository and set up your environment:

```sh
git clone https://github.com/wilmoore/redis-file-monitor.git
cd redis-file-monitor
make
```