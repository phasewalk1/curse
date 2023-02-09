# curse <img src="https://github.com/phasewalk1/curse/blob/master/docs/IMG_5757.JPG" height=300 align="right" style="padding-left: 20px;">
__Command-line Unified Research, Science, and Engineering__

## Overview

`curse` was born out of my disregard for working in integrated LaTeX environments. Though there are many good ones out there ([overleaf](https://www.overleaf.com) being a great example), I constantly found myself missing _vim-motions_ and my tty in general. I found that what I really desired, was something that blends the UX of an integrated environment while retaining a philosophy of granular control. `curse` is meant to be a bare-minimum binary that only acts when called on, i.e., `curse` is not a linter; `curse` is not a frontend; `curse` is not a novel environment; `curse` does not get in your way.

## Features

### curse `new` and `Curse.toml`

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

### curse `compile`

**Single File**
Compile a source file `example.TeX` to pdf:

```bash
curse compile example.TeX
```

Compile a source file to dvi:

```bash
curse compile example.TeX -o dvi
```

**Workspace**
To compile all sources in the `src/` directory to pdfs (add `-o dvi` for .dvi builds):

```bash
curse compile -c
```

To compile all sources in any directory `mysrc/` to pdfs:

```bash
curse compile-src -x mysrc/
```
