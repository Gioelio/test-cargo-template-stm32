# {{project-name}}

A Rust embedded project for STM32 microcontrollers using {% if framework == "stm32rs" %}STM32 HAL{% else %}Embassy{% endif %}.

## Hardware Target

- **MCU**: {{mcu | upper}}
- **Framework**: {{framework}}

## Getting Started

### Prerequisites

1. Install Rust with the embedded target:
```bash
rustup target add thumbv7em-none-eabihf
```

2. Install probe-rs for flashing and debugging:
```bash
cargo install probe-rs-tools --locked
```

### Building

```bash
cargo build --release
```

### Flashing

Connect your ST-Link debugger and run:
```bash
cargo run --release
```

## Project Structure

{% if framework == "stm32rs" -%}
This project uses the STM32 HAL (Hardware Abstraction Layer) which provides:
- Direct register access with type safety
- Blocking APIs with explicit error handling
- Fine-grained control over peripherals
- Lower memory overhead

### Key Files

- `src/main.rs` - Main application code
- `memory.x` - Linker script defining memory layout
- `.cargo/config.toml` - Cargo configuration for embedded target

{% else -%}
This project uses Embassy, an async embedded framework that provides:
- Async/await support for embedded programming
- Built-in drivers for many peripherals  
- Efficient task scheduling
- Modern Rust patterns

### Key Files

- `src/main.rs` - Main application code with async main
- `defmt.toml` - Logging configuration
- `.cargo/config.toml` - Cargo configuration for embedded target

### Logging

This project uses `defmt` for efficient logging. Logs will appear in your probe-rs terminal when running the application.

{% endif -%}

## Development

### Debugging

You can use probe-rs for debugging:
```bash
probe-rs debug --chip {{mcu}}
```

Or use GDB with your preferred setup.

### Memory Usage

Check memory usage with:
```bash
cargo size --release
```

## License

This project is licensed under your preferred license.
