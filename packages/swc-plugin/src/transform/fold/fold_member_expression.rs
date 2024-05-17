use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{Expr, Id, MemberExpr, MemberProp},
    visit::FoldWith,
  },
};

use crate::{
  shared::{
    enums::{ModuleCycle, NonNullProp, NonNullProps, StyleVarsToKeep},
    utils::common::{increase_ident_count, increase_member_ident_count},
  },
  ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_member_expr_impl(&mut self, member_expression: MemberExpr) -> MemberExpr {
    if self.cycle == ModuleCycle::Skip {
      return member_expression;
    }

    if self.cycle == ModuleCycle::Initializing {
      if let Some(obj_ident) = member_expression.obj.as_ident() {
        increase_member_ident_count(&mut self.state, &obj_ident.to_id());
      }

      return member_expression.fold_children_with(self);
    }

    if self.cycle == ModuleCycle::PreCleaning {
      // dbg!(&self.state.member_object_ident_count_map);

      let object = member_expression.obj.as_ref();
      let property = &member_expression.prop;

      let mut obj_name: Option<Id> = Option::None;
      let mut prop_name: Option<Id> = Option::None;

      if let Expr::Ident(ident) = object {
        let obj_ident_name = ident.sym.to_string();

        obj_name = Option::Some(ident.to_id());

        if self.state.style_map.contains_key(&obj_ident_name) {
          if let MemberProp::Ident(ident) = property {
            prop_name = Option::Some(ident.to_id());
          }
        }
      }

      // dbg!(&self.state.style_map);
      if let Some(obj_name) = obj_name {
        if let Some(prop_name) = prop_name {
          if let Some(count) = self.state.member_object_ident_count_map.get(&obj_name) {
            if self.state.style_map.contains_key(obj_name.0.as_str()) && count > &0 {
              // dbg!(
              //   &self.state.member_object_ident_count_map,
              //   &member_expression
              // );
              // dbg!(
              //   &obj_name,
              //   &prop_name,
              //   &count,
              //   &self.state.var_decl_count_map
              // );

              increase_ident_count(
                &mut self.state,
                object.as_ident().expect("Object not an ident"),
              );

              let style_var_to_keep = StyleVarsToKeep(
                obj_name.clone(),
                NonNullProp::Id(prop_name),
                NonNullProps::True,
              );

              // dbg!(&style_var_to_keep);

              self
                .state
                .style_vars_to_keep
                .insert(Box::new(style_var_to_keep));
            }
          }
        }
      }

      return member_expression;
    }

    member_expression.fold_children_with(self)
  }
}
