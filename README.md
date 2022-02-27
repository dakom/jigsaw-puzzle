# [Play Now](https://dakom-jigsaw-puzzle.netlify.app/)

# What is it?

A pure Rust/WebGL jigsaw puzzle for browsers (desktop-only, for now).

Powered by [Shipyard ECS](https://github.com/leudz/shipyard). 

Puzzle pieces generated via [piecemaker](https://github.com/jkenlooper/piecemaker). 

Some other stuff under the hood ([awsm-web](https://github.com/dakom/awsm-web), [shipyard-scenegraph](https://github.com/dakom/shipyard-scenegraph), etc.)

# Status

Core graphics and io is done. Includes proper alpha click-through and other ux goodies.

Still more to do:

* Sound effects
* Random distribution
* Detect and lock-in correct placement
* Animation
* Instancing (get it down to one draw call)
