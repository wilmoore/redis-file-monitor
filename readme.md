![GitHub release](https://img.shields.io/github/v/release/wilmoore/redis-file-monitor)
![Build Status](https://img.shields.io/github/actions/workflow/status/wilmoore/redis-file-monitor/ci.yml)
![License](https://img.shields.io/github/license/wilmoore/redis-file-monitor)
![Issues](https://img.shields.io/github/issues/wilmoore/redis-file-monitor)

# redis-file-monitor

**redis-file-monitor** is a lightweight CLI tool that continuously monitors the current working directory (CWD) for new `.redis` files and automatically pipes their contents to `redis-cli`. This enables seamless execution of Redis commands as soon as new `.redis` script files are created.

## Features

- **Automatic Execution**: Detects new `.redis` files and pipes them to `redis-cli` instantly.
- **Lightweight & Efficient**: Minimal system resource usage.
- **Simple & Portable**: Works on Linux, macOS, and Windows (via WSL or appropriate shell environment).

## Installation

Clone the repository and build the binary:

```sh
git clone https://github.com/wilmoore/redis-file-monitor.git
cd redis-file-monitor
make install  # or follow manual build instructions
```

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
| `WATCH_DIRECTORY` | Directory to monitor | Current directory |
| `FILE_EXTENSION` | Extension of files to monitor | `.redis` |

Example:

```sh
export REDIS_CLI_PATH=/usr/local/bin/redis-cli
export WATCH_DIRECTORY=/var/redis/scripts
redis-file-monitor
```

## Development

To contribute or modify the project, clone the repository and set up your environment:

```sh
git clone https://github.com/wilmoore/redis-file-monitor.git
cd redis-file-monitor
make build  # Compile the project
```

## Contributing

We welcome contributions! Please submit pull requests and report issues via [GitHub Issues](https://github.com/wilmoore/redis-file-monitor/issues).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Authors

Created and maintained by [Wil Moore III](https://github.com/wilmoore).
