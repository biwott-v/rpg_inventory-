// RPG Inventory System

#[derive(Debug, Clone)]
enum ItemType {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    item_type: ItemType,
    value: u32,
}

struct Inventory {
    items: Vec<Item>,
    max_slots: usize,
}

impl Inventory {
    fn new(max_slots: usize) -> Self {
        Inventory {
            items: Vec::new(),
            max_slots,
        }
    }

    fn add_item(&mut self, item: Item) -> Result<(), String> {
        if self.items.len() >= self.max_slots {
            return Err(format!(
                "Inventory full! Cannot add '{}'. Max slots: {}",
                item.name, self.max_slots
            ));
        }
        self.items.push(item);
        Ok(())
    }

    fn remove_item(&mut self, index: usize) -> Result<Item, String> {
        if index >= self.items.len() {
            return Err(format!("Invalid index {}!", index));
        }
        Ok(self.items.remove(index))
    }

    fn get_item(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
    }

    fn is_full(&self) -> bool {
        self.items.len() >= self.max_slots
    }

    fn total_value(&self) -> u32 {
        self.items.iter().map(|item| item.value).sum()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    // Create inventory with 5 slots
    let mut inventory = Inventory::new(5);

    println!("RPG Inventory System\n");

    // Add 5 different items
    let items = vec![
        Item {
            name: "Steel Sword".to_string(),
            item_type: ItemType::Weapon,
            value: 150,
        },
        Item {
            name: "Leather Armor".to_string(),
            item_type: ItemType::Armor,
            value: 100,
        },
        Item {
            name: "Health Potion".to_string(),
            item_type: ItemType::Potion,
            value: 25,
        },
        Item {
            name: "Magic Shield".to_string(),
            item_type: ItemType::Armor,
            value: 200,
        },
        Item {
            name: "Fire Bow".to_string(),
            item_type: ItemType::Weapon,
            value: 180,
        },
    ];

    println!("Adding items to inventory...\n");
    for item in &items {
        match inventory.add_item(item.clone()) {
            Ok(()) => println!("Added: {} ({:?})", item.name, item.item_type),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("\nInventory Status");
    println!("Items: {} / {}", inventory.len(), inventory.max_slots);
    println!("Is full: {}", inventory.is_full());
    println!("Total value: {}\n", inventory.total_value());

    // Try adding when full
    println!("Testing Full Inventory");
    let extra_item = Item {
        name: "Extra Potion".to_string(),
        item_type: ItemType::Potion,
        value: 30,
    };
    match inventory.add_item(extra_item) {
        Ok(()) => println!("Added extra item"),
        Err(e) => println!("Error (expected): {}", e),
    }

    // Remove specific item
    println!("\nRemoving Item at Index 2");
    match inventory.remove_item(2) {
        Ok(item) => println!("Removed: {} (value: {})", item.name, item.value),
        Err(e) => println!("Error: {}", e),
    }

    // Get item by index
    println!("\nGet Item at Index 0");
    match inventory.get_item(0) {
        Some(item) => println!("Item[0]: {} ({:?}) - Value: {}", 
            item.name, item.item_type, item.value),
        None => println!("No item at index 0"),
    }

    // Calculate total value
    println!("\nFinal Inventory");
    println!("Total value: {}\n", inventory.total_value());

    // Print all items
    println!("All Items:");
    for (i, item) in inventory.items.iter().enumerate() {
        println!("  [{}] {} - {:?} - Value: {}", 
            i, item.name, item.item_type, item.value);
    }

    println!("Done!!");
}
