use crate::gameplay::player::components::{BasicEmotions, PlayerData};

pub fn get_player_data() -> Vec<PlayerData> {
    vec![
        PlayerData {
            name: "Chester".to_string(),
            attack_dmg: 5,
            speed: 4.0,
            player_id: "1".to_string(),
            emotion: BasicEmotions::Happiness,
        },
        PlayerData {
            name: "Jakobo".to_string(),
            attack_dmg: 5,
            speed: 3.0,
            player_id: "2".to_string(),
            emotion: BasicEmotions::Hate,
        },
        PlayerData {
            name: "Marcy".to_string(),
            attack_dmg: 5,
            speed: 3.5,
            player_id: "3".to_string(),
            emotion: BasicEmotions::Disgust,
        },
        PlayerData {
            name: "Kitty".to_string(),
            attack_dmg: 10,
            speed: 2.5,
            player_id: "4".to_string(),
            emotion: BasicEmotions::Fear,
        },
        PlayerData {
            name: "Nimbus".to_string(),
            attack_dmg: 3,
            speed: 2.,
            player_id: "5".to_string(),
            emotion: BasicEmotions::Hate,
        },
        PlayerData {
            name: "Andrea".to_string(),
            attack_dmg: 2,
            speed: 4.,
            player_id: "6".to_string(),
            emotion: BasicEmotions::Sadness,
        },
        PlayerData {
            name: "Salem".to_string(),
            attack_dmg: 20,
            speed: 5.,
            player_id: "7".to_string(),
            emotion: BasicEmotions::Excitement,
        },
    ]
}
