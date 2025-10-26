---
title: "Dynamaxed"
slug: "dynamaxed"
abstract:
  "A project to simplify modding of the Pokemon Series games by providing a graphical editor and abstraction for the Pokemon Reverse Engineering Toolkit (PRET)."
date: "2019"
---

This project can be found on github at  
<https://github.com/luxalpa/dynamaxed/>  
<https://github.com/luxalpa/dynamaxed-emerald/>

## Motivation

In the autumn of 2019 my younger brother asked me how to make his own Pokemon Games. After fiddling around with the
given tools (Advance Map, EliteMap, among others), I realize that in this space a proper editor is missing. I had plans
for a more modern version of Advance Map since at least 2010. The biggest issue with the existing tools was that they
all relied on users to find free spaces in memory and modify the ROM file in place. And this proved to be very
problematic, because the heuristic for finding free memory wasn't perfect, and working with this inplace editing without
any kind of undo functionality was a risky endeavor where one small mistake could destroy all your efforts.

In 2010 therefore I had an idea for a new project: Take a ROM file, extract all the modifiable data into XML files which
constitute a project. Then, you could work on those XML files with any kind of editor and in the end you'd just hit
compile and it would rebuild the ROM using those files. I even had a name for it already: Arctic Map Editor, it would be
written with Qt and look very cool (in my head). But I never even started work on it. Other projects at the time and my
losing interest in the Pokemon Series games would mean it would take till 2019 when I actually followed suit on that
premise.

## Implementation

In 2019 I thought about what programming language to write such a tool in. C# would have been ideal since at that time I
didn't really know a lot about it and I could have used this project in order to improve my C# skills to work on Unity3D
projects later (for which C# is the ideal language). But C# doesn't really have a lot of options for User Interface. It
turned out that the most popular ones were WinForms and WPF, but both of these were Windows only and I wanted it to be
cross platform. Anyway, I settled with WPF for now (with the idea to change it later to an Open Source frame work once I
got more comfortable with the language and the frameworks), and started writing a bit of code.

For those unfamiliar with modding ROMs, it mostly comes down to finding offsets to memory locations at which the data is
stored, and then either modifying said data or the offset to point to new data at a different place in the ROM. In other
words, I needed to know what I wanted to edit, and I needed to know what the offsets where, were I could find the data.
My brother wanted the ability to rebalance the trainers, so that's where I wanted to start. I looked in the common tools
I had (most of them part of the EliteMap tool kit) but it turned out none of them had actual trainer data for the ROM
that I wanted to work on (the offsets are different for different language versions of the game as well as the different
games). When I researched this data on the Internet I found a mysterious project called PRET. It turned out that someone
already went through all the effort of decompiling the games. And not only that, with Porymap, they even had their own
map editor already and it was miles ahead from Advance Map (in terms of mapping capabilities at least).

In some sense this was like a dream come true, but in another way, this made my original project idea useless. If there
was already a way to build the entire game from scratch using C, then why was anyone even still be using Advance Map?!
Over Discord, I excitedly told my brother about this project and how it's gonna change modding the games completely and
that he would literally be able to do whatever he wants with it! He of course only wanted to change the trainers, so I
tried to teach him how to work with the .c and .h files. But it turned out that the project wasn't as neatly organized
as I hoped. Now, I'm not criticizing the way the project is organized (although there are some problems with it which
are currently getting resolved), but it wasn't very good for this purpose. In particular, changing the trainer data was
going to be difficult, because they were spread over so many different files with different syntaxes. And my brother is
not a programmer.

The PRET project was awesome, but it needed some structure, and it needed an editor that would allow anyone without any
experience to jump right into modding these games.

I decided to drop the original idea of doing it in C# and instead went for my usual Typescript + Vue.js stack which
allowed me to write better code and especially a much more sophisticated frontend. I named this project *Dynamaxed*.

### Serialization

I quickly realized that in order for this project to work, I needed a .json file with all the information of all the
trainers in it, and then a generator which would create the .c and .h files from it. Creating that was actually a joy to
do. IntelliJ's Search and Replace functionality that allows you to use complex Regular Expressions made it a fun
exercise to transform the original data into JSON, and I created
a [tool set](https://github.com/luxalpa/dynamaxed/blob/70272b6f600aad9c2d3b01f12855902e6d68f715/src/model/serialize/common.ts)
to automatically transform that JSON back into C Header files.

For this to work in production without a myriad of merge conflicts however, I had to fork and modify
the [pokeemerald](https://github.com/pret/pokeemerald) project. This has the side effect that there will now be stable
versioning for Dynamaxed.

### UI

The UI design was the first real problem I encountered. There were tons of options, and I needed a UI design that had to
both look good but also be flexible enough for all kinds of very different inputs and graphical representations, as well
as adaptive to future changes. I initially tried it with a design using the Vuetify library which ended up restricting
me too much. The main issue was that the Vuetify elements were simply too big, had too large margins, and thus were
taking up too much space. I ended up having to manually adjust all the paddings and margins on pretty much all the
elements and that just turned out to really not work all that well. Vuetify is amazing for dashboards, prototyping and
anything that needs Material Design, but customizing it was such a huge amount of work that it wasn't worth it.

I ended up with a custom built UI that is strongly aligned to a grid. I am not 100% happy just yet, but it works
extremely well.

Initially I tried using CSS (with SASS) but I had some issues with regards to theming. I tried Stylus, but the same
issue remained. In the end I decided to use [TypeStyle](https://github.com/typestyle) which is just an amazing CSS in JS
library that I don't want to miss anymore. I can now make use of scoped CSS while at the same time still being able to
import CSS and work with it as if it was JavaScript.

### UX

Another difficult bit was the User Experience. The traditional editors had so many windows, and I wanted to use a design
without requiring windows (mostly because implementing those in Electron is generally a really bad idea). While my UI
design mostly resembles Visual Studio Code, I tried windows, tabs and panes (like in Unity or IntelliJ), but they all
didn't work too well. In the end I came up with a custom solution that I am very happy with. When the user wants to edit
a trainer, then usually one of the next steps would be to edit the pokemon, or the map, or anything that was somehow
related to the trainer. The idea was to move all the info that is related onto the same page and provide navigation
primarily using short cuts to those places (so for example, when you want to edit the position on the map for that
trainer you could just middle click on the map preview and if you wanted to edit the trainer class you could just middle
click on that). The middle click navigation in addition to back and forward buttons turned out to be a really nice
design and I'm still very proud of it.

For this design, I needed a View Manager. I would leave away most dialog windows, except for when directly editing a
value (say a text or a number, or selecting a picture or an element from a list). With a lot of TypeScript magic I was
able to implement
a [View and Dialog system](https://github.com/luxalpa/dynamaxed/blob/70272b6f600aad9c2d3b01f12855902e6d68f715/src/modules/view-manager.ts)
which would allow me to send parameters and return values from the Views / Dialogs that are actually being typechecked.

## The Future

A lot of work has been done already, but there's still quite a bit ahead. Dynamaxed is actually one of my currently
active projects and has the highest priority. The plan is to go into Beta as soon as it is feature complete with Advance
Map (which requires me to still add in the mapping tools which are missing right now). I already created a preconfigured
build system with MSYS2 that can be shipped with it. I also need a solution to migrate the JSON files, I need to add
nice animations, make it more responsive to different window sizes and support auto updating. But overall, the road to
1.0 lies clear ahead and I can't wait to present it to the modding community. 
