---
title: "Houdini Node"
slug: "houdini-node"
abstract:
  "Rust crate / macro that allows implementing a SOP node for SideFX Houdini entirely as a standalone program."
date: "2025"
---

The source code for this project can be found at  
<https://github.com/luxalpa/houdini-node>

## Motivation

For my game, I wanted to test certain physics related libraries and functions. I like using Houdini as a debugging
tool (and editor). So the idea was to have inputs coming from Houdini itself, then my own could would generate and
animate some Geo, then send it back to Houdini so I could study the results.

## Implementation

The project was fairly straight forward to build. There is a Python-based SOP node that collects the input geometry with
its attributes, transforms it into JSON, and then calls the user-provided executable, passing this JSON object in. In
the Rust code for the executable, all the inputs and geometry types are specified in a type-safe way, and a proc macro
handles the serialization and deserialization process for various types (like matrices, vectors, strings, etc). Then it
serializes the result back into JSON, which is then parsed by the former Python script to present the result in Houdini.

## Future

The project is pretty much complete. I am using it internally for testing various components on my game individually, in
particular the complex interactions required for my creatures procedural animations.

