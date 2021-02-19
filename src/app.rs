pub struct App {                    // Aight figured this fucker out. 
    pub sender_name_app: String,    // These are variables with their types.
    pub created_at: String,
    pub text: String,
  }
    
impl Default for App {           // This is the default variables, so nothing loaded/changed.
    fn default() -> Self {   
      App {         
        sender_name_app: String::from("Sender unknown."),
        created_at: String::from("Time created unknown."),
        text: String::from("Contents unknown."),
      }
    }
  }