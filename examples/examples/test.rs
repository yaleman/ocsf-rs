use ocsf::events::BaseEvent;

fn main() {
    let base_event = BaseEvent::default();
    println!("BaseEvent::default() prettyprint:\n{:#?}", base_event);

    println!(
        "BaseEvent::default() json:\n{}",
        serde_json::to_string(&base_event).unwrap()
    );
}
