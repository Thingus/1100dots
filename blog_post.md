Every LD, I will decide that my tooling is holding me back and pick a new one, only to discover that it's my terrible skill in vector maths that is the real problem. Doing this lap again, I chose to use the [Bevy](https://stackoverflow.com/questions/13652518/efficiently-find-points-inside-a-circle-sector) game engine this time around; here's some thoughts about Bevy and working with Rust for jams.

First of all, thanks to the **invaluable** resource that is the [Bevy Cheat Book](https://bevy-cheatbook.github.io/). It's been so good for teaching how to think in Bevy, I'd check it every time I needed to know how to express some concept or clarify something from the official docs; it didn't ever let me down.

### Upsides

- I've never worked with an ECS system before, but I can see why these are popular - it's a model that makes it very easy to move from the some concept, like 'I want everything to wobble now', to execution; give everything the `Wobbler` component, implement `wobble_wobblers` and feel joy. I put the level progression together in the last 15 minutes of the jam with no pain.
 
- Because it's in Rust, the game either didn't compile or Worked(tm). I didn't see any odd bugs to work around that were the fault of race conditions or shared access deep in the guts of the engine.

- Performance even in debug mode was great. I tried a similar tech demo involbing just moving points around the screen; it started to chug around 20,000,000 entities in debug mode, way more than you'd need for anything sensible. It felt like I didn't need to worry about performance even when I started needing to nest loops (for each particle -> for each interactable, ect). Could try using the `inner` join to start modelling particle-particle interactions and see how far the engine can be pushed.

- After *some* messing around, it compiled to a very distributable `wasm` build; no bloated Unity player needed. Thanks to Erik Horton's very useful Itch page framework [here](https://blog.erikhorton.com/2024/03/31/deploy-bevy-to-android-and-wasm.html).

### Things to watch out for

- **Geometry**. The biggest time-sink I had was doing some simple 2D rotations and checks. The standard `Transform` component in Bevy assumes that you're going to be working in 3D, and hence has a rotation component that needs a `Quaternion`; it took a fair bot of wading through the examples to find the `rotate_about_z()` func to treat them as a 2d entity. To be clear - none of this was the fault of the engine, I've just been spoiled by Godot's explicitly simpler 2D projects. Just look out for that if you choose to use Bevy. I'm also indebted to [this StackOverflow answer](https://stackoverflow.com/questions/13652518/efficiently-find-points-inside-a-circle-sector) for giving me the 'is within an arc' dot-product trick.

- Build times. This is the well-known Rust tradeoff, but oof it is real. First build will take around 10-20 minutes, and you can expect to repeat that for each time you add a new target or dependency; it's 'go have a snack' times. Incrementel compliation is a bit slower than I'd like too; around 20-odd secs from `cargo run` to game appearing. There's an experimental hotpatching system in Bevy to get around that now, but I didn't dare use it. For some reason, my dev machine was running incredibly slow Sunday evening, and so I wasted nearly an hour waiting for a build I didn't need to do - it is safe to `ctrl-c` them mid-build, they'll pick up again from where you left off.

- Wasm dependencies. If you're using `Rand::rng`, make sure you put the following in **.cargo/config.toml**, NOT `cargo.toml`:
```
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
```
If you don't, you fall down the hole of trying to implement your own prng out of bits of string and xors.

### Tieup

Can't wait to play with Bevy some more, it's great. Feels like it's what I've been looking for for years.
