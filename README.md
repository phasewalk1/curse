# curse <img src="https://github.com/phasewalk1/curse/blob/master/docs/IMG_5757.JPG" height=200 width=200 align="right" style="padding-left: 20px;">

**Command-line Unified Research, Science, and Engineering**

## Overview

`curse` was born out of my disregard for working in integrated LaTeX environments. Though there are many good ones out there ([overleaf](https://www.overleaf.com) being a great example), I constantly found myself missing _vim-motions_ and my tty in general. I found that what I really desired, was something that blends the UX of an integrated environment while retaining a philosophy of granular control. `curse` is meant to be a bare-minimum binary that only acts when called on, i.e., `curse` is not a linter; `curse` is not a frontend; `curse` is not a novel environment; `curse` does not get in your way.

## Features

### curse `new` and `Curse.toml`

> Instantiates a Workspace

`curse` provides a command for quickly initializing a local LaTeX workspace. This command is the `new` command. `new` creates a directory with `src/`, `artifacts/`, and `build/`. `src/` contains all the LaTeX source files, `artifacts/` is where build artifacts are stored, and `build/` contains the compiled source (.pdf, .dvi). The `new` command also creates a manifest template for you at `Curse.toml`. The default manifest file looks like this,

```toml
[project]
name = "curseforge"
version = "0.1.0"
authors = ["Your Name"]

[defaults]
target = "pdf"
artifacts-dir = "artifacts"
target-dir = "build"
```

**The Manifest**
For recording workspace metadata and configuration defaults, `curse` uses a manifest file at `Curse.toml`. The manifest is to be placed at the root of whatever workspace you want to use `curse` in.

**Defaults**
The `defaults` field of the manifest defines global overrides against `curse`'s predefined defaults.

- `defaults.target` - The default output format when compiling LaTeX source.
- `defaults.artifacts-dir` - The directory to store build artifacts.
- `defaults.target-dir` - The directory to store compiled documents.

### curse `check`

`check` ensures that the `Curse.toml` manifest contains no errors. Namely, that the `defaults.target` field is a valid output format, and that all optional fields are valid (if they stray from defaults).

```bash
curse check
```

### curse `make`

> Compiles all Sources in a Workspace

```bash
curse make
```

### curse `compile`

> Compiles a Single Source File

Compile a source file `example.TeX` to pdf:

```bash
curse compile example.TeX
```

Compile a source file to dvi:

```bash
curse compile example.TeX -o dvi
```
