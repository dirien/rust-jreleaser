# Release Rust projects with jReleaser

# Prerequisites

- [jReleaser](https://jreleaser.org) installed
- [Rust](https://www.rust-lang.org) installed

# Create an example project

In this example, we will create a simple Rust project and release it with jReleaser. The project will be a simple CLI
application that prints the world famous "Hello, World!" message.

```bash
cargo init
```

You should see something like this as a result:

```bash
     Created binary (application) package
```

We want to pass the name of the person we want to greet as an argument to the application. For this we're going to
use `clap`, a [very popular library](https://docs.rs/clap/latest/clap/) for parsing command-line arguments.

```bash
cargo add clap --features derive
```

This will add `clap` under `dependencies` to your `Cargo.toml` file:

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

The notable parts are the definition of the `Args` struct and the `SubCommand` enum. In the `Args` struct, we refer to
the `SubCommand` enum as a subcommand. This means that we can call our application with `greet` as a subcommand.

The `Greet` struct is a struct that contains the `name` argument. The `name` argument is a string that defaults
to `Unknown` if no value is provided.

In the `main` function, we parse the arguments and match the subcommand. If the subcommand is `greet`, we print the
message.

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

# Release with jReleaser

## Initialize jReleaser

To install jReleaser for your platform, please refer to the [installation guide](https://jreleaser.org/guide/latest/install.html#_stable).

As I am on macOS, I will use Homebrew to install jReleaser:

```bash
brew install jreleaser/tap/jreleaser
```

After the installation, we can initialize jReleaser:

```bash
jreleaser init --format yml 
```

This will create a `jreleaser.yml` file in the current directory. 

Check the config

```bash
jreleaser config -grs
```

## Use GitHub actions




