use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const SITE_BASE_URL: &str = "https://jw.nju.edu.cn/";
const ANNOUNCEMENTS_URL: &str = "https://jw.nju.edu.cn/_wp3services/generalQuery?queryObj=articles";
const SITE_ID: &str = "414";
const ANNOUNCEMENTS_COLUMN_ID: &str = "26263";

const ORDERS: &str = r#"[{"field":"top","type":"desc"},{"field":"new","type":"desc"},{"field":"publishTime","type":"desc"}]"#;
// lp是限制标题字数，多出来的变省略号
const RETURN_INFOS: &str = r#"[{"field":"title","pattern":[{"name":"lp","value":"999"}],"name":"title"},{"field":"f1","name":"f1"},{"field":"publishTime","pattern":[{"name":"d","value":"MM-dd"}],"name":"publishTime"},{"field":"topImg","name":"topImg"},{"field":"newImg","name":"newImg"},{"field":"link","name":"link"}]"#;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementPage {
    pub status: i32,
    pub result: String,
    pub total: u64,
    pub data: Vec<Announcement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    pub id: u64,
    pub title: String,
    /// 通知标签，比如"信息,毕业"
    #[serde(rename = "f1")]
    pub tags: Option<String>,
    pub publish_time: String,
    /// 置顶标的图标URL
    pub top_img: Option<String>,
    pub new_img: Option<String>,
    pub wap_url: Option<String>,
    pub true_wap_url: Option<String>,
    pub url: String,
    pub publisher: Option<String>,
    pub publisher_id: Option<u64>,
    pub visit_count: Option<u64>,
    pub mirc_img_path: Option<String>,
    pub site_art_id: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnnouncementQuery<'a> {
    site_id: &'a str,
    column_id: &'a str,
    page_index: u64,
    rows: u64,
    orders: &'a str,
    return_infos: &'a str,
}

/// 获取教务网「公告通知」列表。
///
/// `page_size` 对应请求中的 `rows`，即一页返回的公告数量。该接口不需要额外
/// header 或 cookie；调用方传入的 `client` 可复用已有 reqwest session。
pub async fn get_announcements(
    client: &reqwest::Client,
    page_index: u64,
    page_size: u64,
) -> reqwest::Result<AnnouncementPage> {
    client
        .post(ANNOUNCEMENTS_URL)
        .form(&AnnouncementQuery {
            site_id: SITE_ID,
            column_id: ANNOUNCEMENTS_COLUMN_ID,
            page_index,
            rows: page_size,
            orders: ORDERS,
            return_infos: RETURN_INFOS,
        })
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}

/// 读取公告页面，并转换为 Markdown。
///
/// `url` 可以是公告列表返回的相对链接或完整链接。Markdown 中的相对链接会基于
/// 最终页面地址补全为绝对链接。
pub async fn read_announcement(client: &reqwest::Client, url: &str) -> Result<String> {
    let url = reqwest::Url::parse(SITE_BASE_URL)
        .context("invalid academic affairs site base URL")?
        .join(url)
        .with_context(|| format!("invalid announcement URL: {url}"))?;

    common::read_html_page(client, url.as_str()).await
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_announcements() {
        let client = reqwest::Client::new();
        let announcements = get_announcements(&client, 1, 100).await.unwrap();
        for a in &announcements.data {
            println!("{} {:?} {}", a.title, a.tags, a.url);
        }

        assert!(announcements.data.len() > 0);
    }
}
