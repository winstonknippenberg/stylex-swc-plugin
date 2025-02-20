use std::sync::atomic::{AtomicUsize, Ordering};

use swc_core::ecma::{
  ast::{Expr, KeyValueProp, Prop, PropName, PropOrSpread},
  utils::quote_ident,
};

use crate::shared::{
  constants::messages::ILLEGAL_ARGUMENT_LENGTH, utils::ast::factories::object_expression_factory,
};

static NUMBER: AtomicUsize = AtomicUsize::new(1);

fn uuid() -> String {
  let number = NUMBER.fetch_add(1, Ordering::SeqCst);
  format!("__included_{}__", number)
}

pub(crate) fn stylex_include(args: Vec<Expr>) -> Expr {
  if args.len() > 1 {
    panic!("{}", ILLEGAL_ARGUMENT_LENGTH);
  }

  let first_arg = &args[0];

  let prop = Prop::from(KeyValueProp {
    key: PropName::Ident(quote_ident!(uuid().as_str())),
    value: Box::new(first_arg.clone()),
  });

  object_expression_factory(vec![PropOrSpread::from(prop)])
}
