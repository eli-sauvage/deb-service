`cargo build --release`

`cp target/release/service-apt-test service-apt-test_1.0-1/usr/local/bin/service-apt-test`

`dpkg-deb --build service-apt-test_1.0-1`

`sudo apt install ./service-apt-test_1.0-1.deb`
