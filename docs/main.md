# Syntax

## Function

A function is the identifier for what you're executing:

Ex: `text_color`, `git_branch_name`

## Arguments

Arguments are data that gets passed to a function

Ex: `text_rgb(37, 52, 81)`

`37`, `52` and `81` are the arguments

`text_rgb` is the function

A function may have optional arguments and some may not require them at all.

### Note

To put a space in an argument use `\%s`

and to put a tab use `\%t`

So `exec(printf, hello world)` should be written as:

`exec(printf, hello\%sworld)`

And `exec(printf, hello\tworld)` should be written as

`exec(printf, hello\%tworld)`.

## Task

A task is the function and its arguments (if the function requires them)

Ex: `git_branch_name` and `text_rgb()` are bot tasks.

## Sub

A sub is described as one or more tasks enclosed in `%[]`

Ex: `%[text_rgb(37,52,81);git_branch_name]` is a sub

`%[user]` is also a sub.

### Note

Tasks in a sub can be divided both using `;` and `,`.

So `%[func1;func2]` and `%[func1,func2]` are both valid. 

---

# Available functions

This is a list of currently available functions, their aliases and the arguments they accept.

*Note*: Arguments marked with a `?` are considered optional.

*Note*: Arguments marked with a `*` are considered variadic, this means that you can put one or more of them.

| Name                                 | Aliases                               | Arguments                             | Description                                                                     |
| ------------------------------------:|:-------------------------------------:|:-------------------------------------:|:------------------------------------------------------------------------------- |
| `reset`                              |                                       |                                       | Reset color and effects                                                         |
| [`text_color`](./colors.md)          | `color`                               | `Color`                               | Give text the desired color                                                     |
| [`text_rgb`](./rgb_colors.md)        | `rgb`                                 | `R`, `G`, `B`, ?`Bold`                | Give text the desired rgb color                                                 |
| [`background_color`](./colors.md)    | `background`                          | `Color`                               | Give background the desired color                                               |
| [`background_rgb`](./rgb_colors.md)  |                                       | `R`, `G`, `B`                         | Give background the desired rgb color                                           |
| [`effect`](./effects.md)             |                                       | \*`Effect`                            | Set the desired effects                                                         |
| `get_user`                           | `user`, `username`                    |                                       | Get username (USER env var)                                                     |
| `get_host`                           | `host`, `hostname`                    |                                       | Get hostname (contents of /etc/hostname)                                        |
| [`current_working_dir`](./cwd.md)    | `cwd`, `pwd`                          | ?`Expand`                             | Get current working directory                                                   |
| [`git_branch_name`](./git_branch.md) | `branch`, `git_branch`, `branch_name` | ?`Open separator`, ?`Close separator` | Get current git branch name (if in a git repo)                                  |
| `exec`                               |                                       | `Command`, ?\*`Arguments`             | Execute command and print output                                                |
| `exec_strip`                         |                                       | `Command`, ?\*`Arguments`             | Execute command and print output forcing the removal of any trailing whitespace |



---



# Environment variables

You can change some settings trough the use of environment variables



| Env var                       | Description                                                                                                    |
| -----------------------------:|:-------------------------------------------------------------------------------------------------------------- |
| `GALLIUMPROMPT_NO_WARNING`    | If this variable is set, gallium will suppress every warning message (fatal error ones will still be displayed)|
| `GALLIUMPROMPT_SINGLE_THREAD` | If this variable is set, gallium will only use one thread (which is not default behaviour)                     |



## Note

Single thread could boost performances in only a couple of cases:

* When nothing spawned by `exec` or `exec_strip` takes a high amout of time to complete execution;

* When the system has a low amount of CPU cores.



---





# Usage

Simply pass the template string as an argument to gallium.

Ex: 

```bash
gallium "%[text_color(bold_yellow)] Current dir: %[text_color(bold_red);cwd;reset]"
```
