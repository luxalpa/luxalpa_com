---
title: "Rise of the Behemoth (Game)"
slug: "rise-of-the-behemoth"
abstract:
  "A 3D video game in which the player controls a giant monster with a custom Vulkan-based engine. Currently about 
  16,000 lines of Rust code."
date: "2022 - 2025"
---

A small (slightly outdated) video of this project: <https://youtu.be/ZevczcjQ6nw>

## Motivation

I am a huge fan of giant monsters, dragons, dinosaurs, etc. I always wanted to play a game where you could just play
one. As a kid I used to play SimCity for hundreds of hours, watching the giant monster destroy the city again and again
and always wanting to have more control. At some point, I played Godzilla: Save the Earth, for which I ended up building
modding tools (as well as fixing the Playstation 2 emulator to support the game) and modded the game a lot. However, I
realized that all modding tools in the world could not fix the limited engine. And so over the years I started to learn
the tech and art skills to make my completely own game.

I originally started this project in Unity. I wanted the monsters controls to be more precise, being able to move
individual legs, and completely avoiding foot sliding and clipping. However, at the time my experience with 3D was just
too limited. In the end I took a slight detour and learned SideFX Houdini, a 3D package that allows playing around with
proceduralism and skeletal animations, inverse kinematics, and much more. This turned out to be the exact right move
because it introduced me to the right tools for solving my earlier problems, and the mathematics (such as trigonometry)
that I would need later.

So in 2022 I restarted working on this project in Unreal Engine. I got to quite a few things, but in the end I was
unhappy with the engine. Personally, I don't mind C++, in fact I love it. But Unreal was very overcomplicated and if you
hadn't had a lot of experience in it, it would take forever to understand why things weren't working the way they
should. And lots of the functions I needed (such as physics on bones) were very bugged. I wanted to have something more
lightweight - especially since I knew I wanted to have thousands or even millions of objects on the screen, and for that
I needed something that I could optimize the hell out of. I quickly took a look at Bevy, which at the time looked
promising, but it also lacked many important features. For example, it couldn't do more than a thousand or so entities
on the screen and didn't have support for particle systems. It also didn't support skeletal animation either.

And I wanted to really learn and understand the software that I was writing, so I was very intrigued about learning
Vulkan. I had used OpenGL before in an older project (about 10 years before) and written a complete graphics engine (
with shadows and everything) in it. So in the end my first real Rust project became this Vulkan game engine, roughly
following the guide from [ashen-aetna](https://hoj-senna.github.io/ashen-aetna/).

## Implementation

### Graphics

My custom Vulkan engine supports Dithered cascaded shadows, PBR textures and environment mapping. Getting the
synchronization primitives to work properly was a bit of a challenge, especially with resizing and minimizing the
window, as most guides and tutorials on the web on this matter are wrong. It was a particularly "fun" surprise when I
switched out my nvidia graphics card with a new one from AMD and suddenly certain things stopped working and needed to
be corrected!

### Animations

The main work was done on the animation system which is still work in progress. Originally I used a physics library
called Rapier to handle the tail physics, which worked great (after they fixed some of the bugs I reported), but I
wanted to use it for the legs, and it couldn't do that. In fact, I built my own full-body-IK system roughly following
the reference implementation from Unreal Engine (except their version was bugged because their devs didn't realize that
their matrices were the wrong way around). It was a fun challenge but failed for the leg IK of the monster because I
couldn't get orientation constraints behave well together with position constraints (maybe I'll fix it some time later).

The full body IK in the end turned out to be very useful however, since I was able to use it in order to do the lookat
rotation for the monsters head. I had to invent my own lookat constraint (since this wasn't in the reference
implementation or the paper), and was pleasantly surprised that it was this easy to get working! So the head now uses
Rapier physics joint motors for general animation and reaction, and uses this FBIK lookup to set the goal orientations
of those joint motors.

The legs went through a more difficult phase. Originally, I had used a fully procedural analytical solution using curves
and several 2-bone-ik systems (which have exact solutions and don't run into the same issue as the full body IK). I used
a second order dynamics system to simulate and interpolate between the animation stages. However this system is
currently being reworked in favor of a more transition based approach - basically I animate a walk in Houdini, export
the animation curves and then transition between those curves.

A big question that came up all the time was whether I'd really need this high level of precision on the animations.
After all, many games work well with some amount of foot sliding. The issue is that in my game, I want soldiers to
directly attack and interact with the giant monsters toes. So the toes need to be not moving around needlessly else they
will break the actual gameplay.

I might also switch out Rapier physics with nvidia's Physx library, for which I wrote a complete abstraction layer,
because I was a bit unhappy with the support and feature set that I got on Rapier.

### World generation

The original idea was to have the game world be entirely generated and simulated. First, because it's just fun, and
second, because I wanted the cities to be able to rebuild after the monster rampages through them. I originally looked
at (and reverse engineered) the various [town generators from Watabu](https://www.reddit.com/r/FantasyCities/) and some
papers about L-Systems and learned some useful primitives from them. But they were not well suitable for dynamically
evolving (or rebuilding) a town from the inside. In the end, this information was useful though, because it allowed me
to reverse engineer the city generation algorithm used
by [Transport Fever 2](https://store.steampowered.com/app/1066780/Transport_Fever_2/), which I currently use in a
simplified form. It's basically an L-System splitting like approach, in which you take roads (more precisely individual
sides of roads or "half-edges" on the graph) and randomly split them if they are very long, creating an orthogonal road.
My current system does not feature curves, so it looks very grid like, but that's just because I haven't come around to
implement proper collision handling (the generated roads need to know if they intersect with something), but it would be
relatively easy to implement.

I did create a full geometry crate though in order to handle all kinds of intersections on geometrical objects,
particularly for splitting 2D polylines and polygons.

### Houdini

In order to get a proper debugging experience with a 3D engine like this, I quickly figured out that I needed to analyze
the outputs statically. I created my houdini-debug-logger crate which allows exporting all kinds of geometry directly
via a Houdini Engine implementation directly into my editor. This simplified development on the procedural animations
extremely, and it also was very useful for debugging the city generation algorithms.