use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::ecma::ast::VarDeclarator;
use swc_core::{common::comments::Comments, ecma::ast::Expr};

use crate::shared::structures::functions::FunctionConfigType;
use crate::shared::utils::{
  ast::convertors::string_to_expression,
  validators::{assert_valid_keyframes, is_keyframes_call, validate_stylex_keyframes_indent},
};
use crate::shared::{
  constants::messages::{NON_OBJECT_FOR_STYLEX_CALL, NON_STATIC_VALUE},
  transformers::stylex_first_that_works::stylex_first_that_works,
};
use crate::shared::{
  structures::{
    functions::{FunctionConfig, FunctionMap, FunctionType},
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  transformers::stylex_include::stylex_include,
};
use crate::shared::{
  transformers::stylex_keyframes::stylex_keyframes, utils::js::evaluate::evaluate,
};
use crate::ModuleTransformVisitor;

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_keyframes_call(
    &mut self,
    var_decl: &VarDeclarator,
  ) -> Option<Expr> {
    let is_keyframes_call = is_keyframes_call(var_decl, &self.state);

    let result = if is_keyframes_call {
      validate_stylex_keyframes_indent(var_decl, &mut self.state);

      let call = &var_decl
        .init
        .clone()
        .and_then(|decl| decl.call())
        .expect("Expected call expression");

      let first_arg = call.args.first();

      let first_arg = first_arg.map(|first_arg| match &first_arg.spread {
        Some(_) => unimplemented!("Spread"),
        None => first_arg.expr.clone(),
      })?;

      let mut identifiers: FunctionMapIdentifiers = HashMap::new();
      let mut member_expressions: FunctionMapMemberExpression = HashMap::new();

      let include_fn = FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(stylex_include),
        takes_path: true,
      };

      let first_that_works_fn = FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
        takes_path: false,
      };

      for name in &self.state.stylex_include_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(include_fn.clone())),
        );
      }

      for name in &self.state.stylex_first_that_works_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );
      }

      for name in &self.state.stylex_import {
        member_expressions.entry(name.clone()).or_default();

        let member_expression = member_expressions.get_mut(name).unwrap();

        member_expression.insert(
          "include".into(),
          Box::new(FunctionConfigType::Regular(include_fn.clone())),
        );

        member_expression.insert(
          "firstThatWorks".into(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );
      }

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
      });

      let evaluated_arg = evaluate(&first_arg, &mut self.state, &function_map);

      assert!(evaluated_arg.confident, "{}", NON_STATIC_VALUE);

      let value = match evaluated_arg.value {
        Some(value) => {
          assert!(
            value
              .as_expr()
              .map(|expr| expr.is_object())
              .unwrap_or(false),
            "{}",
            NON_OBJECT_FOR_STYLEX_CALL
          );
          value
        }
        None => {
          panic!("{}", NON_STATIC_VALUE)
        }
      };

      let plain_object = value;

      assert_valid_keyframes(&plain_object);

      let (animation_name, injectable_style) = stylex_keyframes(&plain_object, &mut self.state);

      let (var_name, _) = &self.get_call_var_name(call);

      let mut injected_styles = IndexMap::new();

      injected_styles.insert(animation_name.clone(), Box::new(injectable_style));

      let result_ast = string_to_expression(animation_name.as_str());

      self
        .state
        .register_styles(call, &injected_styles, &result_ast, var_name);

      Some(result_ast)
    } else {
      None
    };

    result
  }
}
