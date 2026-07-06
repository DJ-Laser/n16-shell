use iced::Element;
use n16_core::theme::Base16Theme;

pub mod match_entry;
pub mod provider_section;
pub mod search;

type Component = Element<'static, crate::launcher::gui::Message, Base16Theme, iced::Renderer>;
