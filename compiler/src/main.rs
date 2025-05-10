use inkwell::{context::Context, memory_buffer::MemoryBuffer, module::Module, OptimizationLevel};

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

    let foo = module.get_function("foo").unwrap();
    builder.build_call(foo, &[], "").unwrap();

    let result = context.i32_type().const_int(0, false);
    builder.build_return(Some(&result)).unwrap();

    module.verify().unwrap();

    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe { engine.run_function_as_main(main_fun, &[]) };
}
