//__stylex_metadata_start__[{"class_name":"x1e2nbdu","style":{"rtl":null,"ltr":".x1e2nbdu{color:red}"},"priority":3000},{"class_name":"x1t391ir","style":{"rtl":null,"ltr":".x1t391ir{background-color:blue}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
_inject2(".x1t391ir{background-color:blue}", 3000);
const styles = {
    foo: {
        color: "x1e2nbdu",
        $$css: true
    }
};
function MyComponent() {
    return <>
            <div className={"x1e2nbdu"}/>
            <div className={"x1t391ir"}/>
            <CustomComponent xstyle={styles.foo}/>
            <div className={"x1e2nbdu x1t391ir"}/>
          </>;
}
export { MyComponent };
