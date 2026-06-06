pub mod article;
pub mod calendar;
pub mod download;

pub use article::{
    Announcement, AnnouncementPage, Article, ArticleColumn, ArticlePage, Institution,
    get_announcements, get_column_articles, get_institutions, list_all_column_articles,
    read_announcement, read_article,
};
pub use calendar::{Calendar, get_calendar};
pub use download::{DownloadFile, list_article_download_files};

pub(crate) const SITE_BASE_URL: &str = "https://jw.nju.edu.cn/";
pub(crate) const SITE_ID: &str = "414";
