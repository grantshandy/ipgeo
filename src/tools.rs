use dns_lookup::getnameinfo;
use crate::Service;

// This function gets the users network IP address from "ifconfig.me" a website that just sends you back your IP address.
// There are probably better, faster, ways to do this, but I'm kind of an idiot when it comes to networking so I'm just going to let this slide.
pub fn get_network_ip() -> std::result::Result<String, ureq::Error> {
    let url = format!("http://ifconfig.me/ip");

    let response = ureq::get(&url).call();

    match response {
        Ok(response) => Ok(response.into_string().unwrap()),
        Err(_) => {
            eprintln!("error connecting to ifconfig.me");
            std::process::exit(1);
        }
    }
}

// This is a function for looking up a DNS address that pertains to an IP address.
pub fn get_dns(ip: IpAddr) -> String {
    let socket: SocketAddr = (ip, 80).into();

    let name = match getnameinfo(&socket, 0) {
        Ok(data) => data.0,
        Err(e) => {
            eprintln!("failed to lookup socket {:?}", e);
            std::process::exit(1);
        }
    };

    return name;
}

pub fn match_method(method: &str) -> Service {
    match method {
        "ipapi" => Service::IpApi,
        "ipapico" => Service::IpApiCo,
        "ipwhois" => Service::IpWhois,
        "freegeoip" => Service::FreeGeoIp,
        &_ => panic!("Other method detected"),
    }
}
