use ipgeolocate::Locator;
use clap::{App, Arg, crate_version, ArgMatches};
use ureq::get;

// A simple CLI application for getting the city and country that an IP is located in.
fn main() {
    let matches = App::new("ipgeo")
        .version(crate_version!())
        .author("Grant H. <grantshandy@gmail.com>")
        .about("Finds IP locations")
        .arg(
            Arg::with_name("ADDRESS")
                .help("What IP address to look up, if none are selected your IP address will be chosen")
                .required(false)
                .index(0)
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
                .help("Run without verbose output")
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
                .possible_values(&["ip", "latitude", "longitude", "city", "region", "country", "timezone", "service"])
        )
        .get_matches();

    let ip: String = match matches.value_of("ADDRESS") {
        Some(value) => value.to_string(),
        None => {
            match get_ip() {
                Ok(ok) => {
                    if !matches.is_present("silent"){
                        println!("No IP address set, using network IP address \"{}\"", ok);
                    }
                    ok
                },
                Err(error) => {
                    eprintln!("error getting network IP address: {}", error);
                    String::from("NONE")
                },
            }
        },
    };

    let service = match matches.value_of("method") {
        Some(value) => value,
        None => "ipapi",
    };

    match Locator::get(&ip, service) {
        Ok(ip) => print_data(service, matches.clone(), ip),
        Err(error) => eprintln!("Error getting data: {}", error),
    };
}

fn print_data(service: &str, app: ArgMatches, ip: Locator) {
    if app.is_present("fields") {
        match app.values_of("fields") {
            Some(data) => {
                for foo in data {
                    let bar = match foo {
                        "city" => ip.city.clone(),
                        "country" => ip.country.clone(),
                        "ip" => ip.ip.clone(),
                        "latitude" => ip.latitude.clone(),
                        "longitude" => ip.longitude.clone(),
                        "region" => ip.region.clone(),
                        "timezone" => ip.timezone.clone(),
                        "service" => service.to_string().clone(),
                        &_ => String::from("NONE"),
                    };

                    println!("{}", bar);
                };
            },
            None => eprintln!("ERROR WITH FIELDS!"),
        };
    } else {
        println!("{}: {} - {} ({})", service, ip.ip, ip.city, ip.country);
    };
}

fn get_ip() -> std::result::Result<String, std::io::Error> {
    let url = format!("http://ifconfig.me/ip");

    let response = get(&url).call();

    if !response.ok() {
        eprintln!("error connecting to ifconfig.me");
    };

    return response.into_string();
}
