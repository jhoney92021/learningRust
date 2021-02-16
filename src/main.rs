mod player;

fn main() {
    let all_classes = player::PlayableClasses::all_classes();
    let samurai = player::PlayerClass::new("Samurai".to_string(), 10, 10, 6, 30);
    println!("{:?} Stats, Strength: {:?}",all_classes[0].class_name.to_string(),all_classes[0].strength.to_string());
    println!("{:?} Stats, Strength: {:?}",all_classes[1].class_name.to_string(),all_classes[1].strength.to_string());
    println!("{:?} Stats, Strength: {:?}",all_classes[2].class_name.to_string(),all_classes[1].strength.to_string());
    let mut jhoney = player::Human::new("JHoney".to_string(),0 as usize);
    jhoney.printStats();
    let mut jwiz = player::Human::new("JWiz".to_string(),2 as usize);
    jwiz.printStats();
    let mut jsam = player::Human::new("JSam".to_string(),1 as usize);
    jsam.printStats();
}
