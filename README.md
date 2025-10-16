# SSH Colony

> *This game is a work in progress and is in early development*

SSH Colony is ment to be a colony simulation game
a lot like Dwarf fortress with the twist of the
worlds being much more hostile and I want to put
the enphisis on combat and survival more so then
casual building and story telling.

# Technical Specifications

- ASCII graphics
- Playable over SSH
- Written in almost raw rust

## Rendering

Any object in the game that can be rendered needs to have a `Weak` reference
counter to a `RenderUnitId` object so that when it tells the renderer to start
drawing the object it can hold a reference to that objects id and can tell the
renderer how the object needs to be visually manipulated. when sending things
to the render to be processed you only need to send the raw text and some style
information. the render will apply border, justification and more after it gets
the object from the update thread.

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

