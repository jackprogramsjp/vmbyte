mod vm;
mod lang;

fn main() -> std::io::Result<()> {
    let mut vmb = vm::Vmb::new();

    vmb.add_instructions(vec![
        vm::VmbInstruction::Push(23_u8),
        vm::VmbInstruction::Push(52_u8),
        vm::VmbInstruction::Pop,
        vm::VmbInstruction::Print,
    ]);
    
    match vmb.execute() {
        Err(e) => eprintln!("{}", e),
        _ => {}
    };

    Ok(())
}
