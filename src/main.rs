use std::{env, error, io, process};

use balances::{engine::Engine, input::Tx};

type AnyError = Box<dyn error::Error + Send + Sync + 'static>;

fn main() {
    if let Err(reason) = run() {
        eprintln!("FATAL: {}", reason);
        process::exit(1)
    }
}

fn run() -> Result<(), AnyError> {
    let mut engine = Engine::default();
    let Some(input) = env::args().skip(1).next() else {
        return Err("exactly one argument expected".into());
    };

    eprintln!("processing {}...", input);

    let csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .from_path(input)?;

    for (row_idx, input_row) in csv_reader.into_deserialize::<Tx>().enumerate() {
        let Ok(tx) =
            input_row.inspect_err(|e| eprintln!("[{}] csv deserialize error: {}", row_idx, e))
        else {
            continue;
        };
        eprintln!("processing {:?}...", tx);
        if let Err(reason) = engine.process_tx(tx) {
            eprintln!("[{}] engine processing error: {}", row_idx, reason);
        }
    }

    let stdout = io::stdout().lock();
    let mut csv_writer = csv::WriterBuilder::new().from_writer(stdout);
    for account in engine.accounts() {
        csv_writer.serialize(account)?;
    }
    csv_writer.flush()?;

    Ok(())
}
