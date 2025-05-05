use std::{any::TypeId, hash::Hash};

use iced::advanced::subscription;
use iced::futures;
use iced::window;
use iced::Subscription;
use iced_futures::subscription::{Event, Recipe};
use iced_futures::MaybeSend;

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
    subscription::from_recipe(FilterMap::new(
      recipe,
      window_filter.clone(),
      map_fn.clone(),
    ))
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

impl<A, B, F> FilterMap<A, B, F>
where
  F: Fn(A) -> B + 'static,
{
  fn new(recipe: Box<dyn Recipe<Output = A>>, window_filter: Vec<window::Id>, map_fn: F) -> Self {
    Self {
      recipe,
      window_filter,
      map_fn,
    }
  }
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
    use futures::future;
    use futures::StreamExt;

    let map_fn = self.map_fn;
    let window_filter = self.window_filter;

    let input = input.filter(move |event| match event {
      Event::Interaction {
        window,
        event: _,
        status: _,
      } => future::ready(window_filter.contains(window)),
    });

    Box::pin(self.recipe.stream(input.boxed()).map(map_fn))
  }
}
