use std::{collections::HashMap, rc::Rc};

use indexmap::IndexMap;
use swc_core::{
  atoms::Atom,
  ecma::ast::{BindingIdent, Expr},
};

use crate::shared::enums::data_structures::{
  evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
};

use super::{functions::FunctionConfigType, named_import_source::ImportSources};

pub(crate) type FlatCompiledStyles = IndexMap<String, Box<FlatCompiledStylesValue>>;
pub(crate) type EvaluateResultFns =
  IndexMap<String, (Vec<BindingIdent>, IndexMap<String, Box<Expr>>)>;
pub(crate) type EvaluationCallback = Rc<dyn Fn(Vec<Option<EvaluateResultValue>>) -> Expr + 'static>;
pub(crate) type FunctionMapMemberExpression =
  HashMap<ImportSources, Box<HashMap<Atom, Box<FunctionConfigType>>>>;
pub(crate) type FunctionMapIdentifiers = HashMap<Atom, Box<FunctionConfigType>>;
pub(crate) type StylesObjectMap =
  IndexMap<String, Box<IndexMap<String, Box<FlatCompiledStylesValue>>>>;
