# cargo-launcher

[![Build Status](https://travis-ci.com/watawuwu/cargo-launcher.svg?branch=master)](https://travis-ci.com/watawuwu/cargo-launcher)

If the cargo project is a binary crates, this tool can register the binary in the following launcher.

- [Alfred](https://www.alfredapp.com/workflows/)
    - Register as workflow
- [Hain](https://hainproject.github.io/hain/docs/)
    - Register as devplugin

## TODO
- [ ] cargo workspace(Only single binary crates)
- [ ] customize launcher scripts

## Usage

### Alfred workflow

```
$ cd {your binary crates project}

# Install to local, or manually install
#   The script path is set as follows
#   PATH=$HOME/.cargo/bin:$HOME/.local/bin:/usr/local/bin:$PATH
$ cargo install --path .
...
  Installing /Users/watawuwu/.cargo/bin/{your-binary}

# Export to Alfred
$ cargo launcher alfred
```

- Install to Alfred

<img src="alfred.png" width="300px">

<img src="workflow.png" width="300px">

### Hain plugin

```
$ cd {your binary crates project}

# Install to local, or manually install
#   The script path is set as follows
#   PATH=$HOME/.cargo/bin:$HOME/.local/bin:/usr/local/bin:$PATH
$ cargo install --path .
...
  Installing /Users/watawuwu/.cargo/bin/{your-binary}

# Export to hain devplugin
$ cargo launcher hain
```

- Restart Hain

