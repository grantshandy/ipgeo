use clap::{crate_version, App, Arg, ArgMatches};
use dns_lookup::{lookup_host, getnameinfo};
use ipgeolocate::Locator;
use std::net::{Ipv4Addr, Ipv6Addr, IpAddr, SocketAddr};
use ureq::get;

// A simple CLI application for getting the city and country that an IP is located in.
fn main() {
    // Set CLI application details through clap.
    let matches = App::new("ipgeo")
        .version(crate_version!())
        .author("Grant H. <grantshandy@gmail.com>")
        .about("Finds IP locations")
        .arg(
            Arg::with_name("ADDRESS")
            .help("What IP or DNS address to look up, if none are selected then your network IP address will be chosen")
            .required(false)
            .index(1)
        )
        .arg(
            Arg::with_name("method")
            .long("method")
            .short("m")
            .help("Choose Geolocation API, if not set it defaults to ipapi.")
            .required(false)
            .takes_value(true)
            .value_name("SERVICE")
            .possible_value("ipwhois")
            .possible_value("ipapi")
            .possible_value("ipapico")
            .possible_value("freegeoip")
        )
        .arg(
            Arg::with_name("silent")
            .long("silent")
            .short("s")
            .help("Run without extra output")
            .required(false)
            .takes_value(false)
        )
        .arg(
            Arg::with_name("vertical")
            .long("vertical")
            .help("Print fields vertically. Chosen by default.")
            .required(false)
            .conflicts_with("horizontal")
            .requires("fields")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("horizontal")
            .long("horizontal")
            .help("Print fields horizontally.")
            .required(false)
            .conflicts_with("vertical")
            .requires("fields")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Run with verbose output")
            .required(false)
            .takes_value(false)
        )
        .arg(
            Arg::with_name("fields")
            .long("fields")
            .short("f")
            .help("Choose what fields to print about the IP address.")
            .takes_value(true)
            .required(false)
            .multiple(true)
            .possible_values(&["ip", "latitude", "longitude", "city", "region", "country", "timezone", "method", "dns"])
        )
        .get_matches();

    // Get IP target from clap, if the user didn't specify anything then use get_network_ip() to find your network IP instead.
    let mut ip: String = match matches.value_of("ADDRESS") {
        Some(value) => value.to_string(),
        None => match get_network_ip() {
            Ok(ok) => {
                if !matches.is_present("silent") {
                    println!("no IP address set, using network IP address \"{}\"", ok);
                }
                ok
            }
            Err(error) => {
                eprintln!("error getting network IP address: {}", error);
                String::from("NONE")
            }
        },
    };

    let orig_ip = &ip.clone();

    // Parse the IP address as an IPv4 or IPv6 to find out which one it is.
    if ip.parse::<Ipv4Addr>().is_ok() {
        if matches.is_present("verbose") {
            println!("detected IPv4 address")
        };
    } else if ip.parse::<Ipv6Addr>().is_ok() {
        if matches.is_present("verbose") {
            println!("detected IPv6 address")
        };
    } else {
        // If it isn't either one of them then look it up as a DNS address.
        if matches.is_present("verbose") {
            println!(
                "neither ipv4 or ipv6 IP address found, looking \"{}\" up as a DNS address",
                ip
            );
        };
        match lookup_host(&ip) {
            Ok(data) => {
                if matches.is_present("verbose") {
                    println!("DNS lookup for {} successful", ip);
                };

                // Go through the detected IP addresses and set IP as the first one it finds.
                for foo in data {
                    if foo.is_ipv4() | foo.is_ipv6() {
                        // I know this is a horrible way to do it but seriously how else should it be done?!
                        // There are multiple IP addresses set to a DNS address, which is good but not for me! AGH!
                        // This means that I'm just getting the first IP address that I can find and just using that crap.
                        ip = foo.to_string();
                        continue;
                    };
                };
            }
            Err(error) => {
                // If it isn't an IPv4, IPv6, or DNS address, give the user some instructions on why they're an idiot.
                eprintln!("can't find any information for \"{}\"", ip);
                eprintln!("this probably means that the value for <ADDRESS> is not an IP address or DNS address");
                eprintln!("DNS lookup error: {}", error);
                std::process::exit(1);
            }
        };
    };

    // Set the service variable. If the user hasn't specified anything then just set it as ipapi.
    // ipapi is probably the best in most situations because it has pretty reliable results and has minute by minute request limits so its pretty hard to break in a script.
    let service = match matches.value_of("method") {
        Some(value) => value,
        None => "ipapi",
    };

    // Once we have all the variables set, we can actually run the Locator and then print the results using print_data().
    match Locator::get(&ip, service) {
        Ok(ip) => print_data(service, matches.clone(), ip, orig_ip),
        Err(error) => eprintln!("error getting location data: {}", error),
    };
}

// I'm particularly proud of this function.
// It goes through all the fields set from the user and then print each of the variables individually.
fn print_data(service: &str, app: ArgMatches, ip: Locator, orig_ip: &str) {
    if app.is_present("fields") {
        match app.values_of("fields") {
            Some(data) => {
                for foo in data {
                    let bar = match foo {
                        "dns" => {
                            let dns: String = match orig_ip.parse::<IpAddr>() {
                                Ok(data) => get_dns(data),
                                Err(_) => orig_ip.to_string(),
                            };

                            dns
                        },
                        "city" => ip.city.clone(),
                        "country" => ip.country.clone(),
                        "ip" => ip.ip.clone(),
                        "latitude" => ip.latitude.clone(),
                        "longitude" => ip.longitude.clone(),
                        "region" => ip.region.clone(),
                        "timezone" => ip.timezone.clone(),
                        "method" => service.to_string().clone(),
                        &_ => String::from("NONE"),
                    };

                    if app.is_present("silent") {
                        if app.is_present("vertical") {
                            println!("{}", bar);
                        } else if app.is_present("horizontal") {
                            print!("{} ", bar);
                        } else {
                            println!("{}", bar);
                        };
                    } else {
                        if app.is_present("vertical") {
                            println!("{}: {}", foo, bar);
                        } else if app.is_present("horizontal") {
                            print!("{}: {}, ", foo, bar);
                        } else {
                            println!("{}: {}", foo, bar);
                        };
                    };
                };

                if app.is_present("horizontal") {
                    print!("\n");
                };
            }
            None => eprintln!("field interpretation error, unexpected!"),
        };
    } else {
        // If the user didn't specify any fields, just print it kinda pretty like this:
        println!("{}: {} - {} ({})", service, ip.ip, ip.city, ip.country);
    };
}

// This function gets the users network IP address from "ifconfig.me" a website that just sends you back your IP address.
// There are probably better, faster, ways to do this, but I'm kind of an idiot when it comes to networking so I'm just going to let this slide.
fn get_network_ip() -> std::result::Result<String, std::io::Error> {
    let url = format!("http://ifconfig.me/ip");

    let response = get(&url).call();

    if !response.ok() {
        eprintln!("error connecting to ifconfig.me");
    };

    return response.into_string();
}

fn get_dns(ip: IpAddr) -> String {
    let socket: SocketAddr = (ip, 80).into();

    let name = match getnameinfo(&socket, 0) {
        Ok(data) => data.0,
        Err(e) => {
            eprintln!("Failed to lookup socket {:?}", e);
            "error".to_string()
        }
    };

    return name;
}
