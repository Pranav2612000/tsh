# tsh
##### In Progress.. Looking for contributors.
A (T)ime (S)aving (S)ecure (S)hell. Tired of remembering ip-address, key locations. Try tsh. Built using Rust.

### Adding Entries
tsh uses nicknames to identify stored configurations. Use the succeding command to add configurations.
```code
tsh add <nickname> <ssh-endpoint> <key-location>
```
Here, nickname is the unique name you want to give the current configuration. ssh-endpoint is the username@ip combination to which the connection is made. key-location is the location where the key is stored.
```code
tsh add mycloudpc ubuntu@13.126.131.221 ~/Desktop/keys/secret.pem
```
This will add a new entry named mycloudpc to your system. 

### Connecting to machines
To connect to a machine use
```code
tsh <nickname>
```
where nickname is the name of the entry we have added previously
e.g.
```code
tsh mycloudpc
```
This will open a ssh session to mycloudpc.
