# ipgeo
[![Crates.io](https://img.shields.io/crates/v/ipgeo.svg)](https://crates.io/crates/ipgeo)
[![API](https://docs.rs/ipgeo/badge.svg)](https://docs.rs/ipgeo)
[![Crates.io](https://img.shields.io/crates/d/ipgeo)](https://crates.io/crates/ipgeo)
[![AUR](https://img.shields.io/aur/version/ipgeo-git)](https://aur.archlinux.org/packages/ipgeo-git/)

A CLI tool that finds the location of IP addresses. Built to demonstrate my library [`ipgeolocate`](https://github.com/grantshandy/ipgeolocate).


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
ipgeo 0.1.8
Grant Handy <grantshandy@gmail.com>
Finds IP Information

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
