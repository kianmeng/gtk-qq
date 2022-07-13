use relm4::gtk::gdk::Paintable;
use ricq::client::Token;

pub enum Input {
    Account(String),
    Password(String),
    Login,
    Avatar(Option<Paintable>),
}

pub enum Output {
    Login { account: i64, pwd: String },
    TokenLogin(Token),
    EnableLogin(bool),
}

#[derive(Debug)]
pub(super) enum State {
    NoChange,
    Update,
}

#[derive(Debug, Clone)]
pub(super) enum PwdEntry {
    None,
    Token(Token),
    Password(String),
}

#[derive(Debug, Default)]
pub struct Payload {
    pub account: Option<i64>,
    pub token: Option<Token>,
    pub avatar: Option<Paintable>,
}

impl PwdEntry {
    pub(super) fn is_some(&self) -> bool {
        match self {
            PwdEntry::None => false,
            _ => true,
        }
    }
}
