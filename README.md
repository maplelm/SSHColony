# SSH Colony

> *This game is a work in progress and is in early development*

SSH Colony is ment to be a colony simulation game a lot like Dwarf fortress
with the twist of the worlds being much more hostile and I want to put the
enphisis on combat and survival more so then casual building and story telling.

# Todos

- [ ] Implement enable/disable feature for menus
- [ ] Implement Dyn trait object items for menus as I think this will be better then generics for interoperations with the renderer
- [ ] engine not properly entering alt buffer
- [ ] engine not properly hiding and reveling cursor
- [ ] engine not handling different sized terminals
- [ ] need to make text box elements for world generation
- [ ] Need to make a world generation alg
- [ ] Need to test what I have with just a basic world. 
    - [ ] world generation will just pop out the same very flat and small world every time with no settings.
- [ ] Will need to figure out how pagination should work for menus and such
- [ ] Watch for `SIGWINCH` signals and fire off an event to let the engine know the terminal size has changed.
- [ ] New Object system does not respect width and height all the time right now and doesn't really use the foreground and background at the moment I need to make sure those are built in. not sure of the best way to break that up.
- [ ] figure out the best way to link up my dependencies that I have locally with the github repo as currenlty anyone else that downloads this projects will be missing key parts of the project.
- [ ] convert the renderer to work with lists of render objects that have a position stored in them rather then having a grid and try to mimic manually placing things on the screen. this don'ts work well fro ui and I don't see how I can more cleanly link the rest of my program to the renderer
- [ ] Figure out how to implement absolute position in renderer to make implementing ui elements easier. I don't really want to have to manually move al of the ui evertime I want to camera to move to a different spot in the map.
- [ ] On the rendering side implement dangling object clean up. if an arc `RenderUnitId` has no weak refs it has not connection to the update thread

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



