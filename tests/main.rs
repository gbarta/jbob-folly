#![feature(plugin, custom_attribute)]
#![plugin(jbob)]

#[jbob_expander]
fn expand_this() {
    println!("expand this!");
}

#[test]
fn test_hello_world() {
    expand_this();
}

