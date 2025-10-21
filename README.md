# SSH Colony

> **⚠️ Work in Progress** - This game is in early development

A colony simulation game inspired by Dwarf Fortress, with emphasis on survival, combat, and hostile environments over casual building. Play ASCII-based strategy directly in your terminal or over SSH.

## 🎮 Overview

SSH Colony is a tile-based colony management game where you guide NPCs through a procedurally generated world filled with challenges. Unlike traditional colony sims, this game focuses on survival in hostile conditions where every decision matters.

### Key Features

- **ASCII Graphics** - Classic terminal-based rendering
- **SSH Playable** - Connect and play from anywhere
- **Indirect Control** - Issue orders to autonomous NPCs
- **Procedural Generation** - Unique worlds every playthrough
- **Environmental Simulation** - Temperature, materials, and physics
- **Combat Focus** - In-depth combat mechanics for defense and survival
- **Crafting System** - Resource gathering and item creation
- **World Persistence** - Save and load your colonies

## 🏗️ Architecture

### Engine (`src/engine/`)

A custom multi-threaded game engine built from scratch in Rust:

#### **Core Systems** (`core/`)
- **Main Thread** - Primary game loop and coordination
- **Event Thread** - Input and event processing
- **Audio Thread** - Sound management and playback
- **Render Thread** - Asynchronous rendering pipeline

#### **Rendering System** (`render/`)
- **Camera** - Viewport and world navigation
- **Canvas** - Low-level terminal drawing
- **Drawable** - Object rendering interface
- **Render Units** - Object lifecycle management via weak references

The rendering architecture uses a `Weak<RenderUnitId>` reference counting system. Objects to be rendered send their raw text and style data to the renderer, which then applies borders, justification, and transformations asynchronously.

#### **Input System** (`input/`)
- Keyboard and terminal input handling
- Event queue management

#### **UI System** (`ui/`)
- **Buttons** - Interactive controls
- **Menus** - Navigation and selection
- **Textboxes** - Text input fields
- **Borders** - Decorative framing
- **Selectors** - Option picking
- **Styling** - Color and appearance

#### **Type System** (`types/`)
- **SparseSet** - Efficient entity storage
- **Store** - Template and data management
- **Position** - 2D/3D coordinate handling
- **Instance** - Game state management
- **File** - I/O utilities

### Game Logic (`src/game/`)

#### **Scenes** (`scenes/`)
- **Main Menu** - Game entry point
- **In-Game** - Core gameplay loop
- **Load Game** - World loading interface
- **Settings** - Configuration management
- **Generate World** - Procedural world creation

#### **Game Types** (`types/`)

**World System** (`world.rs`)
- Procedurally generated 3D tile-based worlds
- World size: configurable X×Y×Z dimensions
- Material system (up to 10,000 materials)
- Entity system (up to 10 million entities)
- Save/load functionality using bincode serialization
- Template-based object instantiation

**Entity System** (`entity.rs`)
- **Creatures** - Living beings (Dwarfs, Humans)
  - States: Idle, Combat, Moving, Dead
- **Objects** - Furniture and structures (Doors, Bins, Chairs, Tables)
  - States: Normal, Damaged, Broken
- Inventory management
- Stat system with customizable attributes
- Position tracking in 3D space
- Flags for passability and storability

**Materials** (`material.rs`)
- Physical properties simulation
- Material templates loaded from data files

**Tiles** (`tile.rs`)
- World building blocks
- Shapes: Floor, OpenSpace, etc.

**Inventory** (`inventory.rs`)
- Weight-based capacity system
- Item storage and management

**Stats** (`stat.rs`)
- Dynamic attribute system
- Template-based stat initialization

## 🌡️ Game Specifications

### World Generation
- **3D Tile-Based** - Fully volumetric terrain
- **Procedural Generation** - Unique worlds with configurable parameters
  - Average temperature
  - Average height/elevation
  - Sea level
  - World dimensions
- **Material System** - Rich material properties and interactions

### Environmental Simulation
- **Temperature Range** - 0°C to 2100°C
- **Flammability** - `1.0 - (flash_point / max_temp)`
- **Physical Properties** - Materials have realistic characteristics

### Gameplay Mechanics
- **Indirect NPC Control** - Issue orders and priorities, NPCs execute autonomously
- **Combat System** - Detailed mechanics focused on tactical survival
- **Crafting** - Resource processing and item creation
- **Defensive Construction** - Build fortifications and protection
- **Survival Focus** - Resource management in hostile environments

## 🛠️ Technical Stack

- **Language**: Rust (Edition 2024)
- **Serialization**: Serde, Bincode, RON
- **Terminal Rendering**: Custom implementation with `term` crate
- **Panic Mode**: Abort (for both dev and release)

### Dependencies
```toml
libc = "0.2"
serde = { version = "1.0.224", features = ["derive"] }
ron = "0.11.0"
bincode = { version = "2.0.1", features = ["serde"] }
term = { path="/home/maplelm/Developer/crates/term" }
chrono = "0.4.42"
```

## 📁 Data Files

Game content is loaded from data directories:
- `data/entities/` - Entity templates (RON format)
- `data/materials/` - Material definitions
- `data/sprites/` - Visual representations
- `saves/` - World save files (bincode format)

## 🚀 Getting Started

### Prerequisites
- Rust toolchain (2024 edition)
- Terminal with UTF-8 support

### Building
```bash
cargo build --release
```

### Running
```bash
cargo run --release
```

### Playing
Navigate the menus using keyboard controls. Create a new world or load an existing save to begin your colony simulation.

## 🎯 Development Status

**Current Phase**: Early Development

### Implemented
- ✅ Multi-threaded engine architecture
- ✅ Rendering pipeline with camera system
- ✅ UI component library
- ✅ World generation framework
- ✅ Entity and material systems
- ✅ Save/load functionality
- ✅ Scene management

### In Progress
- 🚧 Combat mechanics
- 🚧 Crafting system implementation
- 🚧 NPC AI and order system
- 🚧 Advanced world generation
- 🚧 Temperature simulation
- 🚧 Construction mechanics

### Planned
- 📋 SSH multiplayer support
- 📋 Advanced entity interactions
- 📋 Expanded material properties
- 📋 Quest and event system
- 📋 Performance optimization for large worlds

## 🏛️ Project Structure

```
TermRPG/
├── src/
│   ├── main.rs              # Entry point
│   ├── engine/              # Core game engine
│   │   ├── core/            # Threading and lifecycle
│   │   ├── render/          # Display pipeline
│   │   ├── input/           # Input handling
│   │   ├── ui/              # User interface
│   │   └── types/           # Engine data structures
│   └── game/                # Game logic
│       ├── scenes/          # Game screens
│       └── types/           # Game entities
├── data/                    # Game content
├── saves/                   # Save files
├── Cargo.toml              # Project configuration
└── README.md               # This file
```

## 📝 License

*[License information not specified]*

## 🤝 Contributing

This project is in early development. Contribution guidelines will be established as the project matures.

---

*Built with Rust 🦀 | ASCII Graphics 🎨 | SSH Ready 🌐*
