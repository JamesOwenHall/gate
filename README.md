# Gate

[![Build Status](https://travis-ci.org/JamesOwenHall/gate.svg?branch=master)](https://travis-ci.org/JamesOwenHall/gate)

A dynamically typed interpreted language written in Rust.

## Usage

To run a program, pass it as an argument to the Gate interpreter.

```
$ gate hello_world.gate
Hello world!
```

There's also a REPL available.

```
$ gate -i
> 5 + 6.6
Number(11.6)
```

## Syntax

### Types

Gate supports basic types such as nil, booleans, numbers and strings.

```
> nil
Nil
> false
Boolean(false)
> true
Boolean(true)
> 3
Number(3)
> -2.3
Number(-2.3)
> "foo bar"
Str("foo bar")
```

### Variables

Variables are assigned using the `=` operator.

```
> x = 32
Number(32)
```

Since variable assignment is an expression, they can be chained together.

```
> a = b = c = 2
Number(2)
> a
Number(2)
> b
Number(2)
> c
Number(2)
```

### Control flow

Loops and if statements form the building blocks of control flow in Gate.  Note that they too are expressions.

```
> if x == 12 { "It's 12" } else { "Definitely not 12" }
Str("Definitely not 12")
> while x < 3 { x = x + 1 }
Number(3)
```

### Blocks

Blocks allow you to evaluate multiple expressions.  They evaluate to the value of the last expression.  This allows you to compose multiple expressions in interesting ways.

```
> { 0 true "foo" }
Str("foo")
> x = { b = 1   if b == 1 "it's 1" else "not 1"}
Str("it\'s 1")
```
