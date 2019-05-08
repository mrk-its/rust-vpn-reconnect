#[macro_use]
extern crate serde_derive;

extern crate serde_yaml;

mod networkmanager;
mod networkmanagersettings;
mod networkmanagersettingsconnection;

use dbus::{Connection, BusType, SignalArgs};
use std::error::Error;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::fs::File;
use std::env;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

use networkmanager::OrgFreedesktopDBusPropertiesPropertiesChanged;
use networkmanager::OrgFreedesktopNetworkManager;
use networkmanager::OrgFreedesktopDBusProperties;
use networkmanagersettings::OrgFreedesktopNetworkManagerSettings;
use networkmanagersettingsconnection::OrgFreedesktopNetworkManagerSettingsConnection;

const SHORT_TIMEOUT: u32 = 1000;
const LONG_TIMEOUT: u32 = 60000;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct VPNConfig {
    #[serde(default)]
    always: bool,
    #[serde(default)]
    connections: BTreeSet<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    log_level: String,
    #[serde(default)]
    reconnect_delay: u64,
    vpn: BTreeMap<String, VPNConfig>,
}

fn activate_vpn_connections(c: &Connection, config: &Config) -> Result<(), Box<Error>> {
    let nm = c.with_path("org.freedesktop.NetworkManager", "/org/freedesktop/NetworkManager", 5000);
    let nm_settings = c.with_path("org.freedesktop.NetworkManager", "/org/freedesktop/NetworkManager/Settings", 5000);

    let active_connections:BTreeSet<String> = nm.get_active_connections()?.iter().map(|conn| {
        match c.with_path("org.freedesktop.NetworkManager", conn, 5000)
                  .get("org.freedesktop.NetworkManager.Connection.Active", "Id") {
            Ok(variant) => variant.0,
            Err(_) => "".to_string()
        }
    }).collect();
    println!("active connections: {:?}", active_connections);

    for conn_path in nm_settings.list_connections()? {
        let settings = c.with_path("org.freedesktop.NetworkManager", &conn_path, 5000).get_settings()?;
        if settings.get("vpn").is_none() {
            continue
        };
        let id = settings.get("connection").unwrap().get("id").unwrap().0.as_str();
        if let Some(id) = id {
            let id = String::from(id);
            if active_connections.contains(&id) {
                println!("{:?} is active, skipping", id);
                continue;
            }
            if let Some(vpn_cfg) = config.vpn.get(&id) {
                let has_req_conn = vpn_cfg.connections.intersection(&active_connections).next().is_some();
                if has_req_conn || vpn_cfg.always {
                    println!("activate_connection: {:?}: {:?}", conn_path, id);
                    let _result = nm.activate_connection(conn_path, dbus::Path::from("/"), dbus::Path::from("/"));
                }
            }
        }
    }
    Ok(())
}

fn dbus_connect() -> Result<Connection, Box<Error>> {
    let connection = Connection::get_private(BusType::System)?;
    let mstr = OrgFreedesktopDBusPropertiesPropertiesChanged::match_str(Some(&"org.freedesktop.NetworkManager".into()), None);
    connection.add_match(&mstr).unwrap();
    Ok(connection)
}

fn reconnect_vpn_loop(config: &Config) -> Result<(), Box<Error>> {
    let mut t = SystemTime::now();
    let mut timeout = SHORT_TIMEOUT;
    let connection = dbus_connect()?;
    loop {
        if !connection.is_connected() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "dbus disconnected")))
        }
        for msg in connection.incoming(timeout) {
            match (msg.path(), OrgFreedesktopDBusPropertiesPropertiesChanged::from_message(&msg)) {
                (Some(path), Some(props)) => {
                    if props.changed_properties.contains_key("ActiveConnections") {
                        println!("{:?} {:?}", path , props);
                        t = SystemTime::now();
                        timeout = SHORT_TIMEOUT;
                        break;
                    }
                },
                _ => ()
            }
        }
        let dt = SystemTime::now().duration_since(t).unwrap_or(Duration::from_secs(0));
        if dt < Duration::from_secs(5) {
            activate_vpn_connections(&connection, &config)?;
        } else {
            timeout = LONG_TIMEOUT;
        }
    }
}

fn main() -> Result<(), Box<Error>> {
    let config: Config = {
        let config_path = &format!("{home}/.config/rust-vpn-reconnect/rust-vpn-reconnect.yml", home=env::var("HOME")?);
        let config_file = File::open(Path::new(config_path)).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("config file not found at {}", config_path)
            )
        })?;
        serde_yaml::from_reader(&config_file).unwrap()
    };
    println!("{:#?}", config);
    loop {
        let result = reconnect_vpn_loop(&config);
        println!("{:?}", result);
        sleep(Duration::from_secs(config.reconnect_delay));
    }
}
