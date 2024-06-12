# Hypridle Manager

Hypridle Manager is a tool designed to manage power profiles based on the power source (AC or battery) using the UPower framework on Linux systems.

## Getting Started

To get started with Hypridle Manager, follow these steps:

### Prerequisites

Make sure you have the following dependencies installed:

- Rust (>= 1.51)
- UPower (>= 0.99)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/viktory683/hypridle_manager.git
   ```

2. Navigate to the project directory:

   ```bash
   cd hypridle_manager
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

4. (Optional) Copy the built executable to a directory in your $PATH, such as `~/.local/bin`:

   ```bash
   cp target/release/hypridle_manager ~/.local/bin
   ```

### Usage

To use Hypridle Manager, run the following command:

```bash
hypridle_manager
```

### Configuration

Hypridle Manager uses a configuration file to define power profiles. The default configuration file is named `hypridle_manager.ron` and is expected to be located in the configuration directory.

Example configuration (`hypridle_manager.ron`):

```rust
Config(
  hypridle_ac_conf: "/path/to/ac_profile.conf",
  hypridle_bat_conf: "/path/to/battery_profile.conf",
  log_level: "info",
  hypridle_quiet: false
)
```

- `hypridle_ac_conf`: Path to the power profile configuration file when on AC power.
- `hypridle_bat_conf`: Path to the power profile configuration file when on battery power.
- `log_level`: Logging level (trace, debug, info, warn, error).
- `hypridle_quiet`: Whether to run Hypridle Manager in quiet mode (true/false).

By default, Hypridle Manager looks for the configuration file in the following locations:
- `$XDG_CONFIG_HOME/hypr/hypridle_manager.ron`
- `~/.config/hypr/hypridle_manager.ron`

### Tip

1. Add this to your `hyprland.conf` instead of `exec-once = hyprpaper`
   ```conf
   exec-once = hypridle_manager
   ```
2. If you maded some changes in hypridle config files, you can simply toggle power source to reload hypridle

### TODO

- [ ] Wait till [hypridle#67](https://github.com/hyprwm/hypridle/issues/67) will be completed
- [ ] Sync listener states between process relaunch?

### Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.
