pub mod scrolled_column;

#[macro_export]
macro_rules! scrolled_column {
    () => (
        $crate::widget::scrolled_column::ScrolledColumn::new()
    );
    ($($x:expr),+ $(,)?) => (
      $crate::widget::scrolled_column::ScrolledColumn::with_children([$(iced::Element::from($x)),+])
    );
}
