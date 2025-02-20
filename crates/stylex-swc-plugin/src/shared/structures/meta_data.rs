use indexmap::IndexMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::shared::structures::injectable_style::InjectableStyle;

use super::injectable_style::InjectableStyleBase;

fn f64_to_int<S>(priority: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  if priority.fract() == 0.0 {
    return serializer.serialize_i32(*priority as i32);
  }

  serializer.serialize_f64(*priority)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]

pub(crate) struct MetaData {
  class_name: String,
  style: InjectableStyleBase,
  #[serde(serialize_with = "f64_to_int")]
  priority: f64,
}

impl MetaData {
  pub(crate) fn new(class_name: String, injectable_style: InjectableStyle) -> Self {
    Self {
      class_name,
      priority: injectable_style.priority.unwrap(),
      style: InjectableStyleBase::from(injectable_style),
    }
  }
  pub(crate) fn _get_style(&self) -> &InjectableStyleBase {
    &self.style
  }

  pub(crate) fn get_css(&self) -> &str {
    self.style.ltr.as_str()
  }

  pub(crate) fn get_css_rtl(&self) -> Option<&String> {
    self.style.rtl.as_ref()
  }

  pub(crate) fn get_class_name(&self) -> &str {
    &self.class_name
  }

  pub(crate) fn get_priority(&self) -> &f64 {
    &self.priority
  }

  pub(crate) fn convert_from_injected_styles_map(
    injected_styles_map: &IndexMap<String, Box<InjectableStyle>>,
  ) -> Vec<MetaData> {
    injected_styles_map
      .into_iter()
      .map(|(class_name, injectable_style)| {
        MetaData::new(class_name.clone(), *injectable_style.clone())
      })
      .collect::<Vec<MetaData>>()
  }
}
