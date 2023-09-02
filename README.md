# Tesuto

<img src="imgs/logo.svg" align="right" width="128px">

Tesuto is a minimalist and lightweight tool for testing.
Tesuto designed to be easy to set up and be as fast as possible.
Tesuto allows developers to focus on solving deployment problems.

## Installation

#### From releases

1. Go to the releases section and select the version you want to install.
2. Download archive and unpack it.
3. Place executable file to the location that exists in  the `PATH` environment variable.

> If you cant run executable with error `permission denied`, run `sudo chmod +x <path_to_tesuto>` to make it possible to run it.

#### From crates.io

1. Download `rustup` and install it by following instructions.
2. Run `cargo install --locked tesuto` and wait until end of compilation.
3. Tesuto ready to use.

#### Build from source

1. Download `rustup` and install it by following instructions.
2. Clone this repository and enter it directory.
3. Run `git checkout latest` to use sources from latest version or `git checkout main` to use unstable version.
4. You can run `cargo build` to build executable with debug info or `cargo build --releases` to build version with optimizations.

## Configuration

### Generate new project
After installation, open a terminal and enter the directory with your project in which you want to use Tesuto.
Next, you need to create a new project file:

```shell
tesuto new
```

### Structure

Tesuto generates default project and save it as `tesuto.yml`.
It will look like this:

```yaml
name: TesutoProject
stages:
  hello:
    before_script: []
    script:
    - echo "Hello World!"
    variables: {}
    quite: false
```

You have 2 options: `name` and `stages`. `name` is a name for youe project. `stages` is where you will write every stage for your project.

You can get a list of stages in project with `tesuto list`.

### Stage structure
Let's take a look at options for `hello` stage.

- `before_script` - commands that will be executed before main scripts will. You can leave it empty.
- `script` - commands for this stage. It's cant be empty.
- `variables` - environment variables for each script (including `before_script`). 
- `quite` - show output or no. By default it's `false` - show all output.

### Configuring new stage

Use command `add` and pass name to create new stage with specific name. 
For example, let's create new stages called `spam`.

```shell
tesuto add spam
```

Now we have a new stage in our project. 
Let's add a new command to stage.

To do this, let's edit `script` option.
In YAML, to add a new string to an array, we use `-` prefix.
Let's add `echo` again, but now "Hello again!" will be printed:

```yaml
  spam:
    before_script: []
    script:
    - echo "Hello World!"
    # New command in stage.
    - echo "Hello again!"
    variables: {}
    quite: false
```

We can also try to add environment variable in `variables` option.
Syntax for variables:
```yaml
    VARIABLE_NAME: "variable value"
```

Let's add `HELLO` variable with `Hello` value and try to print it via last `echo` command.

```yaml
  spam:
    before_script: []
    script:
    - echo "Hello World!"
    # We are adding $ sign beacuse it's an environment variable.
    - echo "$HELLO again!"
    variables: 
        # New variable
        HELLO: "Hello"
    quite: false
```

But why don't we add the `-` prefix? 
Because it's not an array. 
It's an option.

Also we can set quite mode for stage with `quite`. I think his syntax looks similiar for you.

### Run project

To run project use `run` command.

```shell
tesuto run
```

Tesuto will automatically give a sign if there's something wrong happen.

Also, you can run specific stage with `run-stage` command.

```shell
tesuto run-stage hello
```

## Project layout
```
├─ imgs/           Images for README.
├─ target/         Build directory
├─ tesuto/         App source code.
└─ tesuto_project/ Library that contains project structure for Tesuto.
```
