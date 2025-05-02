//! Entry point into the ttop binary.
//!
//! The main module creates the view and associates the model and controllers with their respective
//! view panes.

mod controller;
mod model;
mod top;
mod view;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// run in non-interactive batch mode
    #[arg(short = 'b', long)]
    batch_mode: Option<bool>,
    /// reverse last remembered 'c' state
    #[arg(short = 'c', long)]
    cmdline_toggle: Option<bool>,
    /// iterative delay as SECS [.TENTHS]
    #[arg(short = 'd', long)]
    delay: Option<usize>,
    /// set mem as: k,m,g,t,p,e for SCALE
    #[arg(short = 'E', long)]
    scale_summary_mem: Option<char>,
    /// set mem with: k,m,g,t,p for SCALE
    #[arg(short = 'e', long)]
    scale_task_mem: Option<char>,
    /// show tasks plus all their threads
    #[arg(short = 'H', long)]
    threads_show: Option<bool>,
    /// reverse last remembered 'i' state
    #[arg(short = 'i', long)]
    idle_toggle: Option<bool>,
    /// exit on maximum iterations NUMBER
    #[arg(short = 'n', long)]
    iterations: Option<usize>,
    /// output all field names, then exit
    #[arg(short = 'O', long)]
    list_fields: Option<bool>,
    /// force sorting on this named FIELD
    #[arg(short = 'o', long)]
    sort_override: Option<String>,
    /// monitor only the tasks in the PIDLIST
    #[arg(short = 'p', long)]
    pid: Option<Vec<usize>>,
    /// reverse last remembered 'S' state
    #[arg(short = 'S', long)]
    accum_time_toggle: Option<bool>,
    /// run with secure mode restrictions
    #[arg(short = 's', long)]
    secure_mode: Option<bool>,
    /// show only processes owned by USER
    #[arg(short = 'U', long)]
    filter_any_user: Option<String>,
    /// show only processes owned by USER
    #[arg(short = 'u', long)]
    filter_only_euser: Option<String>,
    /// change print width [,use COLUMNS]
    #[arg(short = 'w', long)]
    width: Option<usize>,
    /// reverse last remembered '1' state
    #[arg(short = '1', long)]
    single_cpu_toggle: Option<bool>,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
