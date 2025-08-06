# Woodhouse

Woodhouse is an opinionated, work-in-progress lightweight task automation server, written in Rust.
It builds on modern GitOps principles, similar to Argo CD. You simply point Woodhouse to a git repository, and it will
run the tasks defined in it.

It is designed to be very simple, whilst adhering to said principles. It is not a full-on CI/CD tool.
It sits in that uncanny valley for stuff that you don't want in a crontab, but you don't want to put in a CI/CD pipeline either.

## Running the dev environment

You will need Nix and direnv.

```shell
$ direnv allow
$ devenv up
```

The Nix development environment comes with a Caddy reverse proxy. To set this up:

1. Add an entry in your hosts file pointing woodhouse.test to the IP address of your machine.
2. If automatic install fails, you need to install the Caddy root certificate located at .devenv/state/caddy/data/caddy/pki/authorities/local/root.crt into your browser or system certificate store.

## Planned features

- Write tasks in TOML
- Sync tasks using Git
- Define tasks using TOML
- Run tasks automatically on a Cron schedule
- Trigger jobs manually in web UI
- Trigger jobs using webhooks
- View jobs and log output in web UI

## Expectations

This is a hobby project that probably won't even get to a working state. It is simply to satisfy my own curiosity.
Perhaps it will be in a decent working state at some point. Feel free to use it, but don't expect me to fix or build anything.
Don't make your business rely on this lol.

## Contributions

Feel free to submit a PR if you desire!
