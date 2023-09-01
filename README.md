# Tesuto

Tesuto is a minimalist and lightweight tool for testing.
Tesuto designed to be easy to configure and fast as possible.
All jobs are cut into stages.
Each stage have scripts that will be executabe before main scripts, scripts to run in this stage and environment variables.
All of this options are specific for each stages.
At that moment, Tesuto available on Linux only. In future I planned to add support for Windows and macOS.

### Installation

#### From releases

1. Go to the releases section and choose version which you want to install.
2. Download archive and unpack it.
3. Place executable file to the place that exists in `PATH` environment variable.

> If you cant run executable because `permission denied`, run `sudo chmod +x <path_to_tesuto>` to make it possible to run.

#### From crates.io

1. Download `rustup` and install it by following instructions.
2. Run `cargo install --locked tesuto` and wait until end of compilation.
3. Tesuto ready to use.

#### Build from source

1. Download `rustup` and install it by following instructions.
2. Clone this repository and enter it directory.
3. Run `git checkout latest` to use sources from latest version or `git checkout main` to use unstable version.
4. You can run `cargo build` to build executable with debug info or `cargo build --releases` to build version with optimizations.

# Configuration

### Generate new project
After installation, open terminal and enter directory with yout project where you want to use Tesuto.
Next, we need to create new project file:

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

Now we have new stage in our project. 
Let's add new command to stage.

We are going to edit `script` option.
In YAML to add new string to array, we use `-` prefix for it.
Let's add `echo` again and now it will print "Hello again!":

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

But why we doesn't add `-` prefix? 
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

