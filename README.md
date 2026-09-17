# 🖱️ Mouse Accelerator

A lightweight **Windows mouse acceleration utility** written in **Rust**.

Mouse acceleration makes your sensitivity increase as you move the mouse faster:

```text
Slow movement → low sensitivity
Fast movement → higher sensitivity
```

This project uses **[Interception](https://github.com/oblitum/Interception)** to intercept and modify mouse movement at a low level.

## ⚙️ Build

### 1. Install Interception

Download the official Interception package:

👉 https://github.com/oblitum/Interception/releases

Open **Command Prompt as Administrator**, go to the `Command Line Installer` folder and run:

```powershell
install-interception.exe /install
```

🔄 Restart Windows after installation.

### 2. Clone the repository

```powershell
git clone https://github.com/null68/mouse-accelerator.git
cd mouse-accelerator
```

### 3. Build

```powershell
cargo build --release
```

That's it. 🚀

The required Interception library files are handled automatically by the build.

Your executable will be here:

```text
target/release/mouse-accelerator.exe
```

## 🎛️ Usage

​```powershell
mouse-accelerator.exe [--sens N] [--accel N] [--power N] [--cap N]
​```

Curve: `multiplier = sens + accel * speed^power`, capped at `cap`.

| Flag      | Meaning               | Default |
|-----------|-----------------------|---------|
| `--sens`  | base sensitivity      | `1.0`   |
| `--accel` | acceleration strength | `0.03`  |
| `--power` | curve exponent        | `2.0`   |
| `--cap`   | maximum multiplier    | `3.0`   |

Run with no flags to use the defaults.

## 🗑️ Uninstall Interception

Run as Administrator:

```powershell
install-interception.exe /uninstall
```

Then restart Windows.
