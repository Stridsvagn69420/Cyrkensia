# Cyrkensia (Go)
Work in Progress

# Installation
## Config file
Every entry commented with it's [flags](#flags)
```js
{
    "vendorName": "Party Royale Radio", // Name flag
    "root": "/etc/cyrkensia/musicCDN", // Files flag
    "uuid": "00000000-0000-0000-0000-000000000000", // Uuid flag
    "hosticon": "default", // Icon flag
    "htpasswd": "/etc/cyrkensia/Cyrkensia.htpasswd", // Htpasswd flag
    "bindAddr": "0.0.0.0", // Bind flag
    "port": 3000 // Port flag
}
```

# Usage
## Flags
- `Name`: Your Display/Vendor name.
- `Files`: Local directory according to the [file structure](../README.md#file-structure).
- `Uuid`: UUID v4 to identify your host.
- `Icon`: [RPC-Asset key](../RPC-Assets.md) representing your host/server.
- `Htpasswd`: Path to a valid `.htpasswd` file for access management. Set `""` if none.
- `Bind`: IP Address to listen to. Set `0.0.0.0` for every Interface/IP-Address.
- `Port`: Port to listen on.