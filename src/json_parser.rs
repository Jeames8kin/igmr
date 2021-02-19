use serde_json::Result;

pub fn main() -> Result<()> {

  #[derive(Serialize, Deserialize, Debug)]
  struct Conversation {
    conversation: Vec<MessageElements>,
  }

   #[derive(Serialize, Deserialize, Debug)]
   struct MessageElements {
    sender: String,
    created_at: String,
    text: String,
  }

    let json_str = r#"
    {
        "participants":[
           "_jeames8kin15",
           "sethh.ogilvie"
        ],
        "conversation":[
           {
              "sender":"_jeames8kin15",
              "created_at":"2020-11-27T07:31:45.487829+00:00",
              "text":"Fucken wow"
           },
           {
              "sender":"_jeames8kin15",
              "created_at":"2020-11-09T08:18:19.248080+00:00",
              "text":"Listen to this dumb cung"
           }
        ]
     }
    "#;

    println!("Reading JSON...");

    let c: Conversation = serde_json::from_str::<Conversation>(json_str).unwrap();

    for i in c.conversation {

      println!("{:#?}", i);

      //Need to find a way to get this variable out of here.

    }
    
  Ok(())

}