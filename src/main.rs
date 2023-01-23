use bcrypt::hash;
use chrono::Duration;
use clap::Parser;
use human_duration::human_duration;

const RAW_PASSWORD: &str = "e54847adfdc8fc9e7a0fc06e";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'M', long, default_value_t = 24)]
    max_cost: u32,
    #[arg(short = 'm', long, default_value_t = 4)]
    min_cost: u32,
    #[arg(short = 'c', long)]
    cost: Option<u32>,
    // TODO
    //#[arg(short = 't', long)]
    //max_time: Option<u32>,
    //#[arg(short = 'T', long)]
    //min_time: Option<u32>,
    #[arg(short = 'i', long, default_value_t = 1)]
    iterations: u32,
}

fn main() {
    let cli = Cli::parse();
    if cli.min_cost > cli.max_cost {
        eprintln!("min_cost must be less than max_cost");
        return;
    }
    if cli.iterations < 1 {
        eprintln!("iterations must be greater than 0");
        return;
    }

    if let Some(cost) = cli.cost {
        match cli.iterations {
            1 => test_one(cost),
            _ => test_iterations(cost, cli.iterations),
        }
    } else {
        for cost in cli.min_cost..=cli.max_cost {
            match cli.iterations {
                1 => test_one(cost),
                _ => test_iterations(cost, cli.iterations),
            }
        }
    }
}

fn test_one(cost: u32) {
    let span = Duration::span(|| {
        let _ = hash(RAW_PASSWORD, cost);
    });
    println!(
        "cost: {} duration: {}",
        cost,
        human_duration(&span.to_std().unwrap())
    );
}

fn test_iterations(cost: u32, iterations: u32) {
    let span = Duration::span(|| {
        for _ in 0..iterations {
            let _ = hash(RAW_PASSWORD, cost);
        }
    });
    println!(
        "iterations: {} cost: {} avg duration: {} (total: {})",
        iterations,
        cost,
        human_duration(&(span.to_std().unwrap()/iterations)),
        human_duration(&span.to_std().unwrap())
    );
}
