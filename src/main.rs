use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Report {
    participants: Vec<Participant>,
    messages: Vec<Message>
}

#[derive(Serialize, Deserialize, Debug)]
struct Participant {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    sender_name: String,
    timestamp_ms: u64,
    videos: Option<Vec<Videos>>,
    photos: Option<Vec<Photos>>, 
    content: Option<String>,
    share: Option<Share>,   
    is_geoblocked_for_viewer: bool,
    #[serde(default)]
    is_unsent_image_by_messenger_kid_parent: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Videos {
    uri: String,
    creation_timestamp:Option<u64>
}

#[derive(Serialize, Deserialize, Debug)]
struct Photos {
    uri: String,
    creation_timestamp:Option<u64>
}

#[derive(Serialize, Deserialize, Debug)]
struct Share {
    link: String,
    share_text: String,
    original_content_owner: Option<String>, 
}

fn main() {
    let path = std::env::args().nth(1).expect("No filename! Give me a JSON file with chat history uwu!");
    let report_string: String = std::fs::read_to_string(path).expect("wtf is this shit!");
    let a = serde_json::from_str::<Report>(&report_string).unwrap().messages; 
 //   println!("{a:?}"); //prints out every message
    //let count = a.into_iter().filter(|a| a.sender_name!="sladkijrulet").collect::<Vec<Message>>().len(); //message counter example with a filter
}
