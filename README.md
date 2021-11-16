# stak
A simple command line based RPN calculator

### Usage

stak can be used in two modes: one-shot from the command line, or in an interactive shell

#### One-shot

In one-shot mode, include the sequence of values and operators you wish to evaluate as arguments to the binary

```
$ stak 3 7 * 4 /
[5.25]
$ stak 12.6 17 / .5 3 ^
[0.7411764705882353, 0.125]
```

#### Interactive

To start stak in interactive mode, simply run the program with no arguments. Values and operators can be entered one at a time or on one line with spaces separating them

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

| Operation       | Symbol | Args |
| --------------- | ------ | ---- |
| Addition        | `+`    | 2    |
| Subtraction     | `-`    | 2    |
| Multiplication  | `*`    | 2    |
| Division        | `/`    | 2    |
| Modulus         | `%`    | 2    |
| Inversion       | `inv`  | 1    |
| Square root     | `sqrt` | 1    |
| Log<sub>2</sub> | `log2` | 1    |

