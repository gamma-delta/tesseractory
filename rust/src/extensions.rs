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

pub trait F32Ext {
  /// Signum but it doesn't fucking return 1 on 0
  fn good_sign(self) -> f32;
}

impl F32Ext for f32 {
  fn good_sign(self) -> f32 {
    if self == 0.0 {
      0.0
    } else {
      self.signum()
    }
  }
}
