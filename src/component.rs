use iced::Element;

use crate::theme::Base16Theme;

pub mod listing;
pub mod search;

type Component = Element<'static, crate::Message, Base16Theme, iced::Renderer>;
