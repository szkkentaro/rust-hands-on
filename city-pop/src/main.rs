extern crate csv;
extern crate getopts;
extern crate rustc_serialize;

use getopts::Options;
use std::env;
use std::fs;
use std::path::Path;
use std::error::Error;

#[derive(Debug, RustcDecodable)]
struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

struct PopulationCount {
    city: String,
    country: String,
    count: u64,
}

fn print_usage(program: &str, opts: Options) {
    println!(
        "{}",
        opts.usage(&format!("Usage: {} [options] <data-path> <city>", program))
    );
}

fn search<P: AsRef<Path>>(
    file_path: P,
    city: &str,
) -> Result<Vec<PopulationCount>, Box<Error + Send + Sync>> {
    let mut found = vec![];
    let file = try!(fs::File::open(file_path));
    let mut rdr = csv::Reader::from_reader(file);
    for row in rdr.decode::<Row>() {
        let row = try!(row);
        match row.population {
            None => {}
            Some(count) => if row.city == city {
                found.push(PopulationCount {
                    city: row.city,
                    country: row.country,
                    count: count,
                });
            },
        }
    }
    if found.is_empty() {
        Err(From::from("No mating cities with a population were found."))
    } else {
        Ok(found)
    }
}

// impl<'a, 'b> From<&'b str> for Box<Error + Send + Sync + 'a>

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this usage message.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let data_file = args[1].clone();
    let data_path = Path::new(&data_file);
    let city = args[2].clone();

    match search(&data_path, &city) {
        Ok(pops) => for pop in pops {
            println!("{}, {}, {}", pop.city, pop.country, pop.count);
        },
        Err(err) => println!("{}", err),
    }
    // for pop in search(&data_path, &city) {
    //     println!("{}, {}, {}", pop.city, pop.country, pop.count);
    // }

    // let file = fs::File::open(data_path).unwrap();
    // let mut rdr = csv::Reader::from_reader(file);

    // $ ./target/debug/city-pop ~/Downloads/uscitiespop.csv "alexander city"
    // alexander city, us: 14993
    // for row in rdr.decode::<Row>() {
    //     let row = row.unwrap();
    //     if row.city == city {
    //         println!(
    //             "{}, {}: {:?}",
    //             row.city,
    //             row.country,
    //             row.population.expect("population count")
    //         );
    //     }
    // }
}
