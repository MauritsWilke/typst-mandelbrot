# Inspired by @MDLC01 => https://github.com/MDLC01/board-n-pieces/blob/main/build.py

#!/usr/bin/env python3

from pathlib import Path
import shutil
import subprocess

TYPST = 'typst'
LIBRARY_DIR = Path('src/')
PLUGIN_DIR = Path('plugin/')
README = 'README.md'
PLUGIN_NAME = 'mandelbrot_plugin.wasm'

def build_plugin():
    print("Building plugin")

    BUILD_TARGET = 'wasm32-unknown-unknown'
    PLUGIN_PATH = PLUGIN_DIR.joinpath('target', BUILD_TARGET, 'release', PLUGIN_NAME)

    subprocess.run(
        ['cargo', 'build', '--release', '--target', BUILD_TARGET, '--quiet'],
        cwd=PLUGIN_DIR,
        check=True,
    )

    shutil.copyfile(PLUGIN_PATH, LIBRARY_DIR.joinpath(PLUGIN_NAME))

def test():
    print("Testing")

    subprocess.run(
        ["typst", "compile", "--root", ".", "-f", 'png', "tests/test.typ"]
    )

def main():
    build_plugin()
    test()


if __name__ == '__main__':
    try:
        main()
    except subprocess.CalledProcessError:
        exit(1)
