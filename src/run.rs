pub fn run<'a>(file_content: &'a str, query:&str) -> Vec<&'a str> {
  let mut vec = vec![];
  file_content.lines().fold(&mut vec,|results, line| {
   if  line.contains(query) { results.push(line); }
    
    results
  });

  vec
}