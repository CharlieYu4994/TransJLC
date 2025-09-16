<div align="center">
  <img src="docs/image/TransJLC.svg" alt="TransJLC Logo" width="200"/>
</div>

<div align="center">

[![crates.io](https://img.shields.io/crates/v/TransJLC.svg)](https://crates.io/crates/TransJLC)
[![license](https://img.shields.io/github/license/HalfSweet/TransJLC)](https://github.com/HalfSweet/TransJLC/blob/main/LICENSE)
[![release](https://img.shields.io/github/v/release/HalfSweet/TransJLC)](https://github.com/HalfSweet/TransJLC/releases)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/HalfSweet/TransJLC/rust.yml)

</div>

<p align="center">
  <a href="./README.md">English</a> | <a href="./README.zh-CN.md">简体中文</a>
</p>

**TransJLC** is a tool for converting Gerber files from other EDA software to a format compatible with JLCEDA (LCSC's online editor), facilitating production at JLCPCB.

## ✨ Features

-   Automatically identifies Gerber files from common EDA software (KiCad, Protel, Altium Designer).
-   Renames files to match JLCPCB's required naming conventions.
-   Can automatically compress the output files into a ZIP archive for easy uploading.
-   Supports multiple languages for the user interface (English, Chinese, Japanese).
-   Cross-platform support (Windows, macOS, Linux).

## 📦 Installation

### From crates.io (Recommended)

Ensure you have the Rust toolchain installed. Then, you can install `TransJLC` directly from crates.io:

```bash
cargo install TransJLC
```

### From Source

1.  Clone the repository:
    ```bash
    git clone https://github.com/HalfSweet/TransJLC.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd TransJLC
    ```
3.  Build the project in release mode:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/TransJLC`.

## 🚀 Usage

Run the tool from your terminal, providing the necessary options.

### Command-Line Options

| Option          | Short | Description                                                                                             | Default     |
| --------------- | ----- | ------------------------------------------------------------------------------------------------------- | ----------- |
| `--language`    | `-l`  | Sets the display language. Available: `auto`, `en`, `zh-CN`, `ja`.                                        | `auto`      |
| `--eda`         | `-e`  | Specifies the source EDA software. Available: `auto`, `kicad`, `jlc`, `protel`.                         | `auto`      |
| `--path`        | `-p`  | The path to the directory containing your Gerber files.                                                 | `.` (current dir) |
| `--output_path` | `-o`  | The path where the converted files will be saved.                                                       | `./output`  |
| `--zip`         | `-z`  | If set to `true`, creates a ZIP archive of the output files.                                            | `false`     |
| `--zip_name`    | `-n`  | The name of the generated ZIP file (without the `.zip` extension).                                      | `Gerber`    |

### Example

Convert Gerber files located in `D:\Projects\MyPCB\Gerber` and save them to `D:\Projects\MyPCB\Output`, then create a ZIP file named `MyProject.zip`.

```bash
TransJLC -p="D:\Projects\MyPCB\Gerber" -o="D:\Projects\MyPCB\Output" -z=true -n=MyProject
```

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/HalfSweet/TransJLC/issues).

## 📄 License

This project is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file for details.
