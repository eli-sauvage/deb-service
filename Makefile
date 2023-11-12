all:
	echo 'nothing to do.'

build:
	cargo build --release
	cp target/release/service-apt-test service-apt-test_1.0-1/usr/local/bin/service-apt-test
	dpkg-deb --build service-apt-test_1.0-1

install: build
	sudo apt install ./service-apt-test_1.0-1.deb -y

remove:
	sudo apt remove service-apt-test -y
