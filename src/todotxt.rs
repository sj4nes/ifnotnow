// TTxt instead of TodoTxt because it would melt TODO scanners.
struct TTxt {
   items: Vec<String>,
}
impl TTxt {
   fn load(filename: String) -> TTxt {
      TTxt {
         items: vec![
            "(A) Hello".to_string(),
            "World".to_string(),
            "x done".to_string(),
            "x 2021-07-06 2021-07-01 was done".to_string(),
         ],
      }
   }
   fn save(filename: String) -> TTxt {}
   fn find(&self, query: String) -> TTxt {}
   fn exclude(&self, query: String) -> TTxt {}
}
