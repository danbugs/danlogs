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

**Step 1)** Clone wabt's repository: `git clone --recursive https://github.com/WebAssembly/wabt`
**Step 2)** Go into your newly cloned directory: `cd wabt`
**Step 3)** Download [CMake](https://cmake.org/) and [Visual Studio 2015 or later](https://www.visualstudio.com/)
**Step 4)** Make a build folder inside wabt: `mkdir build`
**Step 5)** Go into your new folder: `cd build`
**Step 6)** Run the following: `cmake .. -DCMAKE_BUILD_TYPE=DEBUG -DCMAKE_INSTALL_PREFIX=..\ -G "afafasff"` 

> In here, I'm writing whatever (i.e., `"afafasff"`) for the generator name so you can get a list of generators available in your system). For example, I saw this:
>> ![img](https://i.imgur.com/LO36nO7.png)
> There's probably a better way to do this but it works 🤷‍♂️

**Step 7)** Choose the generator you'd like, I choose the latest and run: `cmake .. -DCMAKE_BUILD_TYPE=DEBUG -DCMAKE_INSTALL_PREFIX=..\ -G "Visual Studio 16 2019"`

**Step 8)** After the previous command finishes, run: `cmake --build . --config DEBUG --target install`.

⚠ *Note*:  After this point, we'll be editing your computer's environment variables. If you don't feel confident doing this, I recommend you don't. If you delete environment variables or overwrite something, it can be a pain to fix and it will most probably hurt the functionality of other programs in your computer. If you prefer to not add the wabt binaries to your path, you can still use them by directly referencing the path of the folder where they're at (for example, if they are in your Downloads folder, you can do: `~\Downloads\wabt\bin\wat2wasm --help` to use thw wat2wasm binary).

**Step 9)** To finish up, let's add the generated executables to the path. For convenience, I copied the wabt folder to `C:\`. In the terminal, run `cd C:\ | explorer .` and copy and paste the wabt folder there. 

**Step 10)** In the search bar at the bottom left corner search for "Environment Variables", open the following item:

![img](https://i.imgur.com/u1wmlon.png)

**Step 11)** Click on the "Environment Variables..." button at the bottom right corner of the system's properties window:

![img](https://i.imgur.com/2nw66Av.png)

**Step 12)** You should see a window titled "Environment Variables" with two sections: "User variables for <your-user-name>" and "System variables". In "System variables" look for a variable called "Path" and select it. After selecting the variables (by clicking on it), hit "Edit..." under the "System variables" section.

**Step 13)** You should now see a window titled "Edit environment variable". Click the button titled "New" on the top right side and insert "C:\wabt\bin" (or whatever different path you chose for your `wabt` folder) in the focused input text field.

**Step 13)** Open a new terminal window and run: `wat2wasm --help`. You should see something like this:

![img](https://i.imgur.com/phROlYs.png)

---
For more information, check out wabt's README [here](https://github.com/WebAssembly/wabt).
