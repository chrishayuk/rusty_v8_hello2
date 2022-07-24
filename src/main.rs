use std::env;
use rusty_v8 as v8;

fn main() {
    // accept some arguments   
    let args: Vec<String> = env::args().collect();
    let javascript = &args[1];
    
    // accept argument 2 as boolean if exists
    let print_result = if args.len() > 2 {
        args[2].parse::<bool>().unwrap()
    } else {
        false
    };

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    // collect the code from the args
    let code = v8::String::new(scope, &javascript).unwrap();
    // compile the javascript source code
    let script = v8::Script::compile(scope, code, None).unwrap();

    // run the javascript and get the result
    let result = script.run(scope).unwrap();

    // convert the javascript to a string
    let _result = result.to_string(scope).unwrap();

    // print the result
    if print_result {
        println!("{}", result.to_rust_string_lossy(scope));
    }

}
