# Cyrkensia (PHP)
## Prerequisites
* nginx installed and configured
* PHP for nginx installed and enabled
* Knowledge with the file system of your server
* Access to the file system of your server (sudo)
* Knowing what you're doing

## nginx
You can either set a new location (path/directory) or new server (subdomain) for it:

### location
You basically just have to add this location context to your existing config:
```nginx
location /Cyrkensia/ { #Replace with your preferred name/path
    alias /path/to/CDN-root;
  # auth_basic "Access to password-protected files";
  # auth_basic_user_file /etc/cyrkensia/.htpasswd;
}
```

### server
For this one it's almost the same as [location](#location) with the exception that you need a folder, where hostinfo.php, robots.txt, favicon.ico, etc. are located in and acts like a normal server.
```nginx
server {
    #--Add TLS capability--
    #listen [::]:443 ssl;
    #listen 443 ssl;
    #ssl_certificate /path/to/certificate;
    #ssl_certificate_key /path/to/key;
    #ssl_protocols TLSv1.2 TLSv1.3;
    #--END--
    server_name cyrkensia.yourdomain.com; #Replace with pereferred subdomain name and actual hostname
    root /etc/cyrkensia/html;
    location /musicCDN { #Replace with your preferred name/path
        alias /etc/cyrkensia/musicCDN;
      # auth_basic "Access to password protected files";
      # auth_basic_user_file /etc/cyrkensia/.htpasswd;
    }
    location /hostinfo.php { #Change the location to which URI would match your hostinfo.php
        auth_basic off; #Neccesary so others can read metadata about your server
    }
    location ~ \.php$ {
        include snippets/fastcgi-php.conf;
        # With php-fpm (or other unix sockets):
        fastcgi_pass unix:/run/php/php7.3-fpm.sock; #change version to your php version
   #    # With php-cgi (or other tcp sockets):
   #    fastcgi_pass 127.0.0.1:9000;
    }
}
```

### MIME-types
Make sure your MIME-type file looks like this:  
`/etc/nginx/mime.types`:
```nginx
types {
    audio/mpeg               mp3;
    audio/ogg                ogg oga;
    audio/m4a                m4a;
    audio/wav                wav;
    audio/flac               flac;
    audio/aac                aac;
    audio/opus               opus;
    audio/x-ms-wma           wma;
}
```

## Configuring the hostinfo
Create the main config inside `/etc/cyrkensia/config.json`. You only need to make changes to this file.  
An example of it should look like this:
```js
{
    "vendorName": "Party Royale Radio", //your host name
    "root": "/etc/cyrkensia/musicCDN", //path to your music root,
    "uuid": "00000000-0000-0000-0000-000000000000", //version 4 UUID
    "CDNroot": "", //the name of your path (e.g. myEpicMusic); choose empty if using server method
    "hosticon": "default", //your perfered icon key
    "passwordLocked": true //tell client if authorization is needed
}
```

The `hostinfo.php` must be inside the main root of your server (e.g. `/etc/cyrkensia/html` for [server](#server) method or `/var/www/html` for [location](#location)).