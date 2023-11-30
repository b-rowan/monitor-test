use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use json;

#[derive(Debug, PartialEq)]
pub enum MinerBackend {
    AntMiner,
    WhatsMiner,
    AvalonMiner,
    Innosilicon,
    Goldshell,
    BraiinsOS,
    VNish,
    Hiveon,
    LuxOS,
    Unknown,
}

pub async fn get_miner(ip: &str) -> Option<MinerBackend> {
    let result = get_miner_backend(ip).await;
    // Do more parsing here, check individual types

    result
}

async fn get_miner_backend(ip: &str) -> Option<MinerBackend> {
    let (_devdetails, _version) = tokio::join!(_get_devdetails(ip), _get_version(ip));

    let devdetails_result = match _devdetails {
        Err(_msg) => {
            // println!("No devdetails result: {}", _msg);
            None
        }
        Ok(devdetails) => {
            let upper_devdetails = devdetails.to_uppercase();
            _parse_miner_backend(&upper_devdetails)
        }
    };
    let version_result = match _version {
        Err(_msg) => {
            // println!("No version result: {}", _msg);
            None
        }
        Ok(version) => {
            let upper_version = version.to_uppercase();
            _parse_miner_backend(&upper_version)
        }
    };


    let miner_type = if devdetails_result.is_some() {
        if devdetails_result != Some(MinerBackend::Unknown) {
            devdetails_result
        } else if version_result.is_some() {
            version_result
        } else if devdetails_result.is_some() {
            devdetails_result
        } else {
            None
        }
    } else if version_result.is_some() {
        version_result
    } else {
        None
    };

    miner_type
}


fn _parse_miner_backend(socket_result: &str) -> Option<MinerBackend> {
    if socket_result.contains("BITMICRO") || socket_result.contains("BTMINER") {
        Some(MinerBackend::WhatsMiner)
    } else if socket_result.contains("BOSER") || socket_result.contains("BOSMINER") {
        Some(MinerBackend::BraiinsOS)
    } else if socket_result.contains("VNISH") {
        Some(MinerBackend::VNish)
    } else if socket_result.contains("HIVEON") {
        Some(MinerBackend::Hiveon)
    } else if socket_result.contains("LUXMINER") {
        Some(MinerBackend::LuxOS)
    } else if socket_result.contains("ANTMINER") && !socket_result.contains("DEVDETAILS") {
        Some(MinerBackend::AntMiner)
    } else if socket_result.contains("INTCHAINS_QOMO") {
        Some(MinerBackend::Goldshell)
    } else if socket_result.contains("AVALON") {
        Some(MinerBackend::AvalonMiner)
    } else {
        Some(MinerBackend::Unknown)
    }
}


async fn _get_version(ip: &str) -> Result<String, String> {
    let mut stream = tokio::net::TcpStream::connect(format!("{}:4028", ip)).await.unwrap();

    let command = String::from("{\"command\":\"version\"}");

    stream.write_all(command.as_bytes()).await.unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await.unwrap();

    let response = String::from_utf8_lossy(&buffer).into_owned();

    parse_result(response)
}
async fn _get_devdetails(ip: &str) -> Result<String, String> {
    let mut stream = tokio::net::TcpStream::connect(format!("{}:4028", ip)).await.unwrap();

    let command = String::from("{\"command\":\"devdetails\"}");

    stream.write_all(command.as_bytes()).await.unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await.unwrap();

    let response = String::from_utf8_lossy(&buffer).into_owned();

    parse_result(response)
}

fn parse_result(result: String) -> Result<String, String>{
    let fixed_result = result.replace('\0', "");


    let parsed = json::parse(&fixed_result).unwrap();

    let success_codes = vec!["S", "I"];


    let success = success_codes.contains(&&parsed["STATUS"][0]["STATUS"].to_string()[..]);

    if success {
        Ok(result)
    } else {
        if parsed.has_key("Msg") {
            Err(parsed["Msg"].to_string())
        } else {
            Err(parsed["STATUS"][0]["Msg"].to_string())
        }
    }
}