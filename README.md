# ipgeocli
A CLI tool that finds the location of IP addresses. Built to demonstrate my library `ipgeolocate`.


With `ipgeo` you can get the location for an IP address or a DNS address and do lookups for both of them to get data on the ip, latitude, longitude, city, region, country, timezone, method, and a reverse DNS address.


`ipgeo` was made to work within scripts, so you can use the `--silent` tag to print information without any extra fluff so it's easier to parse.


`ipgeo` offers different methods to get information about IP addresses, and each has their own benefits. By default, ip-api.com is used because it gives accurate results and only limits requests by the minute and not by the day, week, or month.


## Installation
Install it by simply doing
```
$ cargo install ipgeo
```

## Command Line Arguments
```
ipgeo 0.1.4
Grant H. <grantshandy@gmail.com>
Finds IP locations

USAGE:
    ipgeo [FLAGS] [OPTIONS] [--] [ADDRESS]

FLAGS:
    -a, --all           Print all available information
    -h, --help          Prints help information
        --horizontal    Print fields horizontally.
    -s, --silent        Run without extra output
    -V, --version       Prints version information
    -v, --verbose       Run with verbose output

OPTIONS:
    -f, --fields <FIELDS>...    Choose what fields to print about the IP address. [possible values: ip, latitude, longitude, city, region, country, timezone, method, dns]
    -m, --method <SERVICE>      Choose Geolocation API, if not set it defaults to ipapi. [possible values: ipwhois, ipapi, ipapico, freegeoip]

ARGS:
    <ADDRESS>    What IP or DNS address to look up, if none are selected then your network IP address will be chosen
```
