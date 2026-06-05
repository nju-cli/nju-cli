use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result, anyhow};
use clap::Subcommand;
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Subcommand)]
pub enum AcademicAffairsCommand {
    /// 列出教务网公告通知，并把公告 id 与 URL 缓存到本地。
    List {
        /// 每页公告数量。
        #[arg(long, default_value_t = 20)]
        page_size: u64,
    },
    /// 根据已缓存的公告 id 输出公告 Markdown 内容。
    View {
        /// 公告 id。需要先执行 list 以缓存 id 与 URL。
        announcement_id: u64,
    },
    /// 根据已缓存的公告 id 下载公告 Markdown 到缓存目录。
    Download {
        /// 公告 id 列表。需要先执行 list 以缓存 id 与 URL。
        announcement_ids: Vec<u64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedAnnouncement {
    id: u64,
    title: String,
    url: String,
}

pub async fn handle(command: AcademicAffairsCommand, client: &reqwest::Client) -> Result<()> {
    match command {
        AcademicAffairsCommand::List { page_size } => {
            let page = academic_affairs::get_announcements(client, 1, page_size)
                .await
                .context("failed to list academic affairs announcements")?;
            let announcements = page
                .data
                .into_iter()
                .map(|announcement| CachedAnnouncement {
                    id: announcement.id,
                    title: announcement.title,
                    url: announcement.url,
                })
                .collect::<Vec<_>>();

            save_announcements(&announcements)?;

            for announcement in announcements {
                println!("{} {}", announcement.id, announcement.title);
            }
        }
        AcademicAffairsCommand::View { announcement_id } => {
            let announcement = find_cached_announcement(announcement_id)?;
            let markdown = academic_affairs::read_announcement(client, &announcement.url)
                .await
                .with_context(|| format!("failed to read announcement {announcement_id}"))?;

            println!("{markdown}");
        }
        AcademicAffairsCommand::Download { announcement_ids } => {
            if announcement_ids.is_empty() {
                return Err(anyhow!("please provide at least one announcement id"));
            }

            let announcements = load_announcements()?;
            let announcements = announcements
                .into_iter()
                .map(|announcement| (announcement.id, announcement))
                .collect::<HashMap<_, _>>();
            let dir = academic_affairs_cache_dir()?;

            for announcement_id in announcement_ids {
                let announcement = announcements.get(&announcement_id).ok_or_else(|| {
                    anyhow!(
                        "announcement id {announcement_id} is not cached; run `nju-cli academic-affairs list --page-size 100` first"
                    )
                })?;
                let markdown = academic_affairs::read_announcement(client, &announcement.url)
                    .await
                    .with_context(|| format!("failed to read announcement {announcement_id}"))?;
                let file_name = format!("{}.md", sanitize_filename::sanitize(&announcement.title));
                let path = dir.join(file_name);

                std::fs::write(&path, markdown)
                    .with_context(|| format!("failed to write {}", path.display()))?;
                println!("{} {}", announcement_id, path.display());
            }
        }
    }

    Ok(())
}

fn find_cached_announcement(announcement_id: u64) -> Result<CachedAnnouncement> {
    load_announcements()?
        .into_iter()
        .find(|announcement| announcement.id == announcement_id)
        .ok_or_else(|| {
            anyhow!(
                "announcement id {announcement_id} is not cached; run `nju-cli academic-affairs list --page-size 100` first"
            )
        })
}

fn save_announcements(announcements: &[CachedAnnouncement]) -> Result<()> {
    let path = announcements_cache_file()?;
    let json = serde_json::to_string_pretty(announcements)
        .context("failed to serialize cached announcements")?;

    std::fs::write(&path, json).with_context(|| format!("failed to write {}", path.display()))
}

fn load_announcements() -> Result<Vec<CachedAnnouncement>> {
    let path = announcements_cache_file()?;
    let json = std::fs::read_to_string(&path).with_context(|| {
        format!(
            "failed to read cached announcements from {}; run `nju-cli academic-affairs list --page-size 100` first",
            path.display()
        )
    })?;

    serde_json::from_str(&json).with_context(|| format!("failed to parse {}", path.display()))
}

fn announcements_cache_file() -> Result<PathBuf> {
    Ok(academic_affairs_cache_dir()?.join("announcements.json"))
}

fn academic_affairs_cache_dir() -> Result<PathBuf> {
    let app_dirs = AppDirs::new(Some("nju-cli"), true)
        .ok_or_else(|| anyhow!("failed to resolve application cache directory"))?;
    let dir = app_dirs.cache_dir.join("academic-affairs");

    std::fs::create_dir_all(&dir).with_context(|| format!("failed to create {}", dir.display()))?;

    Ok(dir)
}
