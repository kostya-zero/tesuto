# Tesuto Configuration Guide

This guide will help you to configure your Tesuto project.

#### Navigation

- [Getting started](#getting-started)
- - [Create new project](#create-new-project)
- - [Project file](#project-file)
- [Configure project](#configure-project)
- - [Add new job](#add-new-job)
- - [More about actions](#more-about-actions)
- [Running project](#running-project)
- - [Run whole project](#run-whole-project)
- - [Run specific job](#run-specific-job)

## Getting started

#### Create new project

Firstly, you need to initialize new project file for Tesuto.

```shell
tesuto new
```

#### Project file

After initializing project, you will get `tesuto.yml` in your project directory.
By default, it has this structure:

```yaml
name: TesutoProject
jobs:
  hello-job:
    - name: Print 'Hello world!'
      program: echo
      args:
        - Hello world!
```

The `name` field need to make it easier to recognize for which project this configuration was made.

## Configure project

#### Add new job

With new jobs system in 0.2.0 version you can group all action into jobs.
To make new job you need to add new entry to `jobs` section.
Each job contains **list of actions**.

For example, you can left only `name` field and use it as message.

```yaml
...
build-job:
  - name: Building project.
...
```

Let's add new job called `new-job` and add action that will display "Hello again".

```yaml
name: TesutoProject
jobs:
  hello-job:
    - name: Print 'Hello world!'
      program: echo
      args:
        - Hello world!
  new-job:
    - program: echo
      args: 
        - Hello again
```


#### More about actions

Action can contain 3 fields:
- `name` - Text that will display and tell what this action going to do.
- `program` - Program that will be executed.
- `args` - Arguments for the program.

Every field in action is optional.
For example, you can left only `name` field and use it as message.

```yaml
...
build-job:
  - name: Building project.
...
```

This is an example of how actions in jobs can look like:

```yaml
...
  build-job:
    - name: Run cargo build.
      program: cargo
      args:
        - build
        - --release
        - --target=x86_64-unknown-linux-gnu
    - name: Make zip package.
    - program: cp
      args: 
        - target/release/app
        - ./app
    - program: tar
      args: 
        - -cvf
        - app.tar.gz
        - ./app
    - name: Done.
...
```

## Running project

#### Run whole project

Just run `tesuto` with argument `run` to run project.

```shell
tesuto run
```

#### Run specific job

To run specific job, use `run-job` argument and sepcify name of job to run.

```shell
tesuto run-job build-job
```
