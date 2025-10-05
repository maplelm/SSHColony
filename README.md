# SSH Colony

> *This game is a work in progress and is in early development*

SSH Colony is ment to be a colony simulation game a lot like Dwarf fortress
with the twist of the worlds being much more hostile and I want to put the
enphisis on combat and survival more so then casual building and story telling.

# Todos

- [ ] Implement enable/disable feature for menus
- [ ] engine not properly entering alt buffer
- [ ] engine not properly hiding and reveling cursor
- [ ] engine not handling different sized terminals
- [ ] need to make text box elements for world generation
- [ ] Need to make a world generation alg
- [ ] Need to test what I have with just a basic world.
    - [ ] world generation will just pop out the same very flat and small world every time with no settings.
- [ ] Will need to figure out how pagination should work for menus and such
- [ ] Watch for `SIGWINCH` signals and fire off an event to let the engine know the terminal size has changed.
- [ ] figure out the best way to link up my dependencies that I have locally with the github repo as currenlty anyone else that downloads this projects will be missing key parts of the project.
    - This doesn't work well for ui and I don't see how I can more cleanly link the rest of my program to the renderer
    - I don't really want to have to manually move al of the ui evertime I want to camera to move to a different spot in the map.
- [ ] On the rendering side implement dangling object clean up. if an arc `RenderUnitId` has no weak refs it has not connection to the update thread


## Implementations

- [ ] Selector
- [ ] State Management (What UI Element is active, what ingame menu is active if there is one)
- [ ] Basic terrian generation


# Technical Specifications

- ASCII graphics
- Playable over SSH
- Written in almost raw rust

# Game Play Specifications

- Max Temp in game 2100deg C
    - fammability equation should be (1.0 - fashpoint / maxtemp)
- Procedurally Generated World
- Tile Based world
- Each npc in the world can't be directly controled by the player
    - The player will give them orders to follow
- There should be a crafting system
- In-depth combat system
- Defensable constructions

