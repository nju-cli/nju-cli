use anyhow::Result;
use clap::{Parser, Subcommand};

mod academic_affairs;
mod exchange_system;

#[derive(Debug, Parser)]
#[command(name = "nju-cli")]
#[command(about = "南京大学站点命令行工具")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// 教务网公告通知。
    #[command(name = "academic-affairs")]
    AcademicAffairs {
        #[command(subcommand)]
        command: academic_affairs::AcademicAffairsCommand,
    },
    /// 交换生系统新闻通知和项目。
    #[command(name = "exchange-system")]
    ExchangeSystem {
        #[command(subcommand)]
        command: exchange_system::ExchangeSystemCommand,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Command::AcademicAffairs { command } => academic_affairs::handle(command, &client).await?,
        Command::ExchangeSystem { command } => exchange_system::handle(command, &client).await?,
    }

    Ok(())
}
