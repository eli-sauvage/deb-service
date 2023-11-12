use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() {
    println!("start");
    let path = &std::path::Path::new(&env::var("HOME").unwrap()).join("Time");
    println!("file is at {:?}", path);
    let prefix = path.parent().unwrap();
    create_dir_all(prefix).unwrap();

    let mut timer = tokio::time::interval(Duration::from_millis(20));
    let t0 = Instant::now();

    loop {
        let t = Instant::now().duration_since(t0).as_secs().to_string()
            + "."
            + &Instant::now().duration_since(t0).as_millis().to_string()
            + "\n";
        print!("writing {}", t);
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();
        f.write_all(t.to_string().as_bytes()).unwrap();
        timer.tick().await;
    }
}
