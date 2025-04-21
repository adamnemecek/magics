# magics

> Master Thesis Project in Computer Engineering at Aarhus University 2024 on "Simulating Multi-agent Path Planning in Complex environments using Gaussian Belief Propagation and Global Path Finding". Available here (https://drive.google.com/file/d/12g-7bqcy_yfkZdpKzxQAErayFJQhu4sE/view?usp=sharing)

## Motivating Example


<!-- https://github.com/user-attachments/assets/832fe84b-4b8b-4473-bfe1-9d87153988af -->


<!-- https://github.com/user-attachments/assets/501aa26a-1b72-4c93-a5a4-22551b6c6d4b -->



<!-- https://github.com/user-attachments/assets/6b8df209-d1db-4f35-9271-1c61ef660ab6 -->

### Waypoint Tracking

[waypoint.webm](https://github.com/user-attachments/assets/a58a148e-c561-432e-8e4f-bc065d4194d0)

### Path Tracking
[path.webm](https://github.com/user-attachments/assets/cf5e77e9-6df2-4b4f-a273-0ddc83512642)

<!-- | <img src="https://github.com/user-attachments/assets/832fe84b-4b8b-4473-bfe1-9d87153988af
" alt="GIF 3" width="380"/> | <img src="https://github.com/user-attachments/assets/6b8df209-d1db-4f35-9271-1c61ef660ab6" alt="GIF 4" width="380"/> |
-->

## Demo

> The video below demonstrates some of the features of the simulation tool, and shows how the GBP algorithm can handle complex scenarios such as a multi-lane twoway junction.

[magics-functionality-demo](https://github.com/user-attachments/assets/8f5d0db6-dd2c-41a3-9a12-4ccddf80d4f3)

## Thesis

The accompanying thesis is available online [here](https://drive.google.com/file/d/12g-7bqcy_yfkZdpKzxQAErayFJQhu4sE/view?usp=sharing).


## Getting Started

### Prerequisites

- Rust toolchain `stable` channel
- Cargo build system
- External dependencies for graphics (see [External Dependencies](#external-dependencies))

### Installation

1. Clone the repository
2. Install the required dependencies for your platform

<!-- ## ICRA Article -->



## Keyboard Controls

### UI Controls

| Key | Function              |
| --- | --------------------- |
| <kbd>H</kbd>  | Toggle Left Panel     |
| <kbd>L</kbd>  | Toggle Right Panel    |
| <kbd>K</kbd>  | Toggle Top Panel      |
| <kbd>J</kbd>  | Toggle Bottom Panel   |
| <kbd>D</kbd>  | Toggle Metrics Window |
| <kbd>U</kbd>  | Change Scale Kind     |

### Camera Controls

| Key/Mouse                          | Function                                     |
| ---------------------------------- | -------------------------------------------- |
| Arrow Keys                         | Move Camera                                  |
| <kbd>C</kbd>                                 | Toggle Camera Movement Mode (Pan/Orbit)      |
| <kbd>Tab</kbd>                                | Switch Camera                                |
| <kbd>R</kbd>                                 | Reset Camera                                 |
| Mouse Wheel                        | Zoom In/Out                                  |
| Left Mouse Button + Mouse Movement | Move Camera (Pan or Orbit depending on mode) |
| Middle Mouse Button                | Pan Camera                                   |
| Right-click Drag                   | Rotate Camera                                |

### Simulation Controls

| Key   | Function                  |
| ----- | ------------------------- |
| <kbd>F5</kbd>    | Reload Current Simulation |
| <kbd>F6</kbd>    | Load Next Simulation      |
| <kbd>F4</kbd>    | Load Previous Simulation  |
| <kbd>Space</kbd> | Pause/Play Simulation     |

### General Controls

| Key    | Function         |
| ------ | ---------------- |
| <kbd>T</kbd>     | Cycle Theme      |
| <kbd>G</kbd>     | Export Graph     |
| <kbd>Ctrl+S</kbd> | Save Settings    |
| <kbd>Ctrl+P</kbd> | Take Screenshot  |
| <kbd>Ctrl+Q</kbd> | Quit Application |


## External Dependencies

Most dependencies used are available through the [crates.io](https://crates.io) registry. And should work on all major platforms supported by the `cargo` build tool. Still some external dependencies are needed for the graphical session.

| Dependencies | Platform Specific |
|--------------|----------|
| `alsa-lib` | Linux |
| `egl-wayland` | Linux + Wayland |
| `fontconfig` | Linux
| `freetype` | Linux |
| `libxkbcommon` | Linux + X11 |
| `udev` | Linux |
| `vulkan-loader` | Linux |
| `wayland` | Linux + Wayland |
| `xorg.libX11` | Linux + X11 |
| `xorg.libXcursor` | Linux + X11 |
| `xorg.libXi` | Linux + X11 |
| `xorg.libXrandr` | Linux + X11 ||

The exact name of the dependency might vary between platforms, and even between Linux distributions. Consult the respective package management tool used on your system for their exact names.


### Nix/NixOS

The `./flake.nix` file provides a development shell with all the necessary dependencies to run the project. If you have[ `direnv`](https://direnv.net/) installed you can simply use the provided `.envrc` and type `direnv allow` to automatically enter it. Otherwise you can run:

```sh
# To enter the development environment
nix develop
```

## Build

The entire project can be build with the following command:

```
cargo build --release
```

## Run

### Available Scenarios

The simulator comes with several pre-configured scenarios to demonstrate different aspects of multi-agent path planning:

- `Circle Experiment`: Robots arranged in a circle swap positions
- `Junction Experiment`: Robots navigate through a four-way junction
- `Structured Junction`: A more complex junction with structured paths
- `Collaborative Complex`: Multiple robots collaborating in a complex environment
- And many more...

Use the `--list-scenarios` command to see all available scenarios.

> **Important**: When specifying a scenario, use the exact name as shown in the `--list-scenarios` output. Do not use file paths.

### WSL Configuration (Windows 10.)

When running in Windows Subsystem for Linux (WSL), you need to configure an X server:

1. Install an X server on Windows (VcXsrv, Xming, or X410)
2. Launch the X server with "Disable access control" checked
3. Set the following environment variables in WSL:

```sh
export DISPLAY=:0
export WINIT_UNIX_BACKEND=x11
```

```sh
# Open the simulator with the first
cargo run --release
cargo run --release -- --list-scenarios # List all available scenarios

# Run a specific scenario
cargo run --release -- -i <SCENARIO_NAME>
# See ./config/scenarios/ for all distributed scenarios
cargo run --release -- -i "Junction Twoway"
```


## Troubleshooting

### Common Issues

#### Display Issues in WSL
- **Problem**: "Failed to build event loop: Os(OsError { ... error: WaylandError(Connection(NoCompositor)) })"
- **Solution**: Set `export DISPLAY=:0` and `export WINIT_UNIX_BACKEND=x11` and in `Cargo.toml` under `bevy` remove `wayland`:

```diff
bevy = { version = "0.13", default-features = true, features = [
-  "wayland",
] }
```

#### ld linking problem in WSL
- **Problem**: `cannot find 'ld'`,
- **Solution**: You will have to go into [.cargo/config.toml](.cargo/config.toml) and disable the use of [`mold`](https://github.com/rui314/mold):

```diff
[target.x86_64-unknown-linux-gnu]
rustflags = [
-  "-Clink-arg=-fuse-ld=mold", # Use mold Linker
  "-Ctarget-cpu=native",
]
```

## Credits

The primary algorithm for GBP path planning is based on [gbpplanner](https://github.com/aalpatya/gbpplanner) by [Aalok Patwardhan](https://aalok.uk/) from  Imperial College London and Dyson Robotics Lab. As part of this thesis we have reimplemented and extended upon in Rust!
