# Optional arguments of `git_branch_name`

## No arguments

`git_branch_name` will simply result in the current branch name.

## One argument

Only giving it an argument will result in it being used as separator

`git_branch_name(|)` will result in the current branch name sorrounded by `|` character.

There is an exception to this on special cases.

The following characters will autocomplete with their closing variant:

| Character | Autocompletes in |
| --------- | ---------------- |
| `(`       | `)`              |
| `[`       | `]`              |
| `{`       | `}`              |
| `<`       | `>`              |

This means that something such as `git_branch_name([)` will result in the current branch surrounded by `[` and `]`.

## Two arguments

When two arguments are givent they are used for the first and second separator respectively.

For example `git_branch_name(a,b)` will result in the current branch surrounded by `a` and `b`.
