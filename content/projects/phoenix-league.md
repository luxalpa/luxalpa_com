---
title: "Phoenix League (Dota 2 Inhouse League with custom matchmaking)"
slug: "phoenix-league"
abstract:
  "My attempt at improving the Dota 2 matchmaking system. It didn't go very far, I got some great ideas to further improve it, but due to changes in Dota 2 it is no longer feasible."
date: "2017"
---

This project can be found on github at  
<https://github.com/luxalpa/phoenix-league>

## Motivation

Back in 2017 Role based matchmaking in the Valve video game [Dota 2](https://www.dota2.com) did not yet exist and I
wanted to make it a reality. I was frustrated with the way matchmaking worked in this game, since people often tended to
specialize on roles, but based on random chance to people who specialize in the same role could be matched on the same
team which would make the experience much, much worse.

## Implementation

There were multiple implementation paths that I followed, the first one using Go, which I had just learned 9 months prio
to this project. But it quickly became apparent that this would be a case where I could prototype much faster in
TypeScript on Node.js.

A difficult decision was how to actually create a working matchmaking system. I considered a lot of different options
and read up a lot of stuff on the Internet (where I couldn't find much info about it). I ended up settling for a
bucket-system which I would later then revise.

One of the main hurdles was actually the very first step: When a player went to the website, he was supposed to login
with his Steam Account in order to join the matchmaking. This required me to store user data (the user session) on the
server (I used Postgres and Flyway for that) and it required an implementation of WebSockets and OpenID Connect on the
server.

At the time I had no idea about OpenIDConnect (and OAuth) and had to look it up. About 4 hours of different Internet
videos later, I finally understood how it works and was able to script together
a [solution](https://github.com/luxalpa/phoenix-league/blob/067796924292f83bd3af98a88814431683998d06/server/src/webapi.ts)
using Passport.js. This really took multiple days of pretty much fultime work and was a huge effort.

## Future

The project as of now has been abandoned. I got an idea for an extremely efficient matchmaking system which works based
on the fact that you can save the history of which players queued at which time, which you could then later use in order
to predict queue times (instead of just having players wait and the search range increase) which would allow us to
determine things like max spread, find matches much faster, tell players how long they will have to wait, among other
things.

I really want to try out this system in practice at some time.

Anyway, since Dota 2 received an update just months later which effectively added role queue to the base game, it seemed
unreasable to move on with this project as a matchmaking system works better the more players there are and I couldn't
realistically expect a large portion of the player base to move over to my custom league. Another huge deal was the
realization that there was simply no way to get the replays of the players (host bots don't have access to them), which
meant I couldn't do anything about abuse (Leaving, game ruining, flaming, etc) and I would need to have a very complex
system for player reports.

In the end I decided the system was just too much work and too unlikely to succeed, and as such the project had been
canceled. 
