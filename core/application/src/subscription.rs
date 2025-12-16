use std::{any::TypeId, hash::Hash};

use iced::Subscription;
use iced::advanced::subscription;
use iced::futures;
use iced::window;
use iced_futures::MaybeSend;
use iced_futures::subscription::Recipe;

pub fn wrap_subscription<A, B, F>(
  subscription: Subscription<A>,
  window_filter: Vec<window::Id>,
  map_fn: F,
) -> Subscription<B>
where
  A: 'static,
  B: 'static,
  F: Fn(A) -> B + 'static + MaybeSend + Clone,
{
  let recipes = subscription::into_recipes(subscription);

  Subscription::batch(recipes.into_iter().map(move |recipe| {
    subscription::from_recipe(FilterMap {
      recipe,
      window_filter: window_filter.clone(),
      map_fn: map_fn.clone(),
    })
  }))
}

struct FilterMap<A, B, F>
where
  F: Fn(A) -> B + 'static,
{
  recipe: Box<dyn Recipe<Output = A>>,
  window_filter: Vec<window::Id>,
  map_fn: F,
}

impl<A, B, F> Recipe for FilterMap<A, B, F>
where
  A: 'static,
  B: 'static,
  F: Fn(A) -> B + 'static + MaybeSend,
{
  type Output = B;

  fn hash(&self, state: &mut iced_futures::subscription::Hasher) {
    TypeId::of::<F>().hash(state);
    self.window_filter.hash(state);
    self.recipe.hash(state);
  }

  fn stream(
    self: Box<Self>,
    input: iced_futures::subscription::EventStream,
  ) -> iced_futures::BoxStream<Self::Output> {
    use futures::StreamExt;
    use futures::future;

    let map_fn = self.map_fn;
    let window_filter = self.window_filter;

    let input = input.filter(move |event| match event {
      subscription::Event::Interaction {
        window,
        event: _,
        status: _,
      } => future::ready(window_filter.contains(window)),
      subscription::Event::SystemThemeChanged(_) => future::ready(true),
    });

    Box::pin(self.recipe.stream(input.boxed()).map(map_fn))
  }
}
