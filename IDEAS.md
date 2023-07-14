# Ideas

A bunch of random ideas about how to structure the code.

## General

- Each submodule could have a plugin that loads all related stuff (e.g. spawning), or it could be done in a single system in the main module.
  - if it is done in the main module, there could be functions in the submodule that ie create a new sprite
- Physics run at 240fps, independent of the update cycle
  - running on fixed timestep
  - see <https://gamedev.stackexchange.com/questions/1589/when-should-i-use-a-fixed-or-variable-time-step>
