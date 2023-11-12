use std::{
    fs::OpenOptions,
    io::Write,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() {
    println!("start");
    let file = "/home/elico/Time";
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
            .open(file)
            .unwrap();
        f.write_all(t.to_string().as_bytes()).unwrap();
        timer.tick().await;
    }
}
