use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "project-example",
    about = "A SpatialOS worker written in Rust.",
    rename_all = "kebab-case"
)]
pub struct Opt {
    #[structopt(long, short = "w")]
    pub worker_type: String,

    #[structopt(long, short = "i")]
    pub worker_id: Option<String>,

    #[structopt(long, short = "p")]
    pub connect_with_poll: bool,

    #[structopt(parse(from_os_str), long, short)]
    pub log_file: Option<PathBuf>,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "receptionist")]
    Receptionist {
        #[structopt(long, short)]
        connect_with_external_ip: bool,

        #[structopt(long, short)]
        host: Option<String>,

        #[structopt(long, short)]
        port: Option<u16>,
    },

    #[structopt(name = "locator")]
    Locator {
        #[structopt(name = "LOCATOR_TOKEN", long = "locator-token", short = "t")]
        token: String,

        #[structopt(name = "PROJECT_NAME", long = "project-name", short = "n")]
        project_name: String,
    },

    #[structopt(name = "dev-auth")]
    DevelopmentAuthentication {
        #[structopt(name = "DEV_AUTH_TOKEN", long = "dev-auth-token", short = "t")]
        dev_auth_token: String,
    },
}
