import os
import shutil
from pathlib import Path
from datetime import datetime
import hashlib
import logging

import oyaml as yaml
import git

if not os.getenv("CI"):
    raise Exception("This script is only intended to be run in CI")

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("winget script")
logger.info("Loading winget manifest")


def get_file_sha256(file_path):
    sha256 = hashlib.sha256()
    with open(file_path, 'rb') as fi:
        data = fi.read()
        sha256.update(data)
    return sha256.hexdigest().upper()


new_version = os.getenv("GITHUB_REF_NAME")

repo_base = Path(__file__).parent.parent

manifest_folder = repo_base / "winget" / "Zxilly" / "NotifyCli" / "0.1.1"
target_folder = repo_base / "winget" / "Zxilly" / "NotifyCli" / new_version

windows_arm64_file = \
    repo_base / "binary" / "aarch64-pc-windows-msvc" / "znotify-aarch64-pc-windows-msvc.exe"
windows_arm64_sha256 = get_file_sha256(windows_arm64_file)
windows_x64_file = \
    repo_base / "binary" / "x86_64-pc-windows-msvc" / "znotify-x86_64-pc-windows-msvc.exe"
windows_x64_sha256 = get_file_sha256(windows_x64_file)

installer_file = "Zxilly.NotifyCli.installer.yaml"
manifest_file = "Zxilly.NotifyCli.yaml"
locale_file = "Zxilly.NotifyCli.locale.en-US.yaml"

shutil.copytree(manifest_folder, target_folder)
with open(target_folder / installer_file, "r+") as f:
    logger.info("Updating installer file")
    installer = yaml.safe_load(f)

    installer["PackageVersion"] = new_version
    installer["ReleaseDate"] = datetime.now().strftime("%Y-%m-%d")
    installer["Installers"][0]["InstallerUrl"] \
        = f"https://github.com/ZNotify/cli/releases/download/{new_version}/znotify-x86_64-pc-windows-msvc.exe"
    installer["Installers"][0]["InstallerSha256"] = windows_x64_sha256
    installer["Installers"][1]["InstallerUrl"] \
        = f"https://github.com/ZNotify/cli/releases/download/{new_version}/znotify-aarch64-pc-windows-msvc.exe"
    installer["Installers"][1]["InstallerSha256"] = windows_arm64_sha256

    f.seek(0)

    yaml.safe_dump(installer, f, default_flow_style=False, allow_unicode=True)

with open(target_folder / manifest_file, "r+") as f:
    logger.info("Updating manifest file")
    manifest = yaml.safe_load(f)

    manifest["PackageVersion"] = new_version

    f.seek(0)

    yaml.safe_dump(manifest, f, default_flow_style=False, allow_unicode=True)

with open(target_folder / locale_file, "r+") as f:
    logger.info("Updating locale file")
    locale = yaml.safe_load(f)

    locale["PackageVersion"] = new_version

    f.seek(0)

    yaml.safe_dump(locale, f, default_flow_style=False, allow_unicode=True)

logger.info(f"Edited {new_version} manifest")

repo = git.Repo(repo_base)

repo.config_writer().set_value("user", "name", "Zxilly").release()
repo.config_writer().set_value("user", "email", "zxilly@outlook.com").release()

repo.git.add(A=True)
repo.git.commit(m=f"chore: update winget manifest to {new_version}\n\n[skip ci]")
repo.git.push()
