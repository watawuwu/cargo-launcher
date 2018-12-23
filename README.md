# cargo-launcher

[![Build Status](https://travis-ci.com/watawuwu/cargo-launcher.svg?branch=master)](https://travis-ci.com/watawuwu/cargo-launcher)
[![Latest version](https://img.shields.io/crates/v/cargo-launcher.svg)](https://crates.io/crates/cargo-launcher)
[![Documentation](https://docs.rs/cargo-launcher/badge.svg)](https://docs.rs/crate/cargo-launcher)
![License](https://img.shields.io/crates/l/cargo-launcher.svg)

If the cargo project is a binary crates, this tool can register the binary in the following launcher.

- [Alfred](https://www.alfredapp.com/workflows/)
    - Register as workflow
- [Hain](https://hainproject.github.io/hain/docs/)
    - Register as devplugin
- [Albert](https://albertlauncher.github.io/docs/extensions/python/)
    - Register as Python extension

## TODO
- [ ] cargo workspace(Only single binary crates)
- [ ] customize launcher scripts

## Usage

### Common

- Install CLI binary

``` shell
$ cd {your binary crates project}

# Install to local, or manually install
#   The script path is set as follows
#   PATH=$HOME/.cargo/bin:$HOME/.local/bin:/usr/local/bin:$PATH
$ cargo install --path .
...
  Installing /Users/watawuwu/.cargo/bin/{your-binary}
```

### Alfred workflow

- Generate Alfredworkflow file

```
$ cargo launcher alfred
```

- Install to Alfred

<img src="alfred.png" width="300px">

<img src="workflow.png" width="300px">

### Hain plugin

- Export to hain devplugin directory

```
$ cargo launcher hain
```

- Restart Hain


### Albert plugin

- Export to albert module directory

```
$ cargo launcher albert
```

- Check the checkbox of the python extension list and activate the setting

<img src="albert.png" width="300px"/>
