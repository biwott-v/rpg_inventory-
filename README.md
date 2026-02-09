# RPG Inventory System

A simple inventory management system written in Rust, demonstrating object-oriented programming concepts with enums, structs, and methods.

##  Features

- **Add Items**: Add weapons, armor, and potions to inventory
- **Remove Items**: Remove items by index
- **Inventory Tracking**: Monitor slots used/total
- **Value Calculation**: Calculate total value of all items
- **Capacity Management**: Enforce maximum inventory slots

##  Setup

```bash
# Compile and run
cd rpg_inventory-
rustc rpg_inventory.rs -o rpg_inventory
./rpg_inventory

# Or with Cargo (if using Cargo)
cargo run
```


##  Output Rust playgournd

![RPG Inventory Output](screenshot.png)

*https://play.rust-lang.org/?version=beta&mode=debug&edition=2024&gist=b9a393c6193955606ca0099a22fd48ce*


##  Code Overview

### ItemType Enum
```rust
#[derive(Debug, Clone)]
enum ItemType {
    Weapon,
    Armor,
    Potion,
}
```

### Item Struct
```rust
#[derive(Debug, Clone)]
struct Item {
    name: String,
    item_type: ItemType,
    value: u32,
}
```

### Inventory Struct
```rust
struct Inventory {
    items: Vec<Item>,
    max_slots: usize,
}

```

## License

MIT License - see LICENSE file for details

---

