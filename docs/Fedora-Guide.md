# Release Binary

If you do not want to build from source, download the prebuilt binary from [Releases](https://github.com/KnightroParth/legion-keyboard-custom/releases).

1. Make the binary executable:
    ```bash
    chmod +x legion-kb-rgb
    ```

2. (Optional) Move it into your PATH:
    ```bash
    sudo mv legion-kb-rgb /usr/local/bin/
    sudo chmod +x /usr/local/bin/legion-kb-rgb
    ```

3. Open a fresh terminal, then run `legion-kb-rgb` or `legion-kb-rgb -g`. If the command is not found, verify it exists with `ls /usr/local/bin/ | grep "legion-kb-rgb"`.

4. If that still does not work, follow [Building from source](#building-from-source).

# Building From Source

This guide documents all dependencies and steps required to build `legion-kb-rgb` from source on Fedora Linux. Most of this applies to other distros too, with package manager substitutions.

> [!IMPORTANT]
> **Some dependencies must be built from source**
>
> This project pulls in `scrap`, a screen capture library maintained inside the RustDesk repository. Its build script is hardcoded to look for **static** `.a` archive files for `libvpx`, `libaom`, and `libyuv` at the path `/tmp/installed/x64-linux/`. Most Linux distributions including Fedora only ship the **shared** `.so` versions of these libraries in their package repositories. Shared libraries cannot satisfy this requirement, so even if you install `libvpx-devel` via `dnf`, the build will still fail with "could not find native static library" errors. The only solution is to clone and compile these three libraries from source, outputting the static `.a` files to exactly the path `scrap` expects.

---

## System Dependencies

Install required packages via `dnf`:

```bash
sudo dnf install -y \
  rustup \
  git \
  gcc gcc-c++ \
  clang clang-devel \
  cmake \
  nasm yasm \
  libvpx-devel \
  libaom-devel \
  libyuv-devel \
  libxdo-devel \
  libX11-devel \
  libXfixes-devel \
  libXtst-devel \
  libXrandr-devel \
  libXi-devel \
  alsa-lib-devel \
  pulseaudio-libs-devel \
  glib2-devel \
  gtk3-devel \
  pam-devel \
  libjpeg-turbo-devel
```

---

## Rust Toolchain

```bash
rustup-init
rustup install stable
rustup default stable
source ~/.bashrc
```

---

## Static Libraries (Built from Source)

The `scrap` dependency (pulled in via RustDesk's screen capture library) expects **static** `.a` files for `libvpx`, `libaom`, and `libyuv` at `/tmp/installed/x64-linux/`. Most distros, including Fedora, only ship shared `.so` versions, so these must be built manually.

### 1. libvpx (VP8/VP9 codec)

```bash
cd /tmp
git clone https://chromium.googlesource.com/webm/libvpx
cd libvpx
./configure \
  --prefix=/tmp/installed/x64-linux \
  --disable-shared \
  --enable-static \
  --disable-examples \
  --disable-tools \
  --disable-docs \
  --target=x86_64-linux-gcc
make -j$(nproc)
sudo make install
```

### 2. libaom (AV1 codec)

```bash
cd /tmp
git clone https://aomedia.googlesource.com/aom
mkdir aom_build && cd aom_build
cmake ../aom \
  -DCMAKE_INSTALL_PREFIX=/tmp/installed/x64-linux \
  -DBUILD_SHARED_LIBS=OFF \
  -DENABLE_TESTS=OFF \
  -DENABLE_DOCS=OFF \
  -DENABLE_TOOLS=OFF
make -j$(nproc)
sudo make install

# libaom installs to lib64 but scrap looks in lib — symlink it
sudo ln -s /tmp/installed/x64-linux/lib64/libaom.a /tmp/installed/x64-linux/lib/libaom.a
```

### 3. libyuv (YUV color space conversion)

```bash
cd /tmp
git clone https://chromium.googlesource.com/libyuv/libyuv
cd libyuv
cmake . \
  -DCMAKE_INSTALL_PREFIX=/tmp/installed/x64-linux \
  -DBUILD_SHARED_LIBS=OFF
make -j$(nproc)
sudo make install
```

---

## Building the Project

```bash
git clone https://github.com/KnightroParth/legion-keyboard-custom
cd legion-keyboard-custom
cargo build --release
```

The compiled binary will be at:

```
target/release/legion-kb-rgb
```

---

## Verifying Your Keyboard IDs

Before setting up udev rules, confirm that your keyboard's USB IDs match the ones this guide uses (`048d:c996` or `048d:c993`). If they differ, you'll need to use your own IDs in the udev rules.

### Step 1: List USB devices

```bash
lsusb
```

Look for a line mentioning your Legion laptop's keyboard or ITE (the IEC controller brand). Example mine says:

```
Bus 001 Device 004: ID 048d:c993 Integrated Technology Express, Inc. ITE Device(8295)
Bus 001 Device 005: ID 048d:c996 Integrated Technology Express, Inc. ITE Device(8176)
```

The `ID` field is `vendorId:productId` in this case `048d:c996`.

### Step 2: If unsure which line is the keyboard

Unplug an external USB device first to reduce noise, then run:

```bash
lsusb | grep -i "048d\|ITE\|Integrated Technology"
```

Or just try running the app without udev rules first — it will print the IDs it detected:

```
Bus 001 Device 004: ID 048d:c993 Integrated Technology Express, Inc. ITE Device(8295)
Bus 001 Device 005: ID 048d:c996 Integrated Technology Express, Inc. ITE Device(8176)
```

Use those exact values (without the `0x` prefix) in the udev rules below.

### Step 3: Cross-check with your Legion model

| Model | Expected IDs |
|---|---|
| Legion 5 Pro (2021) | `048d:c996` |
| Legion 5 (2021) | `048d:c993` |
| Other models | Run the app and check its output |

If your IDs are different, substitute them in the udev rules in the next section.

---

## Keyboard Permissions (udev Rules)

Without udev rules, the app will fail to find the keyboard even if it's supported. Create the following file:

```bash
sudo nano /etc/udev/rules.d/99-legion-keyboard.rules
```

Add these lines (covers known IEC controller IDs):

```
SUBSYSTEM=="usb", ATTR{idVendor}=="048d", ATTR{idProduct}=="c996", MODE="0666"
SUBSYSTEM=="usb", ATTR{idVendor}=="048d", ATTR{idProduct}=="c993", MODE="0666"
```

Then reload the rules:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

---

## Running

```bash
./target/release/legion-kb-rgb
```

> [!TIP]
>
> After setting up udev rules, `sudo` should not be required. If the keyboard is still not found, try running with `sudo` once to confirm it's a permissions issue and not a hardware/driver issue.

---

## Troubleshooting

| Error | Fix |
|---|---|
| `fatal error: 'vpx/vp8.h' file not found` | Install `libvpx-devel` and build static libvpx from source |
| `fatal error: 'aom/aom.h' file not found` | Install `libaom-devel` and build static libaom from source |
| `could not find native static library 'vpx'` | Build libvpx from source into `/tmp/installed/x64-linux/` |
| `could not find native static library 'aom'` | Build libaom from source; symlink `lib64/libaom.a` → `lib/libaom.a` |
| `unable to find library -lxdo` | Install `libxdo-devel` |
| `Failed to find a valid keyboard` | Set up udev rules as described above |
| `cp: not writing through dangling symlink` | Remove stale symlink with `sudo rm` before running `make install` |

--- 