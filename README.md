`cargo build --release`

`cp target/release/service-apt-test service-apt-test_1.0-1/usr/local/bin/`

`dpkg-deb --build service-apt-test_1.0-1`

# Test
`docker build -t custom-debian` .
`docker run -it -v $PWD/service-apt-test_1.0-1.deb:/service-apt-test_1.0-1.deb debian bash`
puis dans le container : `apt install ./service-apt-test_1.0-1.deb`


