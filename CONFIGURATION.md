# Tesuto Configuration Guide

This guide will help you to configure your Tesuto project.

#### Navigation

- [Getting started](#getting-started)
- - [Create new project](#create-new-project)
- - [Project file](#project-file)
- [Configure project](#configure-project)
- - [Add new stage](#add-new-stage)
- - [Add script to stage](#add-script-to-stage)
- - [Add environment variables](#add-environment-variables)
- - [Run stage without output](#run-stage-without-output)
- [Running project](#running-project)
- - [Run whole project](#run-whole-project)
- - [Run specific stage](#run-specific-stage)

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

The `name` field need to make it easier to recognize for which project this configuration is made.

## Configure project

#### Add new job

With new jobs system in 0.2.0 version you can group all action into jobs.
To make new job you need to add new entry to `jobs` section.
Each job contains **list of actions**.
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

You may have question - why didn't we add `name` field to the action in new job?
Because it's optional, like every field in action. 

Also, you can use only `name` field to display information if you need. E.g.:

```yaml
...
  prepare-job:
    - name: Checkout branch
      program: git
      args:
        - checkout
        - origin/main
    # Like this one:
    - name: Prepare finished.
...
```

#### Dive into actions

As was said before, jobs is a list of actions that Tesuto need to run.
Each action can contain name of program to run and arguments in `program` and `args` fields.
If `args` field set but `program` not, Tesuto will ignore it both and display `name` field (if it's set).

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

#### Run specific stage

To run specific stage, use `run-stage` argument and sepcify name of stage to run.

```shell
tesuto run-stage build
```
