---
title: "Outworldzilla (Dota 2 Training tool)"
slug: "outworldzilla"
abstract:
  "The original idea was to create a platform similar to Dotabuff, but it ended up being just a page for a few minigames to improve at Dota. It really shows how powerful animations are, it was my first 'real' published project and I learned the entire FrontEnd ecosystem during this time."
date: "2016"
---

The source code for this project can be found at Github:  
<https://github.com/luxalpa/outworldzilla>

[Click here for a live version.](http://outworldzilla.luxalpa.io)

## Motivation

I have been playing Dota 2 since 2011 and always had high ambitions. By the end of 2015 I reached 6000 MMR which placed
me among the top tier players (and had me regularly compete with professional players). One thing that always annoyed me
was how little useful statistics there actually were. Dotabuff and OpenDota were the leading stat websites at the time,
and both of them showed (and still do) mostly useless information that wouldn't help anyone who actually wants to get
better.

I wanted to change this, so I started building another stats website.

## Finding the right technology

I have been using PHP until 2010 at which point I've started spending pretty much all of my time learning and improving
my skills for writing desktop applications using mostly C++ (in conjuctions with the Qt framework). When I returned to
Web Development back in 2016 the world had completely changed. All these new tools were flying around all of a sudden.
In 2010 I barely new about Mootools and jQuery (which seemed to have some kind of battle) and all of a sudden there were
things like React, Angular, Webpack, Sass, Gulp, TypeScript, ES6, NodeJS, and many more.

The first step was to get an overview of the forest and so I spent an entire week just ruling out frameworks and tools
that by 2016 had already become mostly outdated. There was still a host of options and I wanted to make it perfect. Me
not knowing about the dangers of premature optimization back then wanted a framework that was fast and easy to use and
optimize.

I took my first steps on the project with AngularJS. I've actually built a tool for customizing the keyboard layout in
Dota 2 and exporting it as `autoexec.cfg` which would allow players to do certain key combos (like using space or any
other button as a modifier key) that Dota 2 at the time did not allow. Unfortunately for me, I had to drop the fully
developed feature just a few months later before release because Dota 2 patched out the use of multiple key binds in
`autoexec.cfg`. Unfortunate!

Anyway, I was very unhappy with AngularJS and switched on to React. I tried to build a basic application but I needed
directives in order for my tooltip system to work (I didn't have and honestly still don't have any idea how else one
could write a decent tooltip system), and ReactJS didn't have those. Disappointed, I rewrote my app a third time, this
time using TypeScript and Angular 2 Beta. I had never used TypeScript before and the decorator syntax was definitely
something new to learn, and Angular 2 was really tough to use. But it was kind of cool, definitely much better for this
purpose than the other two systems I tried before. The beta of course constantly broke things. Actually, when the tool
was already in the Release Candidate stage, they still did major breaking changes (a lot of them as well) which is part
of the reason I never upgraded.

Another thing was getting the code onto the server. I needed a way to relatively easy update my code. It took me two
entire weeks just to create the build system because I had huge problems with first Webpack and then later Gulp. In the
end, the `browserify` tool really helped me out a ton, and I was able to tree shake most of angulars code (which was
incredibly huge!) with Rollup after a lot of trial and error.

An important aspect of this was to get the angular compiler to actually preprocess my templates and back then during
beta this was incredibly tough to deal with. But I worked it out in the end, and I was even able to add in a custom
translation system for if I ever wanted the ability to allow users to switch to different language versions on the fly.

## Implementation

The TypeScript usage was a first for me, just like the TypeScript compiler, so my code might not be looking super great,
but even in 2020 I'm still quite happy with it. One major thing I discovered in 2017 just a couple months short of
releasing the tool was Functional Reactive Programming with RxJs. I was glad that I just understood how promises worked
and used them all over the place, which is why I originally tried to avoid anything with `subscribe` or `observable` in
it. Only in January 2017 after a long video session of angular and Rx tutorials I slowly got a grasp on how to use it.

And I've used it everywhere. It actually made many of the tasks so much nicer to use, and remember when I said my focus
was on writing high performance code? Angular allows you to bypass its change detection mechanism completely if you're
exclusively using RxJs to update component state. And so that's what I did.

Only when I tried to build my own UI framework in 2020 I actually fully understood the genius of RxJs and the idea
behind the Observable pattern. But that's a different story.

Anyway, probably the biggest pain point (aside from the CSS) was getting the SVG graphics for the user score graph
right (the application tracks your scores for every minigame and displays them in a graph for watching your progress). I
had to make sure the lowest and highest points of the graph were always sensible (i.e. visible), so the axes needed to
be in the right positions. Also, I didn't want to have any weird numbers on the axes either,
so [getting that right was quite a lot of work](https://github.com/luxalpa/outworldzilla/blob/4e833678a7c0df98dc3386f4c59de2686478a65d/script/graph.component.ts#L101).
And I feel like it's still not working perfectly some of the time.

Another huge challenge was the CSS. First there was the thing with the animations. I wanted forward and backward
animations for the separate tabs, and I also needed the height of the tab to animate with it. I would kind of have an
idea how to do that in Vue.js now (in fact the former would be quite easy), but back then in Angular this was a nigh
impossibility and required a lot of global tinkering.

Then there was the CSS for the tabs. When a tab is behind a tab, the drop shadow of the box should not be appearing on
the inactive tabs (rendering them unreadable). On the other hand, the outer border needed to overlap. Or in other words,
the box needed to be both before and behind the tab (shadow behind, rest of the box before). Research about stacking
contexts in CSS finally caused the break through and until today it's still my most memorable experience using CSS.

But the animations for the minigames where actually really fun to implement. I still like to boot up the app
occasionally, just to click around and feel happy about the animations. I might have been terrible at designing User
Interfaces from a graphics design standpoint in 2016 (I learned a lot since then and did a few courses), but those
animations (along with my focus on UX) are still top notch in my opinion (check out the awesome animation on the tooltip
when it appears and disappears).

## Future

There is a possibility that I will revive this project at some point. But there is none that I reuse the code. For once,
after my work experience with Angular 7, I am pretty much done with Angular. It's not a bad framework per say, but it's
super unflexible, extremely hard and sometimes even impossible to customize. I would most likely rewrite it in Vue.Js.
TypeScript is fine and I actually started really liking RxJS.

Another thing is that back then I wrote a tool in C++ to export the game data from Dota 2 to put on the website. Today I
would definitely do this in NodeJs as it's much faster and easier to write tools on that, especially with the goals of
exporting as JSON for usage on the browser.

From a design standpoint I would completely change the design. Get rid of the turquoise outlines, get rid of the weird
color scheme and no more tabs, they look really old fashioned and can be quite confusing.

My modern approach to UX design has since changed quite a bit and nowadays I try to see it more from what the user would
actually care about and not try to optimize every tiny bit even if nobody would be using it.
