mod ast;
mod lexer;
mod parser;
mod token;
mod op;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn parse() -> f64 {
    let s = unsafe { std::ffi::CStr::from_ptr(1 as _).to_str().unwrap() };
    parser::parse(&lexer::lex(s)).eval()
}