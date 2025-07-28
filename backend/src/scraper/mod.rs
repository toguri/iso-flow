//! RSSフィード解析とニュース収集機能
//!
//! このモジュールは、複数のソースからNBAニュースを収集し、
//! トレード関連の情報を抽出する機能を提供します。

pub mod models;
pub mod persistence;
pub mod persistence_trait;
pub mod rss_parser;

#[cfg(test)]
mod persistence_trait_test;

pub use models::*;
pub use persistence::*;
pub use persistence_trait::*;
pub use rss_parser::*;
