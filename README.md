# Reverse Polish Notation for Polynomials over one variable

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
```

The program correctly outputs

```
Parse Error
0 2 x * + 1 /
NAN
```
