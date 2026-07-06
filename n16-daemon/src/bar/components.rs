use n16_core::theme::Base16Theme;

use crate::bar::Message;

pub mod clock;

type Component = iced::Element<'static, Message, Base16Theme, iced::Renderer>;
