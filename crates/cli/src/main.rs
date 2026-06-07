use anyhow::Result;
use clap::{Parser, Subcommand};

mod academic_affairs;
mod auth;
mod ehall;
mod exchange_system;
mod itsc;
mod youth_league;

#[derive(Debug, Parser)]
#[command(name = "nju-cli")]
#[command(about = "南京大学站点命令行工具")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// 登录统一认证并缓存 CASTGC cookie。
    Login(auth::LoginCommand),
    /// 读取 HTML 页面并转换为 Markdown。
    #[command(name = "view-html")]
    ViewHtml {
        /// 要读取的 HTML 页面 URL。
        url: String,
    },
    /// 需要 ehall 登录态的服务。
    Ehall {
        #[command(subcommand)]
        command: ehall::EhallCommand,
    },
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
    /// 南大团委最新动态和公告通知。
    #[command(name = "youth-league")]
    YouthLeague {
        #[command(subcommand)]
        command: youth_league::YouthLeagueCommand,
    },
    /// 信息化中心服务说明和正版软件安装教程。
    Itsc {
        #[command(subcommand)]
        command: itsc::ItscCommand,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Command::Login(command) => auth::login(command).await?,
        Command::ViewHtml { url } => {
            let markdown = common::read_html_page(&client, &url).await?;
            println!("{markdown}");
        }
        Command::Ehall { command } => ehall::handle(command).await?,
        Command::AcademicAffairs { command } => academic_affairs::handle(command, &client).await?,
        Command::ExchangeSystem { command } => exchange_system::handle(command, &client).await?,
        Command::YouthLeague { command } => youth_league::handle(command, &client).await?,
        Command::Itsc { command } => itsc::handle(command, &client).await?,
    }

    Ok(())
}
