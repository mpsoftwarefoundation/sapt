<img src="assets/sapt_icon.svg" width="125">

Combine `sudo` and `apt` into one universal command.

```bash
sapt install example
sapt purge example
sapt update && sapt upgrade
```

## Why `sapt`?

`sapt` is useful for three reasons:

* **Efficiency**

  Installing or removing a bunch of packages can be annoying when you constantly have to type `sudo apt ...`. `sapt` removes the friction.

* **Consistency**

  Always type the same thing. No need to remember "`sudo apt`" versus just "`apt`". With `sapt`, it always “just works.”

* **Extendability**

  Since `sapt` is written in Rust, new features (like logging, output coloring, or package install summaries) can be added on top without breaking the normal `apt` behavior.

## Installation

### Option 1 (via apt)
Download the `.deb` file from our [releases page](https://github.com/mpsoftwarefoundation/sapt/releases/latest).

Install it via `apt`:

```bash
sudo apt install path/to/sapt.deb
```

### Option 2 (via Cargo)
Clone and build with Cargo:

```bash
git clone https://github.com/mpsoftwarefoundation/sapt.git
cd sapt
cargo build --release
```

Move the binary into your path:

```bash
sudo mv target/release/sapt /usr/local/bin/
```

Now you can run `sapt` from anywhere:

```bash
sapt update
sapt install neofetch
```

## Usage

`sapt` works exactly like `sudo apt`:

```bash
sapt <apt-arguments>
```

Examples:

```bash
sapt install curl git build-essential
sapt remove firefox
sapt autoremove
```

All prompts, password requests, and progress bars appear as usual.

## How It Works

`sapt` is a thin wrapper around:

```bash
sudo apt <your arguments>
```

It simply passes arguments through while inheriting terminal input/output, so the behavior is **identical** to running the commands directly.
