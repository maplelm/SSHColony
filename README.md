# SSH Colony

SSH Colony is ment to be a colony simulation game a lot like Dwarf fortress
with the twist of the worlds being much more hostile and I want to put the
enphisis on combat and survival more so then casual building and story telling.

# Technical Specifications

- ASCII graphics
- Playable over SSH
- Written in almost raw rust

## Rust Dependancies

- Unix
    - Libc
- Windows
    - WinAPI

# Game Play Specifications

- Procedurally Generated World
- Tile Based world
- Each npc in the world can't be directly controled by the player
    - The player will give them orders to follow
- There should be a crafting system
- In-depth combat system
- Defensable constructions

## Game Play Loop (10-20 second player action loop)

