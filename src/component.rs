use iced::Element;

use crate::theme::Base16Theme;

pub mod listing;

type Component = Element<'static, crate::Message, Base16Theme>;
