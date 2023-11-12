all:
	echo 'nothing to do.'

build:
	cargo build --release
	mkdir -p service-apt-test_1.0-1/etc/systemd/system/
	cp target/release/service-apt-test service-apt-test_1.0-1/usr/local/bin/service-apt-test
	dpkg-deb --build service-apt-test_1.0-1

install: build
	sudo apt install ./service-apt-test_1.0-1.deb -y

remove:
	sudo apt remove service-apt-test -y
