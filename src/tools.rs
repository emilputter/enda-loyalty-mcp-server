use crate::models::ClientClasses;

pub fn get_client_classes() -> Vec<ClientClasses>{

    let classes = vec![
        ClientClasses {
            id: "1".to_string(),
            name: "Gold".to_string(),
            min_score: Some(41),
            max_score: 60,
        },

        ClientClasses{
            id: "2".to_string(),
            name: "Silver".to_string(),
            min_score: Some(21),
            max_score: 40,
        },

        ClientClasses{
            id: "3".to_string(),
            name: "Bronze".to_string(),
            min_score: Some(1),
            max_score: 20,
        },
    ];

    classes
}