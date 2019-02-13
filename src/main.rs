fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 2);
    let mut s = std::collections::HashSet::new();
    for e in std::fs::read_dir(&args[1]).unwrap() {
        let p = e.unwrap().path();
        if !p.is_file() {
            continue;
        }
        let d = std::fs::read(&p).unwrap();
        let m = md5::compute(d);
        if s.contains(&m) {
            println!("remove {:?}", &p);
            std::fs::remove_file(&p).unwrap();
            continue;
        }
        s.insert(m);
    }
}
