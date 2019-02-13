fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 2);
    let mut s = std::collections::HashSet::new();
    for e in std::fs::read_dir(&args[1]).unwrap() {
        let e = e.unwrap();
        let d = std::fs::read(e.path()).unwrap();
        let m = md5::compute(d);
        if s.contains(&m) {
            println!("remove {:?}", e.path());
            std::fs::remove_file(e.path()).unwrap();
            continue;
        }
        s.insert(m);
    }
}
