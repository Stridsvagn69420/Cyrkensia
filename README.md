# Cyrkensia
Repository Server for [Azura](https://github.com/Stridsvagn69420/Azura) and [Shigure](https://github.com/Stridsvagn69420/Shigure) music repositories

# Installation (Go)
See [Go Installation](go/README.md)

# Installation (PHP)
See [PHP Installation](php/README.md)

# Granting access
The file responsible for it is `/etc/cyrkensia/.htpasswd`  
Here's a little cheat sheet:  
```sh
#Adding a user
htpasswd -b /etc/cyrkensia/.htpasswd yourName yourPassword #Replace yourName and yourPassword with the preferec username and password

#Removing a user
htpasswd -D /etc/cyrkensia/.htpasswd yourName #Replace yourName with the username of the account that's supposed to be deleted

#List every existing user
cat /etc/cyrkensia/.htpasswd
```

# File structure
The file tree should look lile this:
```
musicCDN.yourserver.com or yourserver.com/musicCDN (/path/to/CDN-root)
|-- GameXY
|   |-- .metadata.json
|   |-- OST-1.mp3
|   |-- OnlyFiles-NoDirectories.ogg
|   |-- When you realise that there's an easter egg.oga
|
|-- Firefield
|   |-- .metadata.json
|   |-- Intro.wav
|   |-- Night Dragon.flac
|   |-- Hot Spring Time.m4a
|   |-- Dawnbreaker 2.webm
```

# Adding .metadata.json files
These are neccesary in order for the client to know title and cover asset of the currect album. They should look like this:
```js
{
    "name": "Epic Music 2020", //The name of your album
    "cover": "default" //the Rich Presence Asset name you'd like to choose for the currect album, for more look at #RPC-Assets
}
```

# RPC-Assets
Every Rich Presence Asset can be found in the [asset list](RPC-Assets.md).
# Cyrkensia
