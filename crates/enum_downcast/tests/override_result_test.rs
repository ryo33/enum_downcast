#[test]
#[cfg(feature = "derive")]
fn override_Result_test() {
    use enum_downcast::EnumDowncast;
    
    type Result = ();

    struct Player;

    struct Enemy;

    #[derive(EnumDowncast)]
    enum GameCharacter {
        Player(Player),
        Enemy(Enemy),
    }

    let player = GameCharacter::Player(Player);
    let _ = player.enum_downcast_ref::<Player>().unwrap();
}
