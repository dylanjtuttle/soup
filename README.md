# soup
A simple programming language, built from scratch in Rust

![ci workflow](https://github.com/dylanjtuttle/soup/actions/workflows/rust-ci.yml/badge.svg)

## Installation

Note first that soup is designed solely to work on Mac M1 processors.

1. First, ensure you have [installed Rust](https://www.rust-lang.org/tools/install)

2. Clone the repository:

```bash
$ git clone https://github.com/dylanjtuttle/soup.git
$ cd soup
```

3. Open the file `soup`, a bash script that we will call when we want to compile a soup file, in your favourite text editor.

- On line 1, replace `#!/opt/homebrew/bin/bash` with `#![location/of/shell]`, which you can find by running the following command:
    ```bash
    $ which bash
    ```

- Create an environment variable `$SOUP_DIR` which holds the path to the `soup` directory you are currently in, and then add it to your path, by adding the following two lines near the end of your `~/.bashrc` file:
    ```bash
    export SOUP_DIR="[/absolute/path/to/soup/dir]"
    export PATH=$PATH:$SOUP
    ```
    - Note that since `~/.bashrc` is executed once every time a terminal window is opened, these two environment variables only take effect once you exit your current terminal window and open a new one.

4. Compile the compiler with the following command:

```bash
$ cargo build --release
```

5. Create a `.soup` file anywhere on your hard drive, and then compile and run it with the following command:

```bash
$ soup [name].soup
```

Congratulations!!