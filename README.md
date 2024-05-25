# Rust Playground

Some small project to learn Rust.

The "maze" project comes from the [One Lone Coder](https://www.youtube.com/@javidx9) YouTube channel.
Kudos to Javidx9 for providing such a great content!

All projects can be seen in action on the GitHub pages of this project:
https://radwan92.github.io/rust-playground/

> :warning: Currently this is **Windows** only.
> The amount of Windows specific code is minimal, so it should be easy to port to other platforms.

## Usage

### Running projects

List of projects can be found in the root Cargo.toml file.
Every `[[bin]]` entry is a separate project that can be run with the following command:

```bash
cargo.exe run --bin <project_name> --manifest-path ./<project_dir>/Cargo.toml
```

For example, to run the `Basic Sample` project, you would run:

```bash
cargo.exe run --bin basic_sample --manifest-path ./basic_sample/Cargo.toml
```

### Building for WASM (Web)

#### Prerequisites

_Emscripten_

Follow the instructions on the [Emscripten website](https://emscripten.org/docs/getting_started/downloads.html).

_rustup_

Required for installing the `wasm32-unknown-emscripten` target from the next step.
Follow instructions on the [rustup website](https://rustup.rs).

_wasm32-unknown-emscripten target_

Now that we have `rustup` we can install the `wasm32-unknown-emscripten` target:

```bash
rustup target add wasm32-unknown-emscripten
```

_python_

Follow the instructions on the [Python website](https://www.python.org/downloads).

Not strictly required, but the build script uses Python.
You're free to take a look at what the script does and run the commands manually.
The script is located at `./scripts/build_wasm.py`.

#### Building

To build a selected project for the Web use the `./scripts/build_wasm.py` script:

```bash
python ./scripts/build_wasm.py <path_to_emsdk> <project_name>
```

For example, assuming you have the `emsdk` installed in the `C:\emsdk` directory and you want to build
the `Basic Sample` project:

```bash
python ./scripts/build_wasm.py C:\emsdk basic_sample
```

The script will build the project and copy the necessary files to the `./pages` directory.

### Testing Web builds locally

After building the project for WASM, you can execute the `./scripts/serve.sh` which will start a simple HTTP server on
port 8080.
You can then access the project by navigating to `http://localhost:8080` in your browser.