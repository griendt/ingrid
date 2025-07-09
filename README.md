## Ingrid

This is a Telegram bot written in Rust, inspired by [Henk](https://github.com/jvdwetering/Henk).

### How to run Ingrid

* Ensure you have Rust installed (e.g. with [rustup](https://www.rust-lang.org/tools/install)).
* Ensure you have a Telegram token obtained from the [Botfather](https://t.me/botfather).
* Ensure you have a PostgreSQL database set up, e.g. by using the provided `docker-compose.yml`.
* Create a `.env` file with the following variables:
  ```shell
  TELOXIDE_TOKEN=<token>
  POSTGRES_PASSWORD=<secret>
  DATABASE_URL=postgresql://postgres:<secret>@<host>/postgres
  ```
* To run the migrations, run the following commands:
  ```shell 
  cargo install sea-orm-cli
  sea-orm-cli migrate -d ./src/migration up
  ```
* You're all set! Run Ingrid with one of the following commands, depending on your preference:
  ```shell
  cargo run
  docker compose up -d app
  ```
