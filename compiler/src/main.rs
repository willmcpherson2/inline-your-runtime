use inkwell::{
    context::Context,
    memory_buffer::MemoryBuffer,
    module::Module,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    values::FunctionValue,
    OptimizationLevel,
};
use std::{env::args, path::Path, process::exit};

const RTS_BC: &[u8] = include_bytes!("../../target/rts.bc");

fn main() {
    let context = Context::create();
    let buffer = MemoryBuffer::create_from_memory_range(RTS_BC, "rts");
    let module = Module::parse_bitcode_from_buffer(&buffer, &context).unwrap();
    let builder = context.create_builder();

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

    module.verify().unwrap();

    let eval = args().any(|arg| arg == "-e");

    if eval {
        jit(&module, main_fun)
    } else {
        compile(&module)
    }
}

fn jit(module: &Module, main_fun: FunctionValue) -> ! {
    let engine = module
        .create_jit_execution_engine(OptimizationLevel::Default)
        .unwrap();
    let code = unsafe { engine.run_function_as_main(main_fun, &[]) };
    exit(code)
}

fn compile(module: &Module) {
    Target::initialize_all(&InitializationConfig::default());
    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();
    target_machine
        .write_to_file(module, FileType::Object, Path::new("main.o"))
        .unwrap();
}
