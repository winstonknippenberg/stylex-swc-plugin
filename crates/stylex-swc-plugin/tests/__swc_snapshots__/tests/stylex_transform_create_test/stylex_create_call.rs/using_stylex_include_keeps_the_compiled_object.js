//__stylex_metadata_start__[{"class_name":"xju2f9n","style":{"rtl":null,"ltr":".xju2f9n{color:blue}"},"priority":3000},{"class_name":"x1e2nbdu","style":{"rtl":null,"ltr":".x1e2nbdu{color:red}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xju2f9n{color:blue}", 3000);
const importedStyles = {
    foo: {
        color: "xju2f9n",
        $$css: true
    }
};
_inject2(".x1e2nbdu{color:red}", 3000);
const styles = {
    foo: {
        ...importedStyles.foo,
        color: "x1e2nbdu",
        $$css: true
    }
};
stylex.props(styles.foo);
