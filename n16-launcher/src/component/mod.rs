use iced::Element;
use n16_theme::Base16Theme;

pub mod listing;
pub mod search;

type Component = Element<'static, crate::Message, Base16Theme, iced::Renderer>;
