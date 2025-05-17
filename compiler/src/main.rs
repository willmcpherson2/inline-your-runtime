use inkwell::{
    context::Context,
    memory_buffer::MemoryBuffer,
    module::Module,
    passes::PassBuilderOptions,
    targets::{FileType, InitializationConfig, Target, TargetMachine, TargetMachineOptions},
    OptimizationLevel,
};
use std::{env::args, path::Path, process::exit};

const RTS_BC: &[u8] = include_bytes!("../../target/rts.bc");

fn main() {
    Target::initialize_all(&InitializationConfig::default());
    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).unwrap();
    let options = TargetMachineOptions::new().set_level(OptimizationLevel::None);
    let machine = target
        .create_target_machine_from_options(&triple, options)
        .unwrap();

    let context = Context::create();
    let buffer = MemoryBuffer::create_from_memory_range(RTS_BC, "rts");
    let module = Module::parse_bitcode_from_buffer(&buffer, &context).unwrap();
    let builder = context.create_builder();

    module
        .run_passes("internalize", &machine, PassBuilderOptions::create())
        .unwrap();

    let main_fun_type = context.i32_type().fn_type(&[], false);
    let main_fun = module.add_function("main", main_fun_type, None);

    let block = context.append_basic_block(main_fun, "start");
    builder.position_at_end(block);

    let new_foo = module.get_function("new_foo").unwrap();
    let foo = builder
        .build_call(new_foo, &[], "foo")
        .unwrap()
        .try_as_basic_value()
        .unwrap_left()
        .into_pointer_value();

    let sum_foo = module.get_function("sum_foo").unwrap();
    let result = builder
        .build_call(sum_foo, &[foo.into()], "result")
        .unwrap()
        .try_as_basic_value()
        .unwrap_left()
        .into_int_value();

    let free_foo = module.get_function("free_foo").unwrap();
    builder.build_call(free_foo, &[foo.into()], "").unwrap();

    builder.build_return(Some(&result)).unwrap();

    module
        .run_passes("default<O3>", &machine, PassBuilderOptions::create())
        .unwrap();

    module.verify().unwrap();

    let eval = args().any(|arg| arg == "-e");
    if eval {
        let engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        let code = unsafe { engine.run_function_as_main(main_fun, &[]) };
        exit(code)
    } else {
        machine
            .write_to_file(&module, FileType::Object, Path::new("main.o"))
            .unwrap();
    }
}
