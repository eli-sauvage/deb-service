simple apt package that installs and start a service

the service creates and updates the file at $HOME/Time every 20ms, 
setting the value in the file to sec.ms since the program has been started

this project is meant as an example for how to create an apt package that installs a new systemd service

# Install
`make install`
_requires root_

# Remove
`make remove`
_requires root_

# Build
`make build`
