# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///


import os
import pathlib


def main() -> None:
    if not pathlib.Path("./inputs").exists():
        os.makedirs("./inputs")
    for i in range(26):
        path = pathlib.Path(f"./inputs/{i}")
        if not path.exists():
            path.touch()


if __name__ == "__main__":
    main()
