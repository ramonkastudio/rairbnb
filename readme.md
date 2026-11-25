# RAirbnb — Airbnb Web Scraper in Rust

RAirbnb is a web scraping project written in **Rust**. Its purpose is to collect accommodation listing data from [Airbnb](https://www.airbnb.com/) and demonstrate how web scraping can be implemented using Rust.

## Technologies

The project is built with Rust and uses the following crates:

| Crate | Description |
|---|---|
| [clap](https://crates.io/crates/clap) | Command-line argument parsing using derive macros. |
| [csv](https://crates.io/crates/csv) | Reading and writing CSV files. |
| [serde](https://crates.io/crates/serde) | Serialization and deserialization of data structures. |
| [reqwest](https://crates.io/crates/reqwest) | Sending HTTP requests. |
| [thirtyfour](https://crates.io/crates/thirtyfour) | Browser automation through the WebDriver protocol. |
| [tokio](https://crates.io/crates/tokio) | Asynchronous runtime for executing concurrent tasks. |
| [undetected-chromedriver](https://crates.io/crates/undetected-chromedriver) | ChromeDriver integration for browser automation. |
| [url](https://crates.io/crates/url) | URL parsing and manipulation. |

## Prerequisites

Before running the scraper, make sure you have:

- [Rust](https://www.rust-lang.org/tools/install) installed.
- [Chrome for Testing](https://googlechromelabs.github.io/chrome-for-testing/).
- ChromeDriver compatible with your Chrome version.

### Start ChromeDriver

Before running the scraper, start ChromeDriver in a separate terminal:

```bash
./chromedriver --port=51877
```

Keep this process running while using the scraper.

## Build

To compile the project in release mode, run:

```bash
cargo build --release
```

Rust will generate an executable in the `target/release/` directory:

| Operating System | Executable |
|---|---|
| Linux | `target/release/rairbnb` |
| macOS | `target/release/rairbnb` |
| Windows | `target/release/rairbnb.exe` |

## Usage

Once the project is compiled and ChromeDriver is running, execute the scraper:

```bash
./rairbnb --country spain
```

### Command-line Arguments

Currently, the scraper supports the following argument:

| Argument | Description | Example |
|---|---|---|
| `--country` | Country to search for Airbnb listings. | `--country spain` |

## Example Output

The scraper exports the collected data to a CSV file named `airbnb.csv`, located in the repository root directory.

The CSV file contains the following fields:

| Field | Description |
|---|---|
| `title` | Listing title. |
| `description` | Short description of the accommodation. |
| `host` | Host type or listing label. |
| `availability` | Available dates displayed in the listing. |
| `price` | Total price displayed for the selected dates. |
| `star` | Listing rating and review count, or `New` for new listings. |

### CSV Preview

```csv
title,description,host,availability,price,star
"Hotel Turia Badajoz","Hotel in Badajoz","Featured hotel","Dec 1 – 6","€532 total","New"
"Home in Torreperogil","Las Pilistras Rural House","Individual host","Dec 13 – 18","€1,150 total","4.93 (27)"
```

You can open `airbnb.csv` using any spreadsheet application, such as Microsoft Excel, LibreOffice Calc, or Google Sheets.

## Project Status

The following features are implemented or planned:

- [x] Basic Airbnb scraping.
- [ ] Skip listings without a title.
- [ ] Extract listing URLs.
- [ ] Store prices as numeric values.
- [ ] Extract listing images.
- [ ] Add city-based search.
- [ ] Add date-based search.
- [ ] Add support for search filters.

## Notes

Airbnb may change its website structure, which can affect the scraper's functionality. Scraping results and the availability of listing information may also vary depending on the website's content and behavior.