# stak
A simple command line based RPN calculator

### Usage

stak can be used in two modes: one-shot from the command line, or in an interactive shell

#### One-shot

In one-shot mode, include the sequence of values and operators you wish to evaluate as arguments to the binary. Note that it may be required to put the arguments in quote marks depending on your shell.

```
$ stak 3 7 * 4 /
[5.25]
$ stak 12.6 17 / .5 3 ^
[0.7411764705882353, 0.125]
```

#### Interactive

To start stak in interactive mode, simply run the program with no arguments. Values and operators can be entered one at a time or on one line with spaces separating them. Type `help` or `?` for a list of operators.

```
$ stak
> 3
[3.0]
> 4
[3.0, 4.0]
> 13.7 * 1.3 /
[3.0, 42.15384615384615]
> *
[126.46153846153845]
```



### Operators

The following operations are currently supported

| Operation                            | Symbol      | Args |
| ------------------------------------ | ----------- | ---- |
| Addition                             | `+`         | 2    |
| Subtraction                          | `-`         | 2    |
| Multiplication                       | `*`         | 2    |
| Division                             | `/`         | 2    |
| Modulus                              | `%`         | 2    |
| Power                                | `^` or `**` | 2    |
| Parallel sum                         | <code>&#124;&#124;</code> | 1+   |
| Inversion                            | `inv`       | 1    |
| Square root                          | `sqrt`      | 1    |
| Log<sub>2</sub>                      | `log2`      | 1    |
| Floor                                | `floor`     | 1    |
| Ceiling                              | `ceil`      | 1    |
| Absolute value                       | `abs`       | 1    |
| Summation of all values in the stack | `sum`       | 1+   |
| Product of all vales in the stack    | `prod`      | 1+   |
| Pop the top value                    | `.`         | 0    |
| Clear the stack                      | `..`        | 0    |
| Duplicate the top value              | `&`         | 1    |
| Swap the top two values              | `<>`        | 2    |

### Constants

The following mathematical constants are currently supported

- `pi`
- `e`

Example:

```
$ stak
> 2.5 2 ^ pi *
[19.634954084936208]
```

