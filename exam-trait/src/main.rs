#[macro_use]
extern crate impls;

trait Actor {
    fn actor(&self);
}

trait Health {
    fn health(&self);
}

struct Plant;

impl Actor for Plant {
    fn actor(&self) {
        println!("Plant Actor");
    }
}

struct Monster {
    health: f32,
}

impl Actor for Monster {
    fn actor(&self) {
        println!("Monster Actor");
    }
}

impl Health for Monster {
    fn health(&self) {
        println!("Health: {}", self.health);
    }
}

fn main() {
    let plant = Box::new(Plant);
    plant.actor();
    let monster = Box::new(Monster { health: 100f32 });
    monster.actor();
    if impls!(Plant: Actor) {
        println!("Plant is a type of Actor");
    } else {
        println!("not found");
    }
}
