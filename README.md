# fool
A Simple Boolean Algebra Engine

Currently supports simplifying operations involving Not, And, Or, and Xor, as well as the mathematical logic functions implication (`=>`) and biconditional (`<=>`).

## Syntax
Any variable consisting of alphanumeric characters (without a digit at the beginning) and `_` is a valid variable name. 
Note that `t`,`f`,`true` and `false` are keywords. 

The following operations are supported:

- `!` - Not
- `+` - Or
- `*` - And
- `^` - Xor
- `->` - Implication
- `<->` - Biconditional


### Examples
- `!!a` simplifies to `a`
- `!a + b` remains the same as no simplification is possible
- `a * false` simplifies to `false`
- `a -> b` simplifies to `!a + b`
- `!false * !(a ^ b)` simplifies to `!(a ^ b)`
- `var1 + var2 + false * true -> var3` simplifies to `!(var1 + var2) + var3`
- `!var * true ^ b + c ^ (x ^ y -> var2 <-> _3) ^ false * true` simplifies to `var ^ b + c ^ !(!(x ^ y) + var2 ^ _3)`

