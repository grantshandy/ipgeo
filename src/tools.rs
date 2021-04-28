use dns_lookup::getnameinfo;
use crate::Service;
use std::net::{IpAddr, SocketAddr};

// This function gets the users network IP address from "ifconfig.me" a website that just sends you back your IP address.
// There are probably better, faster, ways to do this, but I'm kind of an idiot when it comes to networking so I'm just going to let this slide.
pub async fn get_network_ip() -> String {
    let response = surf::get("http://ifconfig.me/ip").recv_string().await;

    match response {
        Ok(response) => return response,
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
