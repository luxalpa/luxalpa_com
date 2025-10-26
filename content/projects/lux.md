---
title: "The Lux Programming Language"
slug: "lux"
abstract:
  "My current attempt at creating a new programming language. It supports generics and features inferred and deferred types. It currently transpiles to JavaScript."
date: "2017"
---

The source code for this project can be found on Github at  
<https://github.com/luxalpa/lux-ts>

## Motivation

My thoughts about creating my own custom programming language have started as early as 2012, where I started noting down
feature requests and what issues I had with other languages like C++ (really only with C++ in the beginning). The idea
grew and grew and went through many different stages. After a series of excellent video tutorials by stanford professor
Alex Aiken I installed Bison and YACC and attempted to create my own first compiler. It took a while for me to realize
that those tools were quite dated. I tried my hands at handwriting the parser but it really was quite difficult. I
played around with LLVM and nearley.js and eventually found the
awesome [Antlr project](https://www.youtube.com/watch?v=q8p1voEiu8Q). This made my work much easier, however it still
required multiple iterations (first in Go, later in TypeScript) until I finally nailed the concept in 2017.

## Implementation

The key to get the compiler working was to split it into different phases that were independent from each other:

1. Turn the input stream into a parse tree via Antlr by providing a custom Grammar for tokenization and parsing.
2. Turn the parse tree into an Abstract Syntax Tree (AST) for my language using the visitor pattern.
3. Run a type checker over the AST to create type mappings. This sounds easier than it is in practice because the type
   checker must use multiple different stages in order to resolve inferred and deferred types.
4. The transpiler transforms the Lux AST (with type information from the type checker) into a JavaScript AST.
5. Run `astring` to generate JavaScript code from the JS AST.
6. eval this code (mostly just to see how it works).

The advantage to this lengthy process is that I can add in one thing after another fairly easily. Whenever there's
something not implemented in a step, the compiler will run until that point at which it tells me "hey, this node has not
yet been implemented", and I can implement it and improve the code very organically (without a lot of pre planning
needed).

At the time of writing Lux supports inferred and deferred types, inheritance (currently disabled), generics and a
rudimentary support for pointers. There are still tons of todos though, many operators have not been implemented yet,
but it is already a nice proof of concept.

*As a fun fact, almost the entirety of the project (which is one of my largest so far) has been written on the commute
to work in the berlin subway trains (where I had to change trains twice and only had 10 consecutive minutes to work
before changing trains again)*

## Future

The work on this project has currently been suspended as I am not sure whether I want it to support Object Oriented
Syntax (like classes) and in which manner. In addition, I want to research more into other programming languages such as
Haxe and Rust. Possibly it would be easier to just modify those to my liking, or maybe they are already mostly to my
liking, or I could just write a compiler frontend for those languages (while keeping the backend).

The project will most likely be completed at some point and I have many good ideas, but right now it is not a priority. 
