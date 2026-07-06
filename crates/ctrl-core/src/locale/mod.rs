//! 国际化支持模块
//!
//! 提供组件库所需的文案翻译能力。
//! 内置中文（zh-CN）和英文（en-US）两种语言包。

use std::fmt;

/// 语言代码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Lang {
    /// 简体中文
    ZhCN,
    /// 英文
    EnUS,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::ZhCN
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lang::ZhCN => write!(f, "zh-CN"),
            Lang::EnUS => write!(f, "en-US"),
        }
    }
}

/// 组件通用文案 key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LocaleKey {
    Confirm,
    Cancel,
    Close,
    Search,
    Reset,
    NoData,
    Loading,
    PrevPage,
    NextPage,
    Total,
    Items,
    Ok,
    Yes,
    No,
    Clear,
    SelectAll,
    UnselectAll,
}

impl LocaleKey {
    /// 根据语言获取文案
    pub fn text(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::ZhCN => match self {
                LocaleKey::Confirm => "确定",
                LocaleKey::Cancel => "取消",
                LocaleKey::Close => "关闭",
                LocaleKey::Search => "搜索",
                LocaleKey::Reset => "重置",
                LocaleKey::NoData => "暂无数据",
                LocaleKey::Loading => "加载中...",
                LocaleKey::PrevPage => "上一页",
                LocaleKey::NextPage => "下一页",
                LocaleKey::Total => "共",
                LocaleKey::Items => "条",
                LocaleKey::Ok => "确定",
                LocaleKey::Yes => "是",
                LocaleKey::No => "否",
                LocaleKey::Clear => "清空",
                LocaleKey::SelectAll => "全选",
                LocaleKey::UnselectAll => "取消全选",
            },
            Lang::EnUS => match self {
                LocaleKey::Confirm => "Confirm",
                LocaleKey::Cancel => "Cancel",
                LocaleKey::Close => "Close",
                LocaleKey::Search => "Search",
                LocaleKey::Reset => "Reset",
                LocaleKey::NoData => "No data",
                LocaleKey::Loading => "Loading...",
                LocaleKey::PrevPage => "Prev",
                LocaleKey::NextPage => "Next",
                LocaleKey::Total => "Total",
                LocaleKey::Items => "items",
                LocaleKey::Ok => "OK",
                LocaleKey::Yes => "Yes",
                LocaleKey::No => "No",
                LocaleKey::Clear => "Clear",
                LocaleKey::SelectAll => "Select all",
                LocaleKey::UnselectAll => "Deselect all",
            },
        }
    }
}

/// 获取文案的便捷函数
pub fn t(key: LocaleKey, lang: Lang) -> &'static str {
    key.text(lang)
}
