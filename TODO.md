# TODO

- [x] ~~Use animated version of coin sprite~~
- [ ] Fine-tune jump (ie finish going through the videos)
- [ ] Fix crashes when restarting after winning
  - This happens because the player doesn't get despawned
  - Basically we just need to make sure that everything that happens in `OnExit(GameState::Dead)` also happens in `OnExit(GameState::Win)`
    - If possible, just move the stuff into `OnEnter(GameState::StartScreen)`
    - Maybe make a new state for cleanup or smth like that? or `SystemSet`
      might be useful for this
- [ ] Music
- [ ] Joysticks, so mobile is playable
- [ ] Jump animation
- [ ] Increase physics framerate? rn it breaks if its not 60fps
- [ ] Test on iOS
