use lsp_types::{
    notification::Notification, request::Request, CodeActionKind, Position, Range,
    TextDocumentIdentifier
};
use serde::{Deserialize, Serialize};

pub enum Tab {}

impl Request for Tab {
    type Params = TabParams;
    type Result = ();
    const METHOD: &'static str = "web-browser-lsp/tab";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TabParams {}

// enum ConnectBrowser
