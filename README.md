# ipgeocli
A CLI tool that finds the location of IP addresses.

```
ipgeo 0.1.0
Grant H. <grantshandy@gmail.com>
Finds IP locations

USAGE:
    ipgeo [FLAGS] [OPTIONS] [--] [ADDRESS]

FLAGS:
    -h, --help       Prints help information
    -s, --silent     Run without verbose output
    -V, --version    Prints version information

OPTIONS:
    -f, --fields <fields>...    Choose what fields to print about the IP address. [possible values: ip, latitude, longitude, city, region, country, timezone, service]
    -m, --method <SERVICE>      Choose Geolocation API, if not set it defaults to ipapi. [possible values: ipwhois, ipapi, ipapico, freegeoip]

ARGS:
    <ADDRESS>    What IP address to look up, if none are selected your IP address will be chosen
```
