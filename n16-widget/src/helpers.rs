#[macro_export]
macro_rules! scrolled_column {
  () => (
      $crate::ScrolledColumn::new()
  );
  ($($x:expr),+ $(,)?) => (
    $crate::widget::scrolled_column::ScrolledColumn::with_children([$(iced::Element::from($x)),+])
  );
}
