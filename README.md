# Conus
A cellular automata diagram generator

The name is based on the first thing I saw on the [Rule 30](https://en.wikipedia.org/wiki/Rule_30) Wikipedia page

# How to use it

```
USAGE:
    conus [OPTIONS] <RULE> <ITER>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o <FILE>                Output file
    -f, --output <FORMAT>    Output format (ascii,png)

ARGS:
    <RULE>    Sets the rule number
    <ITER>    Number of iterations
```

## Example

One hundred iterations of the rule 30
```bash
conus 30 100 > rule30.txt
```

# Plans
Right now it generates the patterns starting with one cell in the center of the first state, one of my plans is to allow one to be able to provide an initial state.
