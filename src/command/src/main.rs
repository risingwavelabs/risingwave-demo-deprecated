use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CmdArgs {
    #[clap(short, long)]
    conf: String,

}

fn main() {
    let cli = CmdArgs::parse();
}
