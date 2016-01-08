#![feature(plugin, custom_attribute)]
#![plugin(jbob)]

#[jbob_trace_se(FooBar)]
fn expand_this() {
    println!("expand this!");
}

jbob_trace_macro!(fn expand_this_as_well(){
    println!("expand this as well!");
});

#[test]
fn test_hello_world() {
    expand_this();
    expand_this_as_well();
}

