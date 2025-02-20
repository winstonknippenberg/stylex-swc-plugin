#[cfg(test)]
mod ensure_css_values_are_split_correctly {
  use crate::shared::utils::css::common::split_value;

  #[test]
  fn simple_space_separated_numbers() {
    assert_eq!(
      split_value(Some("0 1 2 3")),
      (
        "0".into(),
        Some("1".into()),
        Some("2".into()),
        Some("3".into())
      )
    );
  }

  #[test]
  fn simple_space_separated_lengths() {
    assert_eq!(
      split_value(Some("0px 1rem 2% 3em")),
      (
        "0px".into(),
        Some("1rem".into()),
        Some("2%".into()),
        Some("3em".into())
      )
    );
  }

  #[test]
  fn simple_comma_separated_numbers() {
    assert_eq!(
      split_value(Some("0, 1, 2, 3")),
      (
        "0".into(),
        Some("1".into()),
        Some("2".into()),
        Some("3".into())
      )
    );
  }

  #[test]
  fn simple_comma_separated_lengths() {
    assert_eq!(
      split_value(Some("0px, 1rem, 2%, 3em")),
      (
        "0px".into(),
        Some("1rem".into()),
        Some("2%".into()),
        Some("3em".into())
      )
    );
  }

  #[test]
  fn does_not_lists_within_functions() {
    assert_eq!(
      split_value(Some("rgb(255 200 0)")),
      ("rgb(255 200 0)".into(), None, None, None)
    );

    assert_eq!(
      split_value(Some("rgb(255 200 / 0.5)")),
      ("rgb(255 200/0.5)".into(), None, None, None)
    );
  }

  #[test]
  fn does_not_lists_within_calc() {
    assert_eq!(
      split_value(Some("calc((100% - 50px) * 0.5)")),
      ("calc((100% - 50px) * 0.5)".into(), None, None, None)
    );

    assert_eq!(
      split_value(Some("calc((100% - 50px) * 0.5) var(--rightpadding, 20px)")),
      (
        "calc((100% - 50px) * 0.5)".into(),
        Some("var(--rightpadding,20px)".into()),
        None,
        None
      )
    );
  }
}
