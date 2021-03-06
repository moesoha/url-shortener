# URL Shortener

This is a public copy of *OIH URL Shortener* which runs on `oih.me` currently.

## Get Started

First you should execute the latest SQL script in the directory `./sql` for database.

Then make sure you have installed Rust toolchain.

### For Developing

`cargo run`

### For Production

`cargo run --release`

or

`cargo build --release` and then execute binary file in `./target/release`

## Add a Short Token

### `PUT /`

Params in request body:

  - `url`    Redirect to there
  - `token`  (Optional) custom short token

### DBA

    INSERT INTO `short_token` (`token`,`target`) VALUES (:token,:target)

## Environment Variables

`DATABASE_URL` A URL that describes MySQL server connection information.

`ROCKET_*` Check [Rocket.rs](https://rocket.rs) for details.
