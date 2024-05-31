use swc_core::{
  common::DUMMY_SP,
  css::{
    ast::{
      ComponentValue, Declaration, DeclarationName, Dimension, Function, Ident, Length,
      ListOfComponentValues, Number, Stylesheet,
    },
    visit::{Fold, FoldWith},
  },
};

use crate::shared::{constants::common::ROOT_FONT_SIZE, utils::common::dashify};

struct CssFolder {
  use_rem_for_font_size: bool,
  parent_key: Option<String>,
}

impl CssFolder {
  fn convert_font_size_to_rem_normalizer<'a>(
    &'a mut self,
    declaration: &'a mut Declaration,
  ) -> &'a mut Declaration {
    if let DeclarationName::Ident(ident) = &declaration.name {
      if ident.value.eq("fontSize") || self.parent_key.as_deref() == Some("fontSize") {
        self.parent_key = Some("fontSize".into());
        declaration.value = declaration.value.clone().fold_children_with(self);
        self.parent_key = None;
      }
    }

    declaration
  }
}

impl Fold for CssFolder {
  fn fold_list_of_component_values(
    &mut self,
    list: ListOfComponentValues,
  ) -> ListOfComponentValues {
    list.fold_children_with(self)
  }

  fn fold_declaration(&mut self, mut declaration: Declaration) -> Declaration {
    let declaration = kebab_case_normalizer(&mut declaration);

    if self.use_rem_for_font_size {
      self.convert_font_size_to_rem_normalizer(declaration);
    }

    declaration.clone().fold_children_with(self)
  }

  fn fold_dimension(&mut self, mut dimension: Dimension) -> Dimension {
    let dimension = timing_normalizer(&mut dimension);
    let dimension = zero_demention_normalizer(dimension);

    dimension.clone().fold_children_with(self)
  }

  fn fold_length(&mut self, mut length: Length) -> Length {
    if self.parent_key == Some("fontSize".into())
      && length.unit.value.eq("px")
      && length.value.value != 0.0
    {
      length = Length {
        value: Number {
          value: length.value.value / ROOT_FONT_SIZE as f64,
          raw: None,
          span: length.span,
        },
        unit: Ident {
          value: "rem".into(),
          raw: None,
          span: length.span,
        },
        span: DUMMY_SP,
      };
    };

    length
  }

  fn fold_function(&mut self, func: Function) -> Function {
    let mut fnc = func;

    // NOTE: only last css fucntion value should be folded
    if let Some(last) = fnc.value.last_mut() {
      *last = last.clone().fold_with(self);
    }

    fnc
  }
}

fn timing_normalizer(dimension: &mut Dimension) -> &mut Dimension {
  match dimension {
    Dimension::Time(time) => {
      if !time.unit.eq("ms") || time.value.value < 10.0 {
        return dimension;
      }

      time.value = Number {
        value: time.value.value / 1000.0,
        raw: None,
        span: DUMMY_SP,
      };

      time.unit = Ident {
        span: DUMMY_SP,
        value: "s".into(),
        raw: None,
      };

      dimension
    }
    _ => dimension,
  }
}

fn kebab_case_normalizer(declaration: &mut Declaration) -> &mut Declaration {
  match &declaration.name {
    DeclarationName::Ident(ident) => {
      if !ident.value.eq("transitionProperty") && !ident.value.eq("willChange") {
        return declaration;
      }
    }
    DeclarationName::DashedIdent(_) => return declaration,
  }

  declaration.value = declaration
    .value
    .clone()
    .into_iter()
    .map(|value| match value {
      ComponentValue::Ident(ident) => {
        let ident = Ident {
          value: dashify(ident.value.as_str()).into(),
          raw: None,
          span: ident.span,
        };

        ComponentValue::Ident(Box::new(ident))
      }
      _ => value,
    })
    .collect();

  declaration
}

pub(crate) fn base_normalizer(ast: Stylesheet, use_rem_for_font_size: bool) -> Stylesheet {
  let mut folder = CssFolder {
    use_rem_for_font_size,
    parent_key: None,
  };
  ast.fold_with(&mut folder)
}

fn zero_demention_normalizer(dimension: &mut Dimension) -> &mut Dimension {
  match dimension {
    Dimension::Length(length) => {
      if length.value.value != 0.0 {
        return dimension;
      }

      length.value = get_zero_demansion_value();
      length.unit = get_zero_demansion_unit();

      dimension
    }
    Dimension::Angle(angle) => {
      if angle.value.value != 0.0 {
        return dimension;
      }

      angle.value = get_zero_demansion_value();

      angle.unit = Ident {
        span: DUMMY_SP,
        value: "deg".into(),
        raw: None,
      };

      dimension
    }
    Dimension::Time(time) => {
      if time.value.value != 0.0 {
        return dimension;
      }

      time.value = get_zero_demansion_value();

      time.unit = Ident {
        span: DUMMY_SP,
        value: "s".into(),
        raw: None,
      };

      dimension
    }
    Dimension::Frequency(frequency) => {
      if frequency.value.value != 0.0 {
        return dimension;
      }

      frequency.value = get_zero_demansion_value();
      frequency.unit = get_zero_demansion_unit();

      dimension
    }
    Dimension::Resolution(resolution) => {
      if resolution.value.value != 0.0 {
        return dimension;
      }

      resolution.value = get_zero_demansion_value();
      resolution.unit = get_zero_demansion_unit();

      dimension
    }
    Dimension::Flex(flex) => {
      if flex.value.value != 0.0 {
        return dimension;
      }

      flex.value = get_zero_demansion_value();
      flex.unit = get_zero_demansion_unit();

      dimension
    }
    Dimension::UnknownDimension(unknown) => {
      if unknown.value.value != 0.0 {
        return dimension;
      }

      unknown.value = get_zero_demansion_value();
      unknown.unit = get_zero_demansion_unit();

      dimension
    }
  }
}

fn get_zero_demansion_value() -> Number {
  Number {
    value: 0.0,
    raw: None,
    span: DUMMY_SP,
  }
}

fn get_zero_demansion_unit() -> Ident {
  Ident {
    value: "".into(),
    raw: None,
    span: DUMMY_SP,
  }
}
