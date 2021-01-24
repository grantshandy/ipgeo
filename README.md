# ipgeocli
A CLI tool that finds the location of IP addresses. Built to demonstrate my library `ipgeolocate`.

```
ipgeo 0.1.1
Grant H. <grantshandy@gmail.com>
Finds IP locations

USAGE:
    ipgeo [FLAGS] [OPTIONS] [--] [ADDRESS]

FLAGS:
    -h, --help       Prints help information
    -s, --silent     run without extra output
    -V, --version    Prints version information
    -v, --verbose    run with verbose output

OPTIONS:
    -f, --fields <fields>...    choose what fields to print about the IP address. [possible values: ip, latitude, longitude, city, region, country, timezone, method]
    -m, --method <SERVICE>      choose Geolocation API, if not set it defaults to ipapi. [possible values: ipwhois, ipapi, ipapico, freegeoip]

ARGS:
    <ADDRESS>    what IP address to look up, if none are selected your IP address will be chosen

```
