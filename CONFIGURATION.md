# Configuration
How to use Tesuto projects and make automated workflows.
## Projects
Tesuto uses projects as configuration. To create an example project, you can run `tesuto new`. Tesuto will generate a new project file and place it in the current directory with the name `tesuto.yml`. As the extension suggests, the file contains YAML-structured data. I've chosen YAML to make Tesuto projects more familiar with GitHub Actions workflows.
## Initialize new project
To initialize new project use `new` subcommand to run initialization wizard:
```shell
tesuto new
```
The initialization wizard will ask you 2 questions:
- **How you want to name your project?** This name is used to easily identify which project you are running.
- **Do you want to use example project?** If you type Y, it will generate an example project. This is recommended for new users to understand the structure of a project.
## Structure
As was said before, Tesuto uses YAML syntax for projects. The structure for project is pretty simple (especially if you deal with GitHub Actions workflows):

```yaml
name: TesutoProject
require: []
jobs:
  hello:
  - name: Print 'Hello world!'
    run: echo "Hello World!"
    quite: false
```
In the code block above you can see an example project. If you choose to not generate example project, you will see an empty one like this:
```yaml
name: TesutoProject
require: []
jobs: {}
```
In the root of project there is 3 fields:
- `name` - Used to identify project.
- `require` - List of programs that required to run this project.
- `jobs` - Actions that Tesuto need to do.
We will skip `name` field because it's obvious what it is purpose.
## Required Programs
In the `require` field you can specify which programs are required to run this project:
```yaml
...
require:
	- git
	- cargo
...
```
It's a list of program that will be found through search in PATH. If one of this programs are not found, Tesuto will crash and tell you which program is not found.
## Jobs and Steps
Jobs contain steps that Tesuto needs to perform to complete a job. Every job in a project will be executed step-by-step. The syntax of a job looks like this:
```yaml
...
jobs:
  cargo:
  - name: Build release binary
    run: cargo build --release
    quite: false
...
```
Step has 3 properties:
- `name` - The name of the step. If it's empty, it will be replaced with the command that the step should run.
- `run` - The command to run. If this field is empty, Tesuto will display only the name of the step. If both name and run fields are empty, Tesuto will skip the step.
- `quite` - Whether to display the output or not.
You can create as many jobs and steps as needed. There are no limits for your workflow.
## Running project
You can run your project with ease by using `run` command:
```shell
tesuto run

# If your project is in different location.
tesuto run --project "configs/tesuto.yml"
```
Also, you can run specific job with `run-job` command:
```shell
tesuto run-job cargo
```
