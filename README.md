# stak

A simple command line based RPN calculator

## Usage

stak can be used in two modes: one-shot from the command line, or in an interactive shell

### One-shot

In one-shot mode, include the sequence of values and operators you wish to evaluate as arguments to the binary. Note that it may be required to put the arguments in quote marks depending on your shell.

```
$ stak 3 7 * 4 /
[5.25]
$ stak 12.6 17 / .5 3 ^
[0.7411764705882353, 0.125]
```

### Interactive

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



## Mathematical Operators

The following operations are currently supported

| Operation                            | Symbol      |
| ------------------------------------ | ----------- |
| Addition                             | `+`         |
| Subtraction                          | `-`         |
| Multiplication                       | `*`         |
| Division                             | `/`         |
| Modulus                              | `%`         |
| Power                                | `**` |
| Parallel sum                         | <code>&#124;&#124;</code> |
| Inversion                            | `inv`       |
| Square root                          | `sqrt`      |
| Log<sub>2</sub>                      | `log2`      |
| Floor                                | `floor`     |
| Ceiling                              | `ceil`      |
| Absolute value                       | `abs`       |
| Summation of all values in the stack | `sum`       |
| Product of all vales in the stack    | `prod`      |



## Stack Operators

The following operations can be performed on the stack

### Clear `..`

Clears the stack

Example:

```
$ stak
> 3 4 5 6
[3.0, 4.0, 5.0, 6.0]
> ..
[]
```

### Pop `.`

Removes the top value from the stack

Example:

```
$ stak
> 3 4 5 6
[3.0, 4.0, 5.0, 6.0]
> .
[3.0, 4.0, 5.0]
```

### Swap `<>`

Swaps the top two values of the stack

Example:

```
$ stak
> 3 4 5 6
[3.0, 4.0, 5.0, 6.0]
> <>
[3.0, 4.0, 6.0, 5.0]
```

### Duplicate `~`

Duplicates the top value of the stack. An optional index can be specified, duplicating the value at that value. Stack indexes are zero-based and originate at the top of the stack.

Example:

```
$ stak
> 3 4 5 6
[3.0, 4.0, 5.0, 6.0]
> ~
[3.0, 4.0, 5.0, 6.0, 6.0]
> ~3
[3.0, 4.0, 5.0, 6.0, 6.0, 4.0]
```

### Rotate Left `<<`

Rotates the stack to the left. An optional number of rotations can be specified

Example:

```
$ stak
> 3 4 5 6 7
[3.0, 4.0, 5.0, 6.0, 7.0]
> <<
[4.0, 5.0, 6.0, 7.0, 3.0]
> <<2
[6.0, 7.0, 3.0, 4.0, 5.0]
```

### Rotate Right `>>`

Rotates the stack to the right. An optional number of rotations can be specified

Example:

```
$ stak
> 3 4 5 6 7
[3.0, 4.0, 5.0, 6.0, 7.0]
> >>
[7.0, 3.0, 4.0, 5.0, 6.0]
> >>2
[5.0, 6.0, 7.0, 3.0, 4.0]
```



## Constants

The following mathematical constants are currently supported

- `pi`
- `e`

Example:

```
$ stak
> 2.5 2 ^ pi *
[19.634954084936208]
```

