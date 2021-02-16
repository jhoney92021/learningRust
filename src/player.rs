
#[derive(Debug, Clone)]
pub struct PlayerClass{
    pub class_name: String,
    pub strength: i32,
    pub deterity: i32,
    pub intelligence: i32,
    pub health: i32,
}

impl PlayerClass{

    pub fn new(
                class_name: String,
                strength: i32,
                deterity: i32,
                intelligence: i32,
                health: i32
            ) -> Self{
                    PlayerClass{
                        class_name:class_name,
                        strength: strength,
                        deterity: deterity,
                        intelligence: intelligence,
                        health: health
                    }
                }

}

pub struct PlayableClasses{
    pub all_classes: [PlayerClass; 3],
}

impl PlayableClasses{
    pub fn all_classes() -> [PlayerClass; 3]{[
                PlayerClass::new("None".to_string(), 0, 0, 0, 0),
                PlayerClass::new("Samurai".to_string(), 10, 10, 6, 30),
                PlayerClass::new("Wizard".to_string(), 5, 5, 20, 10)
    ]
    }

}

pub struct Human{
    pub name: String,
    pub player_class: PlayerClass,
}

impl Human{
    pub fn new(name: String, player_class_selection: usize) -> Self {
        Human{
            name:name,
            player_class: PlayableClasses::all_classes()[player_class_selection].clone(),
        }
    }

    pub fn printStats(&self)
    {   println!("Hello, My Name is {:?}, 
        My Class is {:?} 
        Which Have Stats, 
        Strength: {:?}, 
        Dexterity: {:?}, 
        Intelligence: {:?}, 
        Health: {:?}",
        self.name, 
        self.player_class.class_name.to_string(),
        self.player_class.strength.to_string(),
        self.player_class.deterity.to_string(),
        self.player_class.intelligence.to_string(),
        self.player_class.health.to_string(),
        )
    }
}
