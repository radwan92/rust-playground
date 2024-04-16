# Take emsdk path from command line and compile project for wasm
# Usage: python build_wasm.py <path_to_emsdk>
# Note: windows only

import os
import sys
import shutil

def ensure_success(ok, message):
    if ok != 0:
        print(message)
        sys.exit(1)

def install_emsdk(emsdk_path):
    ok = os.system(emsdk_path + "/emsdk install latest")
    ensure_success(ok, "Failed to install latest version of emsdk")

def build_sdl2(emsdk_path):
    ok = os.system(emsdk_path + "/emsdk activate latest && embuilder.bat build sdl2")
    ensure_success(ok, "Failed to build sdl2")

def build_project(emsdk_path, project_name):
    ok = os.system(emsdk_path + "/emsdk activate latest && cargo build --bin " + project_name + 
                   " --target wasm32-unknown-emscripten --manifest-path " + project_name + "/Cargo.toml --release")
    ensure_success(ok, "Failed to build " + project_name + " for WASM")
    shutil.copy("./target/wasm32-unknown-emscripten/release/" + project_name.replace("-", "_") + ".wasm", "./pages/")
    shutil.copy("./target/wasm32-unknown-emscripten/release/" + project_name + ".js", "./pages/")

def main():
    # Ensure that the user has provided the path to emsdk
    if len(sys.argv) != 2:
        print("Usage: python build_wasm.py <path_to_emsdk>")
        sys.exit(1)

    # Get the path to emsdk
    emsdk_path = sys.argv[1]

    install_emsdk(emsdk_path)
    build_sdl2(emsdk_path)
    os.environ["EMCC_CFLAGS"] = "-s USE_SDL=2"
    build_project(emsdk_path, "basic-sample")

main()
