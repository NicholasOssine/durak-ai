use crate::engine::Action;

fn format_action(action: Action) -> String {
    match action {
        Action::Attack(card) => format!("a{card}"),
        Action::Defend(card) => format!("d{card}"),
        Action::Take => "t".to_string(),
        Action::End => "e".to_string(),
    }
}
