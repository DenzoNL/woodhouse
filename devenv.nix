{ pkgs, ... }:

let
  domain = "woodhouse.test";
  port = 4242;
in
{
  # https://devenv.sh/reference/options/
  packages = with pkgs; [ 
    cargo-watch # Automatically re-runs cargo commands when files change
  ];

  # Enable Rust language support
  languages.rust = {
    enable = true;
    channel = "nixpkgs"; # default
  };

  env = {
    RUST_LOG = "debug"; # Set the logging level for Rust applications
  };

  # Run the application using `cargo watch` when we run `devenv up`
  processes = {
    woodhouse.exec = "cargo-watch -x run";
  };
  
  # This is necessary for Caddy to bind to port 443 without root privileges.
  enterShell = ''
    sudo sysctl -w net.ipv4.ip_unprivileged_port_start=0
  '';

  # Creates reverse proxy from https://woodhouse.test to http://localhost:4242
  #
  # You might need to install the certificate located at .devenv/state/caddy/data/caddy/pki/authorities/local/root.crt
  # You also need to add an entry in your /etc/hosts file pointing woodhouse.test to the IP address of your machine.
  services.caddy = {
    enable = true;
    config = ''
      {
        log {
          level ERROR
        }
      }
    '';
    virtualHosts = {
      "${domain}" = {
        extraConfig = ''
          tls internal
          reverse_proxy :${toString port}
        '';
      };
    };
  };
}