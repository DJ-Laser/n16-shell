use std::hash::Hash;

use iced::Subscription;
use iced::advanced::subscription;
use iced::futures;
use iced::window;
use iced_futures::subscription::Recipe;

pub fn wrap_subscription<A, B>(
  subscription: Subscription<A>,
  window_filter: Vec<window::Id>,
  map_fn: fn(A) -> B,
) -> Subscription<B>
where
  A: 'static,
  B: 'static,
{
  let recipes = subscription::into_recipes(subscription);

  Subscription::batch(recipes.into_iter().map(move |recipe| {
    subscription::from_recipe(WindowFilter {
      recipe,
      window_filter: window_filter.clone(),
    })
  }))
  .with(map_fn)
  .map(|(map_fn, v)| map_fn(v))
}

struct WindowFilter<T> {
  recipe: Box<dyn Recipe<Output = T>>,
  window_filter: Vec<window::Id>,
}

impl<T> Recipe for WindowFilter<T>
where
  T: 'static,
{
  type Output = T;

  fn hash(&self, state: &mut iced_futures::subscription::Hasher) {
    self.window_filter.hash(state);
    self.recipe.hash(state);
  }

  fn stream(
    self: Box<Self>,
    input: iced_futures::subscription::EventStream,
  ) -> iced_futures::BoxStream<Self::Output> {
    use futures::StreamExt;
    use futures::future;

    let window_filter = self.window_filter;

    let input = input.filter(move |event| match event {
      subscription::Event::Interaction {
        window,
        event: e,
        status: _,
      } => {
        if let iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) = e {
          println!("Key {key:?} for window {window:?}");
        }

        if let iced::Event::Window(iced::window::Event::Focused) = e {
          println!("Focus for window {window:?}");
        }

        if let iced::Event::Window(iced::window::Event::Unfocused) = e {
          println!("Unfocus for window {window:?}");
        }

        future::ready(window_filter.contains(window))
      }
      subscription::Event::SystemThemeChanged(_) => future::ready(true),
    });

    Box::pin(self.recipe.stream(input.boxed()))
  }
}
