use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'p', long, default_value_t = 8080)]
    pub http_port: u16,

    #[arg(short = 's', long, default_value_t = 4343)]
    pub https_port: u16,
}
