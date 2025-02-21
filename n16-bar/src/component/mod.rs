use iced::Element;
use n16_theme::Base16Theme;

pub mod clock;

type Component = Element<'static, crate::Message, Base16Theme, iced::Renderer>;
