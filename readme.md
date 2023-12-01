# Advent Of Code 2023

Going for 50 stars!

## Usage

Login to [adventofcode](https://adventofcode.com), open the dev tools, and retrieve your session cookie.

Then, create a `.env` file and add it like so:

```
ADVENT_OF_CODE_SESSION=<session>
```

Then, because you're using [fish](https://fishshell.com), source the tasks file.

```shell
source tasks.fish
```

Or, if you want to make your life easier, add this to your fish config so that task files are sourced automatically:

```shell
if cat tasks.fish &>/dev/null
	source tasks.fish
end
```

Then, run the exercises for a particular day.

```shell
.run 23
```
