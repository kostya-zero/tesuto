# CHANGELOG

## Next

TBD

## v0.2.0

- Now the Tesuto source code is not split into multiple projects, but is located in a single project.
- Completely changed the structure of the project in Tesuto. More details in this README section.
- Completely remodeled runner.
- The program will now be run through a shell depending on the operating system (`cmd` on Windows and `bash` on other OS). This option cannot be configured yet.
- The scenarios have been deleted.
- You can no longer add new job to a project via the `add` command.
- Added `require` field: you can specify which programs are needed to execute this project.
- Added `list` command to display all available jobs.
- Added `quite` option for steps to disable program output.
- Improvements to the code to improve performance.

## v0.1.1

- Fix wrong stages order when running a project.

## v0.1.0

First release of Tesuto.
