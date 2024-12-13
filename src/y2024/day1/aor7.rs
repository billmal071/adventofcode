pub struct LogQuery<'a> {
  logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
// 2. Create a public associated function named `new()` that will take a reference to a vector of strings
//
// 3. Create a public method named `search` that accepts a string slice and finds it from the logs and
//    returns a vector of references to those logs.
impl<'a> LogQuery<'a> {
  pub fn new(logs: &'a Vec<String>) -> Self {
      Self { logs }
  }

  pub fn search(&self, keyword: &str) -> Vec<&'a String> {
    self.logs.iter().filter(|log| log.contains(keyword)).collect::<Vec<_>>() 
  }

  // pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
  //   // 🎁 Your code here! 🎁

  // }
}
