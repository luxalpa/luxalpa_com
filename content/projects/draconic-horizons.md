---
title: "Draconic Horizons - Deckbuilder for Magic: The Gathering"
slug: "draconic-horizons"
abstract:
  "A full deckbuilder and collection manager for Magic: The Gathering. Both the frontend and the backend are completely 
  written in Rust. With more than 20,000 lines of code, it is my largest solo project to date."
date: "2023 - 2025"
---

This project is released under <https://www.draconic-horizons.com/>

## Motivation

I had been playing the collectible card and deck-building game Magic: The Gathering since 2022. Building decks was a
joy, but I found the deckbuilding tools like Archidekt and Moxfield a bit clunky to use. In addition to this, I wanted
to try out writing a web-frontend and backend entirely in Rust and to learn Leptos to use for any other future project.

## Implementation

The implementation can be grouped into several sub-projects, each with their own set of challenges.

### UI

The frontend code was written in Leptos and makes heavy use of Server-Side rendering.

#### Virtual Scroll Grid

One element that was difficult to build was the virtual scroll grid, which allows players to scroll through all the
available cards. Unlike normal infinite scroll grids or lists, one thing I wanted to have was a system in which it was
easy and intuitive to scroll to the middle or the very bottom of the list. The game has >30,000 unique cards, and >
100,000 cards that differ in edition, variant or artwork. Rendering all of these with their pictures and event handlers
proved way too slow, so I ended up virtualizing the list. Based on the grid-sizes, it automatically calculates how large
the grid needs to be, and based on the scroll position it decides which cards to display.

#### Tooltips

When the user moves their mouse over a card, I want to show a tooltip with a larger version of the card. This was tricky
to implement with the aforementioned virtual scroll grid, because on certain DOM operations (which are done under the
hood by leptos) the web API does not trigger a mouse-leave (or any other) event, causing the popup to be stuck.
Currently the way I workaround this is to simply close all tooltips whenever the user scrolls or moves the mouse over
another object. The downside is that while scrolling down, the popup will close even if the mouse is still over the same
card. I am still looking for a better solution here, but the very limited web API's make this very tricky to implement.

#### Dialogs

There is a web proposal for HTML-based dialogs, however I have still used the old version of using mainly divs. There
were some tricky issues with backdrop-stacking and animations which I managed to solve, and the dialog system ended up
using some fairly advanced interactions with dyn-traits and reference counting in order to allow dialog functions to be
typed properly - for example, the callback closure that runs when the dialog is being closed should only need to
implement `OnceFn` instead of the way more restrictive `Fn` trait.

#### Animations

Originally I used normal CSS transitions and animations, however, I really wanted to have close- and flip-transitions,
which require custom code. Leptos does not have first-class support for animations (unlike Vue.JS which I had used
before) and its fine-grained reactivity approach (as opposed to diff-based) made implementing animation tracking a real
challenge. Elements need to stay in the DOM even after they had already been removed, but reactivity needed to be
disabled from those elements, else they would break. Furthermore, the inability to use view-transitions and to animate
`display: none` in Firefox (my daily browser)  meant that elements needed to remain in their position in the DOM in
order to receive the correct stylings. I ended up building a fairly sophisticated (but also a bit brittle) animation
system named "leptos-animate" which is its own project.

I also added physics-based animations as I find them to be a lot more satisfying. A fun thing I did for this was to take
my Second-Order dynamics Euler integrator from my other game dev project and convert it into SASS-code which looks very
wild!

#### Multi-Select

One feature that I was sorely missing from other deckbuilders was the ability to easily select multiple cards in your
deck in order to copy+paste them between decks, remove them, add tags to them, etc. Originally I tried doing this using
the normal text selection feature of the browser, but it proved to be very insufficient. I ended up implementing my own
drag-selection algorithm, which tracks the start and current/end position of the mouse cursor during the drag,
calculates the position of all the cards and then finds those that overlap with the selection box. A challenge here was
to handle the stacked-cards view in which some cards were behind other cards. The selection needed to avoid selecting
the card if the rectangle only intersected with the occluded parts of the card. If all visible parts of the card were
outside of the selection rectangle, it should not be included in the selection.

Another thing it needed to do was to handle different select operations. I took inspiration from the Windows File
explorer here in order to implement union-behavior on Shift and intersection on Ctrl.

### Routing

As the app grew more and more complex, routing requirements increased. I ended up wrapping `leptos-router` with my own
macro-based system in order to have typed routes that I can use to generate the appropriate links. Furthermore I had to
opt-out of `leptos-router`'s Router Outlet (i.e. displaying components) as I needed the behavior of elements on the
screen to be independent of the route - and because I wanted to use proper page transitions. For example, on the front
page, there is a search mask, and when entering some text into it, it will then move to the search results page - but
the actual page component is still the same, and the search mask simply moves between the two pages (from bottom of the
screen to top of the search results).

### State management

A large challenge and something that I wanted to fix over other deckbuilding apps was the handling of state. It should
be snappy and performant on the client side, while also cheap on the server. And it should not run out of sync. It is
fairly common to have multiple tabs with the software open, and I wanted all of these tabs to always reflect the correct
current state of your deck. Thankfully, these parts were fairly easy to implement. I built an event system where each
state modification is done by creating an event. This event will then be sent to the server, to other browser tabs and
also be handled in the current browser tab. All of the users own data is local and is simply synchronized with the
server via these events. This allows state updates to be instant - the user does not have to wait for the server to
update.

In addition to that, this allowed me to implement undo/redo in a very simple and straight forward manner.

A more complex feature was the ability to use the app with and without a user account. If the user is not logged in, I
still want them to test out and experiment with the deckbuilder. I am a big fan of trying something out before going
through the process of registering an account and saving passwords, which I find to be a big commitment. However, this
proved to make the Request system really complex due to the change in behavior on SSR (Server Side Rendering).
Basically, the timing at which user-data is available changes, and this was really difficult to implement and test.

### Requests and Caching

The single largest challenge when creating the deckbuilder was caching and general request handling (like batching). The
main blocker here was the leptos framework. It uses a Resource-system which clearly was not made with caching in mind,
and especially on the first leptos version I used (v0.6) it was also extremely buggy. You had to subscribe to a resource
within a `<Suspense>` block by using `get` or `track`. Most importantly, you couldn't fetch resources from outside of
those blocks, and you couldn't have resources be dependent on one-another.

For example, I wanted to load the Deck which gave me a list of card ID's, which I then wanted to use to fetch the
information about the cards. But a card that was already in the cache shouldn't be fetched again. And while leptos did
allow this perfectly for local resources on the client, it would give cryptic hydration bugs if you tried to do this on
SSR.

This meant that in the end I actually had to create a Request batching system where each page would initially have to
declare which resources it would need to prefetch on initial SSR page load.

The entire system went through many iterations and complete rewrites before I got to the current system. And likely
there will be many more, because each minor patch level update on Leptos seems to be breaking something again. And I
have invested literal months to make this work at all.

The request system uses Bitcode's encoding under the hood (instead of JSON) because it is significantly faster and
smaller and I think it's very cool that thanks to Rusts `derive` system this can be very easily implemented.

### Card Search

In order to find cards for the deck (or for adding to the collection) I could have just used Scryfall's web API.
However, there were things I didn't like about it: I wanted to implement my own custom search operators (and syntax), I
wanted to be able to use its search markup language also on cards in the Deck, Collection and other places, and I wanted
it to be very fast.

Implementing this in Rust was originally fairly straight forward with the added benefit that I could simply use the same
code both on the server and on the client (if needed). However, I wanted it to be very compatible with Scryfall's own
syntax, which has its quirks, and in the process I ended up building a more modular system of Sources and Matchers that
allow more code-reuse and fancy code-gen via templates. I then optimized it a lot, in particular the string searches
which were very slow (for a feature that people would be using all the time). For example, I ended up storing lower-case
versions of all of the card-text in a separate index in order to avoid doing the conversion at runtime.

Another big piece of work was card loading. Every hour it will do a request to scryfall, asking if changes were made to
the current data. In that case it would then download the batchfile which is a huge compressed JSON file and store it in
a local cache (so that on restarts or reparses it wouldn't need to download it again). I extended the `serde_json` crate
with a streaming json parser, as I wanted the entire thing to consume as little memory as possible (and loading a
gigabyte of JSON into RAM was a bit much for an app that would only use like 100MB at runtime). The final card structs
actually make a lot of use of Rust's typing and enums in order to minimize the memory footprint. Whereas the original
JSON stores everything as Strings, I manually parse things like power-toughness values and mana-costs in order to store
them in a more condensed manner. This doesn't just reduce the internal memory footprint (and therefore massively improve
search speed), but it also reduces network load as well!

### Deployment

#### Migrations

The user can have their user data stored on the server. But they could also have it stored locally in the browser
instead if they didn't have an account yet. I built a versioning system that tracked my data-model versions and if the
user had an outdated version, it requests a specific wasm/js module which then runs a migration script locally in the
browser. Furthermore, each request to the server sends the frontend-api-version, and when the server detects that it is
out of date, it will send a specific error to the client that forces the client to reload the page.

On the server-side, migrations primarily affect the Postgres Database. Some of the migration code can be run via SQL,
but in a lot of cases, I store encoded objects which need to go through Rust in order to be decoded, modified, and then
reencoded.

#### Other stuff

Handling OpenTelemetry data, setting up Jaegr and Prometheus for tracing, logging and metrics proved fairly challenging
as the docs and guides for OpenTelemetry were clearly not intended to be consumed by solo developers for solo projects
and overcomplicated and abstracted everything. I was very happy when I got everything working - and code-wise it isn't
much, but it was tricky to find out. Same thing for SSL. The SSL implementation itself is very easy in the end, but
understanding what specifically I needed to do and how the ACME stuff works under the hood and how to marry it with
Actix-Web was a challenge.

The release build process had a few quirks as well. I needed to build inside docker in order to match my target OS. But
my release builds needed also to be fast in case I'd have to do a hotfix after release. So I worked a lot to improve
layering and caching inside the docker-container. A particular challenge here was to get the stuff to work with
`cargo-leptos` which was required for a leptos frontend build. And it needed to use a specific Nightly build of Rust.

## Future

I definitely focused on the wrong parts of the project. When I initially released it, I got very little reception. Part
of this is certainly because I should have provided pictures and videos instead of just a link and a description. But I
think I focused way too much on making the interaction with the app fast and getting rid of annoyances - which turned
out to be not as useful if you're still missing a lot of features that other apps had. I definitely underestimated the
amount of work it would take to implement all the minor helper functionalities for the user - things like sorting the
cards, changing the view-style, selecting the export format, etc. I kinda gave up on the project because it seemed like
it would take me forever to implement all those little utilities. And then people would probably still use the other
apps simply because that's what they always used.

I think what I really need to do is add more big features. Ironically, a huge feature is about as difficult to implement
for me as a minor utility. So I should really focus more on several large features that give this app more of an
identity and especially relevance.

Originally the goal was to improve incremental deckbuilding by making playtesting a lot easier and to add several
innovative (?) features to integrate playtesting and deckbuilding (such as placeholder cards, playing through what-if
scenarios, etc). This is likely what I will focus on again in the future.

This project was originally started as a commercial project, but doing it like this is just not fun and really just very
stressful. Being at the whim of the community opinion while at the same time living on my savings and unemployment
benefits was a huge drain on my motivation and has caused me to get very depressed. Therefore, the biggest lesson is
that I need to attack future hobby projects in a more laid-back manner without them needing to satisfy everyone else,
and without deadlines.
