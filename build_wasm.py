# Take emsdk path from command line and compile project for wasm
# Usage: python build_wasm.py <path_to_emsdk>
# Note: windows only

import os
import sys

# Ensure that the user has provided the path to emsdk
if len(sys.argv) != 2:
    print("Usage: python build_wasm.py <path_to_emsdk>")
    sys.exit(1)

# Get the path to emsdk
emsdk_path = sys.argv[1]

os.system(emsdk_path + "/emsdk activate latest")
os.system("embuilder.bat build sdl2")
os.environ["EMCC_CFLAGS"] = "-s USE_SDL=2"
os.system("cargo build --target wasm32-unknown-emscripten --release")