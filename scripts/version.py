import os
import rtoml

current_version = os.getenv("VERSION")

config = rtoml.load("Cargo.toml")

if config["package"]["version"] != current_version:
    # exit with error message
    print(f"Version mismatch, expected {current_version}, got {config['package']['version']}")
    exit(1)

print(f"Version match, expected {current_version}, got {config['package']['version']}")
