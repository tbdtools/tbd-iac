# TBD Infrastructure as Code

TBD is a modern Infrastructure as Code (IaC) tool written in Rust that allows you to define your infrastructure using Python. It combines the speed and reliability of Rust with the flexibility and ecosystem of Python.

## Features

- 🚀 Python-based infrastructure definitions
- 🔧 Multiple cloud provider support (AWS, GCP, Azure)
- 🔄 State management with SQLite
- 🔌 Plugin system for providers
- 📦 Built with Rust for performance
- 🔒 Concurrent operations support
- 🌐 gRPC communication between core and providers

## Quick Start

### Installation

```bash
# Install using cargo
cargo install tbd-iac

# Or build from source
git clone https://github.com/tbdtools/tbd-iac.git
cd tbd-iac
cargo install --path .
```

### Initialize a New Project

```bash
# Create a new project
tbd init --name my-infrastructure

# Navigate to project directory
cd my-infrastructure
```

### Project Structure

```
my-infrastructure/
├── stacks/          # Infrastructure stack definitions
│   └── main.py      # Example stack
├── modules/         # Reusable infrastructure modules
├── providers/       # Provider configurations
├── tests/           # Infrastructure tests
├── pyproject.toml   # Python dependencies and project metadata
└── .gitignore
```

### Example Stack

```python
from tbdtools import Stack, aws

class MainStack(Stack):
    def __init__(self, name: str):
        super().__init__(name)

        vpc = aws.ec2.Vpc(
            self,
            "MainVpc",
            cidr="10.0.0.0/16",
            max_azs=2,
        )

        self.output(
            "vpc_id",
            value=vpc.vpc_id,
            description="ID of the VPC"
        )

stack = MainStack("main")
```

### Usage

```bash
# Preview changes
tbd plan -s main

# Apply changes
tbd apply -s main

# Show current state
tbd show -s main

# Destroy infrastructure
tbd destroy -s main
```

## Command Reference

- `init`: Create a new TBD project
- `plan`: Preview infrastructure changes
- `apply`: Apply infrastructure changes
- `destroy`: Destroy infrastructure
- `show`: Display current state
- `provider`: Manage infrastructure providers
  - `list`: List installed providers
  - `install`: Install a provider
  - `remove`: Remove a provider

## Development

### Prerequisites

- Rust 1.75+
- Python 3.9+
- Poetry (for Python dependency management)

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details on:

- Commit message conventions
- Development workflow
- Testing requirements
- Pull request process

## License

MIT License. See [LICENSE](LICENSE) for details.

## Related Projects

- [tbd-cfg](https://github.com/tbdtools/tbd-cfg): Configuration management tooling
- [tbd-ui](https://github.com/tbdtools/tbd-ui): Web UI for TBD Tools
- [tbd-cpl](https://github.com/tbdtools/tbd-cpl): Control plane for self-hosting
- [tbd-sdk](https://github.com/tbdtools/tbd-sdk): Python SDK for infrastructure definitions

## Project Status

🚧 **Early Development** - APIs and features are subject to change.

Currently implemented:

- ✅ Basic project initialization
- ✅ CLI framework
- ✅ State management structure
- ✅ Provider interface
- ✅ Configuration system

Coming soon:

- 🔄 AWS provider implementation
- 🔄 State backend
- 🔄 Resource dependency management
- 🔄 Testing framework
- 🔄 Documentation site

## Support

- [GitHub Issues](https://github.com/tbdtools/tbd-iac/issues)
- [Project Documentation](https://github.com/tbdtools/tbd-docs)
