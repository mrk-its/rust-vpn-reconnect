# rust-vpn-reconnect

Automatically reconnects NetworkManager's VPN connections based on rules defined in yaml config file.

Example config file (store it in $HOME/.config/rust-vpn-reconnect/rust-vpn-reconnect.yml)
```
log_level: INFO
reconnect_delay: 2
vpn:
  work-vpn:
    connections:
      - work-wifi
  home-vpn:
    connections:
      - home-wifi
```
