use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filename: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let filename = args.filename;
    dotlinker::link(filename);
}
