extern crate alloc;

use wasmi::{Engine, Linker, Module, Store};

/// Loads a WebAssembly module, attaches Utah-Kernel host functions, then runs exported `_start`.
pub fn run_web_assembly_program(web_assembly_code: &[u8]) {
    let execution_engine = Engine::default();

    let verified_module = Module::new(&execution_engine, web_assembly_code)
        .expect("Failed to verify the WebAssembly program.");

    let mut program_memory_store = Store::new(&execution_engine, ());

    let mut system_linker = <Linker<()>>::new(&execution_engine);

    // Attach system calls so the guest can ask the kernel to use the screen safely.
    crate::system_calls::register_system_calls(&mut system_linker, &execution_engine);

    let running_instance = system_linker
        .instantiate(&mut program_memory_store, &verified_module)
        .expect("Failed to assemble the program sandbox.")
        .start(&mut program_memory_store)
        .expect("Failed to start the WebAssembly program.");

    let start_function = running_instance
        .get_typed_func::<(), ()>(&program_memory_store, "_start")
        .expect("Could not find the starting point in the WebAssembly program.");

    start_function
        .call(&mut program_memory_store, ())
        .expect("The WebAssembly program crashed while running.");
}
