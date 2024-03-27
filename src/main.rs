use nodejs_resolver::Resolver;

fn main() {
  let cwd = std::env::current_dir().unwrap();
  let join = &cwd.join("/test");
  let resolver = Resolver::new(Default::default());
  println!("cwd is: {:?}", join);
  match resolver.resolve(&cwd.join("/test"), "./index") {
    Ok(v) => println!("ok, {:?}", v),
    Err(e) => println!("error, {:?}", e)
  }
}
