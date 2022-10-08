# How to release Rust 🦀 apps with jReleaser

## TL;DR Code

%[https://github.com/dirien/rust-jreleaser]

## Introduction

Recently I decided to start learning Rust 🦀!

![image.png](https://cdn.hashnode.com/res/hashnode/image/upload/v1665239841251/nHf9buDL5.png align="left")

%[https://twitter.com/_ediri/status/1577868970593583104?s=20&t=0NWRHqD7ohV8drrQyYVkuQ]

And of course, one of the important parts for me was: How can I release the binaries and distribute them. When I program in Golang, I always use [GoReleaser](https://goreleaser.com/) or [ko](https://github.com/ko-build/ko)

Used to these tools, I wanted a similar tool for Rust too. After searching though the Web and GitHub I found out that [jReleaser](https://jreleaser.org) is also able to release and deploy  Rust projects!

In this article, I want to show how to use jReleaser with a simple Rust CLI app and use Homebrew as a first distribution integration.

## Prerequisites

To code along, you should install the following tools in your workstation:

- [jReleaser](https://jreleaser.org)
- [Rust](https://www.rust-lang.org)

## Creation of the example app

In this example, we will create a simple Rust project and release it with `jReleaser`. The project will be a simple CLI application that prints the world-famous "Hello, World!" message.

With following command, you can scaffold an empty Rust project:

```bash
cargo init
```

The output should look something like this:

```bash
     Created binary (application) package
```

Next, we want to pass the name of the person we want to greet as an argument to the application. For this, I am going to use `clap`, a [very popular library](https://docs.rs/clap/latest/clap/) for parsing command-line arguments.

Enter this cargo command in your terminal:

```bash
cargo add clap --features derive
```

And the `clap` library will be added under `dependencies` to your `Cargo.toml` file:

```toml
[dependencies]
clap = { version = "4.0.10", features = ["derive"] }
```

With features, we can add additional functionality to our dependencies. In this case, we want to use the `derive`

The final code of our application will look like this:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "Engin Diri", version, long_about = None)]
/// A very, very simple Hello World application
struct Args {
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Greet someone
    Greet {
        /// Name of the person to greet
        #[clap(default_value = "Unknown")]
        name: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Greet { name } => println!("Hello, {}!", name),
    }
}
```

The notable parts are the definition of the `Args` struct and the `SubCommand` enum. In the `Args` struct, we refer to the `SubCommand` enum as a subcommand. This means that we can call our application with `greet` as a subcommand.

The `Greet` struct is a struct that contains the `name` argument. The `name` argument is a string that defaults to `Unknown` if no value is provided.

In the `main` function, we parse the arguments and match the subcommand. If the subcommand is `greet`, we print the message.

Go try it out:

```bash
cargo run -- greet Engin
```

Should print:

```bash
Hello, Engin!
```

If you don't provide a name, it will default to `Unknown`:

```bash
cargo run -- greet
```

Should print:

```bash
Hello, Unknown!
```

## Release with jReleaser

### Initialize jReleaser

To install jReleaser for your platform, please refer to the [installation guide](https://jreleaser.org/guide/latest/install.html#_stable).

I am a macOS user, that is the reason I will use Homebrew to install jReleaser:

```bash
brew install jreleaser/tap/jreleaser
```

After the installation, we can initialize jReleaser:

```bash
jreleaser init --format yml 
```

This will create a `jreleaser.yml` file in the current directory.

Some values in the file are already filled in, and you need to change them to match your project. As we are going to use Homebrew, we need to add the `packagers` section with the right values. Here I added the `tap` name and `commitAuthor`

```yaml
packagers:
  brew:
    active: ALWAYS
    commitAuthor:
      name: dirien
      email: engin.diri@mail.schwarz
    tap:
      owner: dirien
      name: homebrew-dirien-dev
```      

The rest of the file `jreleaser.yml` is pretty straightforward. You can find more information about the configuration [here](https://jreleaser.org/guide/latest/configuration/index.html)

Now you can build your project with cargo:

  ```bash
  cargo build --release --all-features
```

Assemble the artifacts:

```bash
jreleaser assemble -grs
```

And check the configuration with:

```bash
jreleaser config
```

You should see something like this:

```bash
[INFO]  JReleaser 1.2.0
[INFO]  Konfiguriere mit jreleaser.yml
[INFO]    - Basisverzeichnis 'basedir' ist /Users/dirien/Tools/repos/rust-jreleaser
[INFO]  Reading configuration
[INFO]  Loading variables from /Users/dirien/.jreleaser/config.properties
[WARN]  Variables source /Users/dirien/.jreleaser/config.properties does not exist
[INFO]  Validating configuration
...
```

Looks very good, now we are ready to create a GitHub workflow to release our project.

### Create the GitHub actions

As we're going to create binaries for multiple platforms, we need to create a GitHub workflow which will build the binaries on multiple platforms. For this case, I will use the `matrix` feature of GitHub actions.

```yaml
...
strategy:
  fail-fast: true
  matrix:
    os: [ ubuntu-latest, macOS-latest, windows-latest ]
runs-on: ${{ matrix.os }}
...
```

So we have two jobs, one called `build` and the other called `release`.

The notable parts of the `build` job are, that we set the toolchain to `stable` and we use the `actions-rs/cargo` action to build the project. Next step is to call the jReleaser assemble command to assemble the artifacts and finally we upload the artifacts to a folder called `artifacts`.

```yaml
      - uses: actions-rs/toolchain@b2417cde72dcf67f306c0ae8e0828a81bf0b189f # tag=v1.0.7
        with:
          toolchain: stable

      - uses: actions-rs/cargo@ae10961054e4aa8b4aa7dffede299aaf087aa33b # tag=v1.0.3
        with:
          command: build
          args: --release --all-features

      - name: jReleaser assemble
        uses: jreleaser/release-action@9d00b8a3e38acac18558faf7152ca24368ed0d9f # tag=v2.2.0
        with:
          arguments: assemble
        env:
          JRELEASER_GITHUB_TOKEN: ${{ secrets.GH_PAT }}

      - name: Upload artifacts
        uses: actions/upload-artifact@3cea5372237819ed00197afe530f5a7ea3e805c8 # tag=v3.1.0
        with:
          name: artifacts
          path: |
            out/jreleaser/assemble/rust-jreleaser/archive/*.zip
```

The `release` job very simple, it will download the artifacts folder and uses the `jreleaser/release-action` action to execute the `release` command from jReleaser. Use `PartifactsDir` flag to point to the `artifacts` folder.

```yaml
      - name: Download artifacts
        uses: actions/download-artifact@fb598a63ae348fa914e94cd0ff38f362e927b741 # tag=v3.0.0

      - name: jReleaser release
        uses: jreleaser/release-action@9d00b8a3e38acac18558faf7152ca24368ed0d9f # tag=v2.2.0
        with:
          arguments: release -PartifactsDir=artifacts -PskipArchiveResolver
        env:
          JRELEASER_GITHUB_TOKEN: ${{ secrets.GH_PAT }}
```

> As we going to write to a different repository, we need to create a personal access token with the `repo` scope and add it to the GitHub secrets. The name is `GH_PAT`.

## Release

Now we are ready to release our project. We need to create a tag and push it to GitHub. I will use the `v0.1.2` tag for and push it to GitHub.

![image.png](https://cdn.hashnode.com/res/hashnode/image/upload/v1665239621090/FSswfRl9M.png align="left")

If everything went well, you should see the artifacts under the release page, and you should be able to install the binary via Homebrew.

```bash
brew install dirien/homebrew-dirien-dev/rust-jreleaser
Running `brew update --auto-update`...
==> Auto-updated Homebrew!
Updated 2 taps (dirien/dirien-dev and homebrew/core).

You have 28 outdated formulae installed.
You can upgrade them with brew upgrade
or list them with brew outdated.

==> Downloading https://github.com/dirien/rust-jreleaser/releases/download/v0.1.2/rust-jreleaser-0.1.2-darwin-amd64.zip
==> Downloading from https://objects.githubusercontent.com/github-production-release-asset-2e65be/547448893/7b2bf4ad-0cdd-49e5-ba23-49cb580d1963?X-Amz-Algorithm=AWS4-
######################################################################## 100.0%
==> Installing rust-jreleaser from dirien/dirien-dev
🍺  /usr/local/Cellar/rust-jreleaser/0.1.2: 5 files, 1003.6KB, built in 5 seconds
==> Running `brew cleanup rust-jreleaser`...
Disable this behaviour by setting HOMEBREW_NO_INSTALL_CLEANUP.
Hide these hints with HOMEBREW_NO_ENV_HINTS (see `man brew`).
Removing: /Users/dirien/Library/Caches/Homebrew/rust-jreleaser--0.1.0.zip... (394.7KB)
```

And the final test, with running the app:

```bash
rust-jreleaser
A very, very simple Hello World application

Usage: rust-jreleaser <COMMAND>

Commands:
  greet  Greet someone
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Wrap-Up

`JReleaser` is a very powerful tool, not only for JVM based application but also for Rust.

Next steps would be from here:

- Add signing with Cosign
- Create a Docker Image
- Add ARM architecture
