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
stages:
- name: hello
  script:
  - echo "Hello World!"
  variables: []
  quite: false
```

The `name` field need to make it easier to recognize for which project this configuration is set.

## Configure project

#### Add new stage

To add new stage to project, run Tesuto with argument `add` and provide name for new stage:

```shell
tesuto add build
```

#### Add script to stage

Open `tesuto.yml` in your favorite text editor and find section with stage that you want to change.
Add new item to `script` section by adding new line and start it with `-`.

```yaml
...
- name: build
  # script is an array, so to add new item it must start with `-`.
  script: 
  - cargo build --release
  variables: []
  quite: bool
```

#### Add environment variables

If you still have `tesuto.yml` open - great. 
To add new variable you need to add new item to `variables` field with `name` and `value` fields.

```yaml
  ...
  # variables, same as script, is an array, so when
  # we adding new item we must start it first field with `-`.
  variables: 
    - name: HELLO_PHRASE
      value: hello
    - name: WORLD_PHRASE
      value: world
  ...
```

#### Run stage without output

You can set option `quite` to true to get no output from stage when it running.

```yaml
  ...
  quite: true
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
