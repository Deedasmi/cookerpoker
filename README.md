# CookerPoker

NL Texas Hold 'em, but with like, slow cookers or smokers or grills or
something.

This repository holds all the crates in the CookerPoker ecosystem.

These are the main crates.

1. **poker-core**: Core poker game logic. What a card is, what a hand is, what
   hands beat what other hands, game tables, betting, etc.
2. **poker-bin**: Small CLI binaries that depend on poker-core for some small
   purpose. Maybe little poker drills that the authors dreamed up. Maybe some
other poker utility. Probably not generally useful.
3. **poker-server**: Rocket/Diesel web server. Maintains user profiles, client
   API, running a game, interacting with the DB, etc. Depends on poker-core.
4. **poker-client**: WASM client that runs in a web browser. Interacts with
   poker-server via an API over HTTP.
5. **poker-messages**: The messages/structures that the client and server use
   to communicate.

## Deployment layout

The CookerPoker web server is the central authority. It generates all
randomness, hosts games, tells all players what is happening in their game,
hears from players what they choose to do (and forward that on, if legal
action), has sole interaction with the DB, etc.

To support the web server, diesel is used to interact with an sqlite database.
Postgres may be used in the future.

## Features

In these early stages, consider this section more of a wishlist/todo than an
actual feature list.

### Server

MVP:
- [ ] Reflect game state via REST API
- [ ] Generate a game log (optionally blinded to one player's view) during or
  after a game is played.
- [ ] Ability to run concurrent games.
- [ ] Correctly handle side pots and win conditions

Stretch:
- [ ] Intra-table chat.
- [ ] Lobby chat.
- [ ] Asyncronous server->client communication
- [ ] Real time hand limits

### Client

- [ ] Players can be denoated as an admin of a game
- [ ] Admin players can add/remove other admins
- [ ] Admin players can add to/remove from other player's stack (for rebuys and
  to correct mistakes ["we were all supposed to start with 1,000 chips,
Dave!"])

### Core

- [x] What 5-card hand beats another (calculate once, then just lookup?)

### Unclear fit

- [ ] Randomness generated by all involved parties (all players in game + the
  server), combined, then used for the next hand/entire game.
- [ ] Way to cryptographically verify all player actions were legitimately
  made, server acted correctly, etc.
