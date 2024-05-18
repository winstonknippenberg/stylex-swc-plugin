use std::{collections::HashMap, panic};

use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Id, Ident};
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::shared::structures::functions::FunctionMap;
use crate::shared::structures::named_import_source::ImportSources;
use crate::shared::utils::common::gen_file_based_identifier;
use crate::shared::utils::css::stylex::evaluate::evaluate;
use crate::shared::utils::js::stylex::stylex_define_vars::stylex_define_vars;
use crate::shared::utils::stylex::js_to_expr::{convert_object_to_ast, NestedStringObject};
use crate::shared::utils::validators::{is_define_vars_call, validate_stylex_define_vars};
use crate::shared::{constants, structures::functions::FunctionConfigType};
use crate::shared::{
  enums::TopLevelExpressionKind,
  utils::js::stylex::{stylex_keyframes::get_keyframes_fn, stylex_types::get_types_fn},
};
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_define_vars(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_define_vars = is_define_vars_call(call, &self.state);

    let result = if is_define_vars {
      validate_stylex_define_vars(call, &mut self.state);

      let first_arg = call.args.first();

      let first_arg = first_arg.and_then(|first_arg| match &first_arg.spread {
        Some(_) => todo!(),
        None => Option::Some(first_arg.expr.clone()),
      })?;

      // let mut resolved_namespaces: IndexMap<String, FlatCompiledStyles> = IndexMap::new();

      // let injected_keyframes: IndexMap<String, InjectableStyle> = IndexMap::new();

      let mut identifiers: HashMap<Box<Id>, Box<FunctionConfigType>> = HashMap::new();
      let mut member_expressions: HashMap<
        Box<ImportSources>,
        Box<HashMap<Box<Id>, Box<FunctionConfigType>>>,
      > = HashMap::new();

      let keyframes_fn = get_keyframes_fn();
      let types_fn = get_types_fn();

      for name in &self.state.stylex_keyframes_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );
      }

      for name in &self.state.stylex_types_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(types_fn.clone())),
        );
      }

      for name in &self.state.stylex_import {
        let member_expression = member_expressions.entry(name.clone()).or_default();

        member_expression.insert(
          Box::new(Ident::new("keyframes".into(), DUMMY_SP).to_id()),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );

        let identifier = identifiers
          .entry(Box::new(
            Ident::new(name.get_import_str().into(), DUMMY_SP).to_id(),
          ))
          .or_insert(Box::new(FunctionConfigType::Map(HashMap::default())));

        if let Some(identifier_map) = identifier.as_map_mut() {
          identifier_map.insert(
            Ident::new("types".into(), DUMMY_SP).to_id(),
            types_fn.clone(),
          );
        }
      }

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
      });

      let evaluated_arg = evaluate(&first_arg, &mut self.state, &function_map);

      // dbg!(evaluated_arg.clone());

      assert!(
        evaluated_arg.confident,
        "{}",
        constants::messages::NON_STATIC_VALUE
      );

      let value = match evaluated_arg.value {
        Some(value) => {
          assert!(
            value
              .as_expr()
              .map(|expr| expr.is_object())
              .unwrap_or(false),
            "{}",
            constants::messages::NON_OBJECT_FOR_STYLEX_CALL
          );
          value
        }
        None => {
          panic!("{}", constants::messages::NON_STATIC_VALUE)
        }
      };
      // dbg!(&evaluated_arg.confident, &value);

      let Some(file_name) = self.state.get_filename_for_hashing() else {
        panic!("No filename found for generating theme name.")
      };
    // println!("!!!!!file_name: {}",&file_name);

      // todo!();

      let export_expr = self
        .state
        .get_top_level_expr(&TopLevelExpressionKind::NamedExport, call);

      let export_name = export_expr
        .and_then(|expr| expr.2)
        .map(|decl| decl.0.to_string())
        .expect("Export variable not found");

      self.state.theme_name = Option::Some(gen_file_based_identifier(
        &file_name,
        &export_name,
        Option::None,
      ));

      // dbg!(&self.state.theme_name);

      let (variables_obj, injected_styles_sans_keyframes) =
        stylex_define_vars(&value, &mut self.state);

      // dbg!(&variables_obj, &injected_styles_sans_keyframes);

      let mut injected_styles = self.state.injected_keyframes.clone();
      injected_styles.extend(injected_styles_sans_keyframes);

      // dbg!(&variables_obj);

      let (var_name, _) = self.get_call_var_name(call);

      let result_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(variables_obj));

      self
        .state
        .register_styles(call, &injected_styles, &result_ast, &var_name);

      return Option::Some(result_ast);
    } else {
      Option::None
    };

    result
  }
}
