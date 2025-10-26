---
title: "Dynamaxed-Translator"
slug: "dynamaxed-translator"
abstract:
  "An extension for the Dynamaxed project that translates its .json files to German language."
date: "2020"
---

This project can be found on github at  
<https://github.com/luxalpa/dynamaxed-translator>

For more information about Dynamaxed, read [the Dynamaxed project page](/projects/dynamaxed) first

## Motivation

My brother wanted to edit Pokemon Emerald, but the [PRET/pokeemerald](https://github.com/pret/pokeemerald) (at least at
the time of writing this) only had the English version. He said he wouldn't need everything translated, but the names (
and descriptions) for the Pokemon, Trainers, Moves, etc would be really cool.

## Implementation

The project was a one day side-project that I quickly scripted together in TypeScript. As input the command line utility
uses a German pokeemerald ROM and the folder to the dynamaxed .json files, and it modifies those files (so any other
changes to the stats etc will not be overridden).

The most tricky part of the project was
the [readPokeString](https://github.com/luxalpa/dynamaxed-translator/blob/ce41f24347f2fbef033f241ec4bf128dff8b0069/src/util.ts#L8)
function which translates the games custom character encoding into Unicode. The pokeemerald project uses a custom syntax
for characters that are not expressed as unicode (like the PKMN character sequence and various icons) and the game uses
a custom encoding for some of these characters as well (storing them as multiple bytes instead of just one).

My solution isn't perfect but it worked for this usecase. It did however require a bit of trial and error and isn't a
general solution for all other language translations.

Other than that, most of the work was just to find the different offsets, which I did by changing the text in the
English version, compiling it, then going at the address where its stored and looking at the encoded version, then
opening WinHEX with the German ROM and finding the encoded version of the text there. The offsets were different but
usually not by that much, so the data was rather easy to find.

Originally I thought I could just use the data from the [pokeruby project](https://github.com/pret/pokeruby) which has a
decompiled German version, but ruby doesn't seem to be super compatible to Emerald and in particular all the IDs (and
thus order of the names etc) were different, rendering it mostly useless.

The process however was fast and successful, and I can extend it at any point.

## Future

The project is basically completed, however there may be future cases where I want to add more translations. However,
I'd prefer a real translated version as part of the pokeemerald project and likely I would rather contribute there than
continuing to build up this one.
