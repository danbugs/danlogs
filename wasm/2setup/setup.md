# How to setup your WASM workflow?

Here I'll explain how to setup your workflow for **macOS** and (most of) **Windows**, which are the two operational systems in which I've done the described processes before. I have never done it on Linux but, if you have, and would like to add to this tutorial a PR is welcome! ッ

## macOS

- We'll install:
    1. `brew`: useful package manager! With it, we can easily install a bunch of command-line tools.
    1. `python3`: we'll use Python to start a simple HTTP server!
    1. `wabt`: This is the WebAssembly Binary Toolkit. We'll primarily use it to convert `.wat` files to `.wasm`.

### `brew`

- Run the following command in your terminal: `/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"`

### `python3`

- Run: `brew install python3`

### `wabt`

- Run: `brew install wabt`

## Windows

- We'll install:
    1. `git`: A version control manager. Necessary in Windows to download `wabt`!
    1. `py`: we'll use Python to start a simple HTTP server!
    1. `wabt`: This is the WebAssembly Binary Toolkit. We'll primarily use it to convert `.wat` files to `.wasm`.

### `git`
- Download it from [here](https://git-scm.com/downloads).
    - When installing:
        1. Keep the initial selection of components.
        1. Select your preferred editor.
        1. Select "Git form the command line and also 3rd-party software".
        1. After this, select whatever better suits your needs. If you'd like help, just comment and I'll get to you.

### `py`
- Download it from [here](https://www.python.org/downloads/).
- Instructions:
    - The provided link should automatically give you the proper installation for your OS and the latest stable version. Just click *Download Python `<your version>`*. When installing, make sure to select that you want to add Python to your PATH.
- You should now be able to open the command prompt and run the command `py --help`.
    - You should get a bunch of specifications for the use of this command-line tool. If your system is not recognizing the command or you have any other error, let me know in the comments.

### `wabt`

- I haven't downloaded `wabt` for Windows but they got great documentation! Follow it [here](https://github.com/WebAssembly/wabt). If you encounter any errors, again, let me know and I'll do my best to help you out!
