pub struct Vmb {
    pub stack: Vec<u8>,
    instructions: Vec<u8>
}

pub enum VmbInstruction {
    Push(u8),
    Pop,
    Print,
}

impl Vmb {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            instructions: Vec::new()
        }
    }

    pub fn add_instruction(&mut self, instruction: VmbInstruction) {
        match instruction {
            VmbInstruction::Push(x) => self.instructions.extend([b'1', x].iter()),
            VmbInstruction::Pop => self.instructions.push(b'2'),
            VmbInstruction::Print => self.instructions.push(b'3')
        }
    }

    pub fn add_instructions(&mut self, instructions: Vec<VmbInstruction>) {
        for instruction in instructions {
            self.add_instruction(instruction)
        }
    }

    pub fn add_file(&mut self, filename: &str) -> std::io::Result<()> {
        self.stack.extend(std::fs::read(filename)?);
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let mut push_done = false;

        for i in 0..self.instructions.len() {
            let instruction = self.instructions[i];

            match instruction {
                b'1' => {
                    push_done = true;
                    match self.instructions.get(i + 1) {
                        Some(x) => self.stack.push(*x),
                        None => {
                            return Err("Push instruction must have a value: u8 to push".to_string());
                        }
                    }
                }

                b'2' => match self.stack.pop() {
                    Some(_) => {}
                    None => return Err("Pop instruction, stack must have at least one value pushed in it".to_string())
                }
                
                b'3' => match self.stack.get(0) {
                    Some(x) => println!("{}", x),
                    None => return Err("Print instruction, stack must have at least one value pushed in it".to_string()),
                },

                x => {
                    if push_done {
                        push_done = false;
                    } else {
                        return Err(format!("Invalid instruction '{}'", x))
                    }
                }
            }
        }

        Ok(())
    }
}

impl From<Vec<u8>> for Vmb {
    fn from(instructions: Vec<u8>) -> Self {
        Self {
            stack: Vec::new(),
            instructions,
        }
    }
}

impl From<u8> for Vmb {
    fn from(instruction: u8) -> Self {
        Self {
            stack: Vec::new(),
            instructions: vec![instruction],
        }
    }
}
