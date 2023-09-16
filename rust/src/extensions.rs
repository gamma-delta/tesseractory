use godot::prelude::*;

pub trait GodotObjectExt {
  fn totally<T>(&self, field: &str) -> T
  where
    T: FromVariant;
}

impl GodotObjectExt for godot::engine::Object {
  fn totally<T>(&self, field: &str) -> T
  where
    T: FromVariant,
  {
    let x: Variant = self.get(field.into());
    match T::try_from_variant(&x) {
      Ok(it) => it,
      Err(ono) => panic!(
        "on {:?}, field {:?} was not a {} : {:?}",
        &self,
        field,
        std::any::type_name::<T>(),
        ono
      ),
    }
  }
}
