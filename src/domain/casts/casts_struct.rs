use crate::domain::casts::cast_type::CastType;
use crate::kernel::error::KernelError;
use crate::kernel::types::core::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Casts {
    pub definitions: HashMap<String, CastType>,
}

impl Casts {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    pub fn add(mut self, field: &str, cast_type: CastType) -> Self {
        self.definitions.insert(field.to_string(), cast_type);
        self
    }

    pub fn add_from_str(self, field: &str, cast_type: &str) -> Result<Self, KernelError> {
        let cast = CastType::from_str(cast_type)?;
        Ok(self.add(field, cast))
    }

    pub fn get_cast(&self, field: &str) -> Option<&CastType> {
        self.definitions.get(field)
    }

    pub fn cast_on_load(&self, field: &str, value: &Value) -> Result<Value, KernelError> {
        if let Some(cast_type) = self.get_cast(field) {
            cast_type.cast_from_value(value)
        } else {
            Ok(value.clone())
        }
    }

    pub fn cast_on_save(&self, field: &str, value: &Value) -> Result<Value, KernelError> {
        if let Some(cast_type) = self.get_cast(field) {
            cast_type.cast_to_value(value)
        } else {
            Ok(value.clone())
        }
    }
}

impl Default for Casts {
    fn default() -> Self {
        Self::new()
    }
}

pub trait HasCasts {
    fn casts() -> Casts {
        Casts::new()
    }

    fn cast_attributes_on_load(attributes: &mut HashMap<String, Value>) -> Result<(), KernelError> {
        let casts = Self::casts();
        for (field, value) in attributes.iter_mut() {
            *value = casts.cast_on_load(field, value)?;
        }
        Ok(())
    }

    fn cast_attributes_on_save(attributes: &mut HashMap<String, Value>) -> Result<(), KernelError> {
        let casts = Self::casts();
        for (field, value) in attributes.iter_mut() {
            *value = casts.cast_on_save(field, value)?;
        }
        Ok(())
    }
}
