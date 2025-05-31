# Reverse Polish Notation Calculator for Polynomials over one variable

E -> E E + | E E - | E E \* | E E / | E d | E n ^

E -> x | n | n.PPPPP

P -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9

In this version it is not enforced that the floating number must have 5 digits after the decimal point

The problem statement can be found [here](./project.pdf)

The calculator has the functionality of adding, subtracting, multiplying, exponentiating, dividing, and differentiating
polynomials over one variable **x**

For the input:

```
1 x x 2 ^ + 1 x +
x 2 ^ 1.00001 + d
3 2 3 - 1 + /
2 x 2 ^ * 2 /
```

The program correctly outputs

```
Parse Error
2 x *
NAN
1 x 2 ^ *
```

Reduces the polynomial if the denominator is a non zero constant polynomial

Returns `Parse Error` if error in parsing

Returns `NAN` if any attempt at division by zero
