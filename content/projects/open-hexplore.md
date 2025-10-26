---
title: "Open Hexplore"
slug: "open-hexplore"
abstract:
  "A decompilation project to mod and improve the 1998 video game Hexplore."
date: "2020"
---

This project can be found on github at  
<https://github.com/luxalpa/openhexplore/>

## Motivation

I have spent a large fraction of my childhood playing video games, one of them
was [Hexplore](https://en.wikipedia.org/wiki/Hexplore). The game always intrigued me because there were so many puzzles
and it was made with exploration in mind while at the same time creating a really nice and exciting atmosphere.

Since about that long, I dreamed of having access to a level editor for this game. Unfortunately, there was no official
one. I attempted a project at a level editor in 2010, which was abandoned due to the complexity of the task (mostly I
didn't yet have full understanding of OpenGL at the time).

Fast forward to 2020 I really wanted to find out how the game worked. I played it over network with my friend (who I
already had played this game with 20 years earlier), and we noticed that the game - although it was still fun - could be
using a lot of modern improvements. I took a look at the GoG version of the game, but unfortunately it didn't really
modernize the experience.

For the most part, this project is just an adventure. I don't know how far it will go, I am just doing it because
researching how this game was made is tons of fun, and there is the potential of creating a great remake out of it.

## Implementation

I have played OpenTTD and OpenRCT2 before, and decided that this was a very nice way of approaching decompilation. The
way it works in those games is, they essentially just compile a library with some of the decompiled functions, and then
have external function calls that jump back into the original binary for functions that have not yet been decompiled.
The compiled library, in my case `OpenHexplore.dll` is injected into the original .exe via a new entry in the IAT and
the first instruction in the original `Hexplore.exe`s `WinMain()` function is overwritten with a `jmp` instruction that
jumps straight to my exported function in the .dll.

I would have loved to write the project in Rust, but I'm unfamiliar in Rust and IDA Pro only outputs C-Pseudocode, so I
went with C++ for now. I might port it later.

The rest of the project was to just decompile the original game function by function, adjusting Struct definitions and a
lot of improvements on the decompiled code so that it looks pretty and can be understood easily. I already got quite
far, decompiling the entire sound engine and the game start until the first intro (stand 2020-05-27). The process isn't
too difficult, and the game has maybe about 1000 functions that need to be decompiled.

## Future

It is unclear whether or not I will decompile the entire thing. It is possible that I will only decompile the functions
that read the different proprietary file types, and then build my own project from it in Unity. It is a side project
that currently doesn't have a lot of focus, but I'm using it to teach one of my friends how to program in C++.

The most important goal would be to have the game run on native, widescreen resolutions, to improve the hotkeys and the
general user interface, among other things. A more sophisticated approach sees a complete remake of the game in Unity3D
with 3D graphics, an improved path finding system and modding support.
