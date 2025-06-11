use rue_ast::{CstRoot, ExpressionNode, FunctionNode, StatementNode};
use rue_semantic::Scope;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CodegenError {
    pub message: String,
}

// x86-64 instruction representation
#[derive(Debug, Clone)]
pub enum Instruction {
    // Stack operations
    Push(Operand),
    Pop(Operand),

    // Arithmetic
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand),
    Div(Operand),

    // Comparison
    Cmp(Operand, Operand),

    // Control flow
    Jmp(String),  // Unconditional jump
    Je(String),   // Jump if equal
    Jne(String),  // Jump if not equal
    Jle(String),  // Jump if less or equal
    Jg(String),   // Jump if greater
    Call(String), // Function call
    Ret,          // Return

    // System
    Mov(Operand, Operand), // Move data
    Syscall,               // System call

    // Labels (not real instructions, but markers)
    Label(String),
}

#[derive(Debug, Clone)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
    Memory(String), // Variable name or memory reference
    Label(String),  // For jumps and calls
}

#[derive(Debug, Clone)]
pub enum Register {
    Rax, // Accumulator, return value
    Rbx, // Base
    Rcx, // Counter
    Rdx, // Data
    Rsp, // Stack pointer
    Rbp, // Base pointer
    Rsi, // Source index
    Rdi, // Destination index
}

// Code generator state
pub struct Codegen {
    instructions: Vec<Instruction>,
    label_counter: usize,
    stack_offset: i64,
    variables: HashMap<String, i64>, // Variable -> stack offset
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            label_counter: 0,
            stack_offset: 0,
            variables: HashMap::new(),
        }
    }

    // Generate a unique label
    fn next_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    // Emit an instruction
    fn emit(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    // Generate code for the entire program
    pub fn generate(
        &mut self,
        ast: &CstRoot,
        scope: &Scope,
    ) -> Result<Vec<Instruction>, CodegenError> {
        // Generate program prologue
        self.emit_prologue();

        // Find and generate main function first
        let mut main_generated = false;
        for item in &ast.items {
            if let rue_ast::CstNode::Function(func) = item {
                if let rue_lexer::TokenKind::Ident(name) = &func.name.kind {
                    if name == "main" {
                        self.generate_function(func, scope)?;
                        main_generated = true;
                        break;
                    }
                }
            }
        }

        if !main_generated {
            return Err(CodegenError {
                message: "No main function found".to_string(),
            });
        }

        // Generate other functions
        for item in &ast.items {
            if let rue_ast::CstNode::Function(func) = item {
                if let rue_lexer::TokenKind::Ident(name) = &func.name.kind {
                    if name != "main" {
                        self.generate_function(func, scope)?;
                    }
                }
            }
        }

        self.emit_epilogue();

        Ok(self.instructions.clone())
    }

    // Generate program entry point
    fn emit_prologue(&mut self) {
        // Entry point label
        self.emit(Instruction::Label("_start".to_string()));

        // Call main function
        self.emit(Instruction::Call("main".to_string()));

        // Exit program with main's return value (in rax)
        self.emit(Instruction::Mov(
            Operand::Register(Register::Rdi),
            Operand::Register(Register::Rax),
        )); // exit code
        self.emit(Instruction::Mov(
            Operand::Register(Register::Rax),
            Operand::Immediate(60),
        )); // sys_exit
        self.emit(Instruction::Syscall);
    }

    fn emit_epilogue(&mut self) {
        // Additional functions would go here
    }

    // Generate code for a function
    fn generate_function(
        &mut self,
        func: &FunctionNode,
        scope: &Scope,
    ) -> Result<(), CodegenError> {
        // Function label
        if let rue_lexer::TokenKind::Ident(name) = &func.name.kind {
            self.emit(Instruction::Label(name.clone()));
        }

        // Function prologue
        self.emit(Instruction::Push(Operand::Register(Register::Rbp)));
        self.emit(Instruction::Mov(
            Operand::Register(Register::Rbp),
            Operand::Register(Register::Rsp),
        ));

        // Save parameter if exists
        if let Some(param) = func.param_list.params.first() {
            if let rue_lexer::TokenKind::Ident(param_name) = &param.kind {
                self.stack_offset -= 8; // Allocate space for parameter
                self.variables.insert(param_name.clone(), self.stack_offset);
                // Move parameter from rdi to stack
                self.emit(Instruction::Mov(
                    Operand::Memory(format!("rbp{:+}", self.stack_offset)),
                    Operand::Register(Register::Rdi),
                ));
            }
        }

        // Generate function body
        let mut return_value = None;
        for stmt in &func.body.statements {
            return_value = self.generate_statement(stmt, scope)?;
        }

        // Ensure return value is in rax
        if return_value.is_none() {
            self.emit(Instruction::Mov(
                Operand::Register(Register::Rax),
                Operand::Immediate(0),
            ));
        }

        // Function epilogue
        self.emit(Instruction::Mov(
            Operand::Register(Register::Rsp),
            Operand::Register(Register::Rbp),
        ));
        self.emit(Instruction::Pop(Operand::Register(Register::Rbp)));
        self.emit(Instruction::Ret);

        // Reset state for next function
        self.stack_offset = 0;
        self.variables.clear();

        Ok(())
    }

    // Generate code for a statement, returns true if it's an expression that produces a value
    fn generate_statement(
        &mut self,
        stmt: &StatementNode,
        scope: &Scope,
    ) -> Result<Option<()>, CodegenError> {
        match stmt {
            StatementNode::Expression(expr) => {
                self.generate_expression(expr, scope)?;
                // Expression result is now in rax (return value)
                Ok(Some(()))
            }
            StatementNode::Let(let_stmt) => {
                // Generate the value expression
                self.generate_expression(&let_stmt.value, scope)?;

                // Store in variable
                if let rue_lexer::TokenKind::Ident(var_name) = &let_stmt.name.kind {
                    self.stack_offset -= 8; // Allocate space
                    self.variables.insert(var_name.clone(), self.stack_offset);
                    self.emit(Instruction::Mov(
                        Operand::Memory(format!("rbp{:+}", self.stack_offset)),
                        Operand::Register(Register::Rax),
                    ));
                }
                Ok(None)
            }
            StatementNode::If(if_stmt) => {
                let else_label = self.next_label("else");
                let end_label = self.next_label("end_if");

                // Generate condition
                self.generate_expression(&if_stmt.condition, scope)?;

                // Compare with 0 (false)
                self.emit(Instruction::Cmp(
                    Operand::Register(Register::Rax),
                    Operand::Immediate(0),
                ));
                self.emit(Instruction::Je(else_label.clone()));

                // Generate then block
                let mut then_return = None;
                for stmt in &if_stmt.then_block.statements {
                    then_return = self.generate_statement(stmt, scope)?;
                }

                self.emit(Instruction::Jmp(end_label.clone()));

                // Generate else block if it exists
                self.emit(Instruction::Label(else_label));
                let mut else_return = None;
                if let Some(else_clause) = &if_stmt.else_clause {
                    match &else_clause.body {
                        rue_ast::ElseBodyNode::Block(block) => {
                            for stmt in &block.statements {
                                else_return = self.generate_statement(stmt, scope)?;
                            }
                        }
                        rue_ast::ElseBodyNode::If(nested_if) => {
                            else_return = self
                                .generate_statement(&StatementNode::If(nested_if.clone()), scope)?;
                        }
                    }
                }

                self.emit(Instruction::Label(end_label));

                // If both branches return values, this is an expression
                if then_return.is_some() && else_return.is_some() {
                    Ok(Some(()))
                } else {
                    Ok(None)
                }
            }
            StatementNode::While(while_stmt) => {
                let loop_start = self.next_label("loop_start");
                let loop_end = self.next_label("loop_end");

                // Loop start label
                self.emit(Instruction::Label(loop_start.clone()));

                // Generate condition
                self.generate_expression(&while_stmt.condition, scope)?;

                // Compare with 0 (false) and jump to end if condition is false
                self.emit(Instruction::Cmp(
                    Operand::Register(Register::Rax),
                    Operand::Immediate(0),
                ));
                self.emit(Instruction::Je(loop_end.clone()));

                // Generate loop body
                for stmt in &while_stmt.body.statements {
                    self.generate_statement(stmt, scope)?;
                }

                // Jump back to condition check
                self.emit(Instruction::Jmp(loop_start));

                // Loop end label
                self.emit(Instruction::Label(loop_end));

                Ok(None) // While loops don't return values
            }
        }
    }

    // Generate code for an expression, result goes to rax
    fn generate_expression(
        &mut self,
        expr: &ExpressionNode,
        _scope: &Scope,
    ) -> Result<(), CodegenError> {
        match expr {
            ExpressionNode::Literal(token) => {
                if let rue_lexer::TokenKind::Integer(value) = &token.kind {
                    self.emit(Instruction::Mov(
                        Operand::Register(Register::Rax),
                        Operand::Immediate(*value),
                    ));
                }
                Ok(())
            }
            ExpressionNode::Identifier(token) => {
                if let rue_lexer::TokenKind::Ident(name) = &token.kind {
                    if let Some(&offset) = self.variables.get(name) {
                        self.emit(Instruction::Mov(
                            Operand::Register(Register::Rax),
                            Operand::Memory(format!("rbp{:+}", offset)),
                        ));
                    } else {
                        return Err(CodegenError {
                            message: format!("Undefined variable: {}", name),
                        });
                    }
                }
                Ok(())
            }
            ExpressionNode::Binary(binary_expr) => {
                // Generate left operand
                self.generate_expression(&binary_expr.left, _scope)?;
                self.emit(Instruction::Push(Operand::Register(Register::Rax))); // Save left value

                // Generate right operand
                self.generate_expression(&binary_expr.right, _scope)?;

                // Pop left value to rbx
                self.emit(Instruction::Pop(Operand::Register(Register::Rbx)));

                // Perform operation (rbx op rax -> rax)
                match &binary_expr.operator.kind {
                    rue_lexer::TokenKind::Plus => {
                        self.emit(Instruction::Add(
                            Operand::Register(Register::Rax),
                            Operand::Register(Register::Rbx),
                        ));
                    }
                    rue_lexer::TokenKind::Minus => {
                        // rbx - rax -> rax, so we need rax = rbx - rax
                        self.emit(Instruction::Sub(
                            Operand::Register(Register::Rbx),
                            Operand::Register(Register::Rax),
                        ));
                        self.emit(Instruction::Mov(
                            Operand::Register(Register::Rax),
                            Operand::Register(Register::Rbx),
                        ));
                    }
                    rue_lexer::TokenKind::Star => {
                        self.emit(Instruction::Mul(Operand::Register(Register::Rbx)));
                        // Result is in rax
                    }
                    rue_lexer::TokenKind::Slash => {
                        // Move dividend to rax, divisor to rbx
                        self.emit(Instruction::Mov(
                            Operand::Register(Register::Rax),
                            Operand::Register(Register::Rbx),
                        ));
                        self.emit(Instruction::Mov(
                            Operand::Register(Register::Rbx),
                            Operand::Register(Register::Rax),
                        ));
                        self.emit(Instruction::Div(Operand::Register(Register::Rbx)));
                    }
                    rue_lexer::TokenKind::LessEqual => {
                        // rbx <= rax ? 1 : 0
                        self.emit(Instruction::Cmp(
                            Operand::Register(Register::Rbx),
                            Operand::Register(Register::Rax),
                        ));
                        let true_label = self.next_label("le_true");
                        let end_label = self.next_label("le_end");

                        self.emit(Instruction::Jle(true_label.clone()));
                        // False case
                        self.emit(Instruction::Mov(
                            Operand::Register(Register::Rax),
                            Operand::Immediate(0),
                        ));
                        self.emit(Instruction::Jmp(end_label.clone()));
                        // True case
                        self.emit(Instruction::Label(true_label));
                        self.emit(Instruction::Mov(
                            Operand::Register(Register::Rax),
                            Operand::Immediate(1),
                        ));
                        self.emit(Instruction::Label(end_label));
                    }
                    rue_lexer::TokenKind::Greater => {
                        // rbx > rax ? 1 : 0
                        self.emit(Instruction::Cmp(
                            Operand::Register(Register::Rbx),
                            Operand::Register(Register::Rax),
                        ));
                        let true_label = self.next_label("gt_true");
                        let end_label = self.next_label("gt_end");

                        // Need to add Jg instruction
                        self.emit(Instruction::Jg(true_label.clone()));
                        // False case
                        self.emit(Instruction::Mov(
                            Operand::Register(Register::Rax),
                            Operand::Immediate(0),
                        ));
                        self.emit(Instruction::Jmp(end_label.clone()));
                        // True case
                        self.emit(Instruction::Label(true_label));
                        self.emit(Instruction::Mov(
                            Operand::Register(Register::Rax),
                            Operand::Immediate(1),
                        ));
                        self.emit(Instruction::Label(end_label));
                    }
                    _ => {
                        return Err(CodegenError {
                            message: format!(
                                "Unsupported operator: {:?}",
                                binary_expr.operator.kind
                            ),
                        });
                    }
                }
                Ok(())
            }
            ExpressionNode::Call(call_expr) => {
                // Generate arguments (right to left for x86-64 calling convention)
                if !call_expr.args.is_empty() {
                    self.generate_expression(&call_expr.args[0], _scope)?;
                    self.emit(Instruction::Mov(
                        Operand::Register(Register::Rdi),
                        Operand::Register(Register::Rax),
                    ));
                }

                // Call function
                if let ExpressionNode::Identifier(func_token) = &*call_expr.function {
                    if let rue_lexer::TokenKind::Ident(func_name) = &func_token.kind {
                        self.emit(Instruction::Call(func_name.clone()));
                    }
                }

                Ok(())
            }
        }
    }
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

// ELF executable generator
pub struct Assembler {
    code: Vec<u8>,
    symbol_table: HashMap<String, u64>,
    relocations: Vec<Relocation>,
}

#[derive(Debug)]
struct Relocation {
    offset: u64,
    symbol: String,
    #[allow(dead_code)]
    rel_type: RelocationType,
}

#[derive(Debug)]
enum RelocationType {
    Rel32, // 32-bit relative call/jump
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            symbol_table: HashMap::new(),
            relocations: Vec::new(),
        }
    }

    // Convert instructions to machine code
    pub fn assemble(&mut self, instructions: Vec<Instruction>) -> Result<Vec<u8>, CodegenError> {
        // First pass: calculate symbol addresses
        let mut address = 0u64;
        for instr in &instructions {
            match instr {
                Instruction::Label(name) => {
                    self.symbol_table.insert(name.clone(), address);
                }
                _ => {
                    address += self.instruction_size(instr);
                }
            }
        }

        // Second pass: emit machine code
        for instr in &instructions {
            self.emit_instruction(instr)?;
        }

        // Resolve relocations
        self.resolve_relocations()?;

        Ok(self.code.clone())
    }

    fn instruction_size(&self, instr: &Instruction) -> u64 {
        match instr {
            Instruction::Push(_) => 1,   // push reg = 50+r (1 byte)
            Instruction::Pop(_) => 1,    // pop reg = 58+r (1 byte)
            Instruction::Add(_, _) => 3, // add reg, reg = 48 01 ModR/M
            Instruction::Sub(_, _) => 3, // sub reg, reg = 48 29 ModR/M
            Instruction::Mul(_) => 4,    // imul reg = 48 0f af ModR/M
            Instruction::Div(_) => 5,    // cqo + idiv reg = 48 99 + 48 f7 ModR/M
            Instruction::Cmp(op1, op2) => {
                match (op1, op2) {
                    (Operand::Register(_), Operand::Register(_)) => 3, // cmp reg, reg = 48 39 ModR/M
                    (Operand::Register(_), Operand::Immediate(_)) => 4, // cmp reg, imm8 = 48 83 /7 ib
                    _ => 3,                                             // default
                }
            }
            Instruction::Jmp(_) => 5,  // jmp rel32 = e9 imm32
            Instruction::Je(_) => 6,   // je rel32 = 0f 84 imm32
            Instruction::Jne(_) => 6,  // jne rel32 = 0f 85 imm32
            Instruction::Jle(_) => 6,  // jle rel32 = 0f 8e imm32
            Instruction::Jg(_) => 6,   // jg rel32 = 0f 8f imm32
            Instruction::Call(_) => 5, // call rel32 = e8 imm32
            Instruction::Ret => 1,     // ret = c3
            Instruction::Mov(dst, src) => {
                match (dst, src) {
                    (Operand::Register(_), Operand::Immediate(_)) => 10, // mov reg, imm64 = 48 b8+r imm64
                    (Operand::Register(_), Operand::Register(_)) => 3, // mov dst, src = 48 89 ModR/M
                    (Operand::Memory(_), Operand::Register(_)) => 4, // mov [rbp+offset], reg = 48 89 45 offset
                    (Operand::Register(_), Operand::Memory(_)) => 4, // mov reg, [rbp+offset] = 48 8b 45 offset
                    _ => 10,                                         // default to worst case
                }
            }
            Instruction::Syscall => 2,  // syscall = 0f 05
            Instruction::Label(_) => 0, // Labels don't emit code
        }
    }

    fn emit_instruction(&mut self, instr: &Instruction) -> Result<(), CodegenError> {
        match instr {
            Instruction::Label(_) => {
                // Labels don't emit code
            }
            Instruction::Push(operand) => match operand {
                Operand::Register(reg) => {
                    self.code.push(0x50 + self.register_code(reg));
                }
                _ => {
                    return Err(CodegenError {
                        message: "Push only supports registers".to_string(),
                    });
                }
            },
            Instruction::Pop(operand) => match operand {
                Operand::Register(reg) => {
                    self.code.push(0x58 + self.register_code(reg));
                }
                _ => {
                    return Err(CodegenError {
                        message: "Pop only supports registers".to_string(),
                    });
                }
            },
            Instruction::Mov(dst, src) => {
                match (dst, src) {
                    (Operand::Register(dst_reg), Operand::Immediate(imm)) => {
                        // mov reg, imm64 = 48 b8+r imm64
                        self.code.push(0x48); // REX.W prefix
                        self.code.push(0xb8 + self.register_code(dst_reg));
                        self.code.extend_from_slice(&imm.to_le_bytes());
                    }
                    (Operand::Register(dst_reg), Operand::Register(src_reg)) => {
                        // mov dst, src = 48 89 ModR/M
                        self.code.push(0x48); // REX.W prefix  
                        self.code.push(0x89);
                        self.code.push(
                            0xc0 | (self.register_code(src_reg) << 3) | self.register_code(dst_reg),
                        );
                    }
                    (Operand::Memory(mem), Operand::Register(src_reg)) => {
                        // mov [rbp+offset], reg = 48 89 45 offset
                        if mem.starts_with("rbp") {
                            let offset = self.parse_offset(mem)?;
                            self.code.push(0x48); // REX.W prefix
                            self.code.push(0x89);
                            if (-128..=127).contains(&offset) {
                                self.code.push(0x45 | (self.register_code(src_reg) << 3));
                                self.code.push(offset as u8);
                            } else {
                                return Err(CodegenError {
                                    message: "Large stack offsets not supported".to_string(),
                                });
                            }
                        }
                    }
                    (Operand::Register(dst_reg), Operand::Memory(mem)) => {
                        // mov reg, [rbp+offset] = 48 8b 45 offset
                        if mem.starts_with("rbp") {
                            let offset = self.parse_offset(mem)?;
                            self.code.push(0x48); // REX.W prefix
                            self.code.push(0x8b);
                            if (-128..=127).contains(&offset) {
                                self.code.push(0x45 | (self.register_code(dst_reg) << 3));
                                self.code.push(offset as u8);
                            } else {
                                return Err(CodegenError {
                                    message: "Large stack offsets not supported".to_string(),
                                });
                            }
                        }
                    }
                    _ => {
                        return Err(CodegenError {
                            message: "Unsupported mov operands".to_string(),
                        });
                    }
                }
            }
            Instruction::Add(dst, src) => {
                match (dst, src) {
                    (Operand::Register(dst_reg), Operand::Register(src_reg)) => {
                        // add dst, src = 48 01 ModR/M
                        self.code.push(0x48); // REX.W prefix
                        self.code.push(0x01);
                        self.code.push(
                            0xc0 | (self.register_code(src_reg) << 3) | self.register_code(dst_reg),
                        );
                    }
                    _ => {
                        return Err(CodegenError {
                            message: "Add only supports register operands".to_string(),
                        });
                    }
                }
            }
            Instruction::Sub(dst, src) => {
                match (dst, src) {
                    (Operand::Register(dst_reg), Operand::Register(src_reg)) => {
                        // sub dst, src = 48 29 ModR/M
                        self.code.push(0x48); // REX.W prefix
                        self.code.push(0x29);
                        self.code.push(
                            0xc0 | (self.register_code(src_reg) << 3) | self.register_code(dst_reg),
                        );
                    }
                    _ => {
                        return Err(CodegenError {
                            message: "Sub only supports register operands".to_string(),
                        });
                    }
                }
            }
            Instruction::Cmp(op1, op2) => {
                match (op1, op2) {
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        // cmp reg1, reg2 = 48 39 ModR/M
                        self.code.push(0x48); // REX.W prefix
                        self.code.push(0x39);
                        self.code.push(
                            0xc0 | (self.register_code(reg2) << 3) | self.register_code(reg1),
                        );
                    }
                    (Operand::Register(reg), Operand::Immediate(imm)) => {
                        // cmp reg, imm8 = 48 83 /7 ib
                        if *imm >= -128 && *imm <= 127 {
                            self.code.push(0x48); // REX.W prefix
                            self.code.push(0x83);
                            self.code.push(0xf8 | self.register_code(reg));
                            self.code.push(*imm as u8);
                        } else {
                            return Err(CodegenError {
                                message: "Large immediate comparisons not supported".to_string(),
                            });
                        }
                    }
                    _ => {
                        return Err(CodegenError {
                            message: "Unsupported cmp operands".to_string(),
                        });
                    }
                }
            }
            Instruction::Jmp(label) => {
                // jmp rel32 = e9 imm32
                self.code.push(0xe9);
                self.add_relocation(label.clone(), RelocationType::Rel32);
                self.code.extend_from_slice(&[0, 0, 0, 0]); // Placeholder for address
            }
            Instruction::Je(label) => {
                // je rel32 = 0f 84 imm32
                self.code.push(0x0f);
                self.code.push(0x84);
                self.add_relocation(label.clone(), RelocationType::Rel32);
                self.code.extend_from_slice(&[0, 0, 0, 0]); // Placeholder for address
            }
            Instruction::Jne(label) => {
                // jne rel32 = 0f 85 imm32
                self.code.push(0x0f);
                self.code.push(0x85);
                self.add_relocation(label.clone(), RelocationType::Rel32);
                self.code.extend_from_slice(&[0, 0, 0, 0]); // Placeholder for address
            }
            Instruction::Jle(label) => {
                // jle rel32 = 0f 8e imm32
                self.code.push(0x0f);
                self.code.push(0x8e);
                self.add_relocation(label.clone(), RelocationType::Rel32);
                self.code.extend_from_slice(&[0, 0, 0, 0]); // Placeholder for address
            }
            Instruction::Jg(label) => {
                // jg rel32 = 0f 8f imm32
                self.code.push(0x0f);
                self.code.push(0x8f);
                self.add_relocation(label.clone(), RelocationType::Rel32);
                self.code.extend_from_slice(&[0, 0, 0, 0]); // Placeholder for address
            }
            Instruction::Call(label) => {
                // call rel32 = e8 imm32
                self.code.push(0xe8);
                self.add_relocation(label.clone(), RelocationType::Rel32);
                self.code.extend_from_slice(&[0, 0, 0, 0]); // Placeholder for address
            }
            Instruction::Ret => {
                self.code.push(0xc3);
            }
            Instruction::Mul(operand) => {
                match operand {
                    Operand::Register(reg) => {
                        // imul rax, reg = 48 0f af ModR/M
                        self.code.push(0x48); // REX.W prefix
                        self.code.push(0x0f);
                        self.code.push(0xaf);
                        self.code.push(0xc0 | self.register_code(reg));
                    }
                    _ => {
                        return Err(CodegenError {
                            message: "Mul only supports register operands".to_string(),
                        });
                    }
                }
            }
            Instruction::Div(operand) => {
                match operand {
                    Operand::Register(reg) => {
                        // Sign extend rax to rdx:rax for division
                        self.code.push(0x48); // REX.W prefix
                        self.code.push(0x99); // cqo instruction
                        // idiv reg = 48 f7 /7 ModR/M
                        self.code.push(0x48); // REX.W prefix
                        self.code.push(0xf7);
                        self.code.push(0xf8 | self.register_code(reg));
                    }
                    _ => {
                        return Err(CodegenError {
                            message: "Div only supports register operands".to_string(),
                        });
                    }
                }
            }
            Instruction::Syscall => {
                self.code.push(0x0f);
                self.code.push(0x05);
            }
        }
        Ok(())
    }

    fn register_code(&self, reg: &Register) -> u8 {
        match reg {
            Register::Rax => 0,
            Register::Rbx => 3,
            Register::Rcx => 1,
            Register::Rdx => 2,
            Register::Rsp => 4,
            Register::Rbp => 5,
            Register::Rsi => 6,
            Register::Rdi => 7,
        }
    }

    fn parse_offset(&self, mem: &str) -> Result<i32, CodegenError> {
        if let Some(pos) = mem.find('+') {
            mem[pos + 1..].parse().map_err(|_| CodegenError {
                message: "Invalid memory offset".to_string(),
            })
        } else if let Some(pos) = mem.find('-') {
            let offset: i32 = mem[pos + 1..].parse().map_err(|_| CodegenError {
                message: "Invalid memory offset".to_string(),
            })?;
            Ok(-offset)
        } else {
            Ok(0)
        }
    }

    fn add_relocation(&mut self, symbol: String, rel_type: RelocationType) {
        self.relocations.push(Relocation {
            offset: self.code.len() as u64,
            symbol,
            rel_type,
        });
    }

    fn resolve_relocations(&mut self) -> Result<(), CodegenError> {
        for reloc in &self.relocations {
            let target_addr = self
                .symbol_table
                .get(&reloc.symbol)
                .ok_or_else(|| CodegenError {
                    message: format!("Undefined symbol: {}", reloc.symbol),
                })?;

            let current_addr = reloc.offset + 4; // Address after the instruction
            let relative_addr = (*target_addr as i64) - (current_addr as i64);

            if relative_addr < i32::MIN as i64 || relative_addr > i32::MAX as i64 {
                return Err(CodegenError {
                    message: "Relative address out of range".to_string(),
                });
            }

            let bytes = (relative_addr as i32).to_le_bytes();
            for (i, &byte) in bytes.iter().enumerate() {
                self.code[reloc.offset as usize + i] = byte;
            }
        }
        Ok(())
    }

    // Generate minimal ELF executable
    pub fn generate_elf(&self, machine_code: &[u8]) -> Vec<u8> {
        let mut elf = Vec::new();

        // ELF header
        let base_addr = 0x400000u64;
        let entry_point = base_addr + 0x78; // After ELF header + program header

        // ELF identification
        elf.extend_from_slice(&[0x7f, 0x45, 0x4c, 0x46]); // ELF magic
        elf.push(0x02); // 64-bit
        elf.push(0x01); // Little endian
        elf.push(0x01); // ELF version
        elf.push(0x00); // System V ABI
        elf.extend_from_slice(&[0; 8]); // Padding

        // ELF header fields
        elf.extend_from_slice(&2u16.to_le_bytes()); // Executable file
        elf.extend_from_slice(&0x3eu16.to_le_bytes()); // x86-64
        elf.extend_from_slice(&1u32.to_le_bytes()); // Version
        elf.extend_from_slice(&entry_point.to_le_bytes()); // Entry point
        elf.extend_from_slice(&64u64.to_le_bytes()); // Program header offset
        elf.extend_from_slice(&0u64.to_le_bytes()); // Section header offset
        elf.extend_from_slice(&0u32.to_le_bytes()); // Flags
        elf.extend_from_slice(&64u16.to_le_bytes()); // ELF header size
        elf.extend_from_slice(&56u16.to_le_bytes()); // Program header size
        elf.extend_from_slice(&1u16.to_le_bytes()); // Program header count
        elf.extend_from_slice(&0u16.to_le_bytes()); // Section header size
        elf.extend_from_slice(&0u16.to_le_bytes()); // Section header count
        elf.extend_from_slice(&0u16.to_le_bytes()); // Section name string table index

        // Program header (LOAD segment)
        elf.extend_from_slice(&1u32.to_le_bytes()); // PT_LOAD
        elf.extend_from_slice(&5u32.to_le_bytes()); // PF_R | PF_X (readable, executable)
        elf.extend_from_slice(&0u64.to_le_bytes()); // Offset in file
        elf.extend_from_slice(&base_addr.to_le_bytes()); // Virtual address
        elf.extend_from_slice(&base_addr.to_le_bytes()); // Physical address
        let total_size = 120u64 + machine_code.len() as u64; // ELF header + program header + code
        elf.extend_from_slice(&total_size.to_le_bytes()); // Size in file
        elf.extend_from_slice(&total_size.to_le_bytes()); // Size in memory
        elf.extend_from_slice(&0x1000u64.to_le_bytes()); // Alignment

        // Machine code
        elf.extend_from_slice(machine_code);

        elf
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

// High-level compilation function
pub fn compile_to_executable(ast: &CstRoot, scope: &Scope) -> Result<Vec<u8>, CodegenError> {
    // Generate instructions
    let mut codegen = Codegen::new();
    let instructions = codegen.generate(ast, scope)?;

    // Assemble to machine code
    let mut assembler = Assembler::new();
    let machine_code = assembler.assemble(instructions)?;

    // Generate ELF executable
    let elf = assembler.generate_elf(&machine_code);

    Ok(elf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rue_lexer::Lexer;

    fn compile_program(source: &str) -> Result<Vec<Instruction>, CodegenError> {
        // Parse
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let ast = rue_parser::parse(tokens).map_err(|e| CodegenError {
            message: format!("Parse error: {}", e.message),
        })?;

        // Semantic analysis
        let scope = rue_semantic::analyze_cst(&ast).map_err(|e| CodegenError {
            message: format!("Semantic error: {}", e.message),
        })?;

        // Code generation
        let mut codegen = Codegen::new();
        codegen.generate(&ast, &scope)
    }

    #[test]
    fn test_simple_main() {
        let instructions = compile_program(
            r#"
fn main() {
    42
}
"#,
        );
        assert!(instructions.is_ok());
        let instrs = instructions.unwrap();

        // Should have program setup and main function
        assert!(
            instrs
                .iter()
                .any(|i| matches!(i, Instruction::Label(l) if l == "_start"))
        );
        assert!(
            instrs
                .iter()
                .any(|i| matches!(i, Instruction::Label(l) if l == "main"))
        );
        assert!(instrs.iter().any(|i| matches!(
            i,
            Instruction::Mov(Operand::Register(Register::Rax), Operand::Immediate(42))
        )));
    }

    #[test]
    fn test_arithmetic() {
        let instructions = compile_program(
            r#"
fn main() {
    2 + 3
}
"#,
        );
        assert!(instructions.is_ok());
        let instrs = instructions.unwrap();

        // Should contain arithmetic operations
        assert!(instrs.iter().any(|i| matches!(i, Instruction::Add(_, _))));
    }

    #[test]
    fn test_function_with_parameter() {
        let instructions = compile_program(
            r#"
fn test(x) {
    x
}

fn main() {
    test(5)
}
"#,
        );
        assert!(instructions.is_ok());
    }

    #[test]
    fn test_assembler_simple() {
        let instructions = vec![
            Instruction::Label("_start".to_string()),
            Instruction::Mov(Operand::Register(Register::Rax), Operand::Immediate(42)),
            Instruction::Mov(
                Operand::Register(Register::Rdi),
                Operand::Register(Register::Rax),
            ),
            Instruction::Mov(Operand::Register(Register::Rax), Operand::Immediate(60)),
            Instruction::Syscall,
        ];

        let mut assembler = Assembler::new();
        let result = assembler.assemble(instructions);
        assert!(result.is_ok());

        let machine_code = result.unwrap();
        assert!(!machine_code.is_empty());
    }

    #[test]
    fn test_elf_generation() {
        let machine_code = vec![
            0x48, 0xc7, 0xc0, 0x2a, 0x00, 0x00, 0x00, // mov rax, 42
            0x48, 0x89, 0xc7, // mov rdi, rax
            0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00, // mov rax, 60
            0x0f, 0x05, // syscall
        ];

        let assembler = Assembler::new();
        let elf = assembler.generate_elf(&machine_code);

        // Check ELF magic
        assert_eq!(&elf[0..4], &[0x7f, 0x45, 0x4c, 0x46]);
        // Check that machine code is included
        assert!(elf.len() > machine_code.len());
    }

    #[test]
    fn test_factorial_compilation() {
        let factorial_source = r#"
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    factorial(5)
}
"#;

        // Parse
        let mut lexer = Lexer::new(factorial_source);
        let tokens = lexer.tokenize();
        let ast = rue_parser::parse(tokens).expect("Parse failed");

        // Semantic analysis
        let scope = rue_semantic::analyze_cst(&ast).expect("Semantic analysis failed");

        // Code generation
        let executable = compile_to_executable(&ast, &scope);
        if let Err(ref e) = executable {
            println!("Error: {}", e.message);
        }
        assert!(executable.is_ok());

        let elf = executable.unwrap();
        // Should produce a valid ELF executable
        assert_eq!(&elf[0..4], &[0x7f, 0x45, 0x4c, 0x46]); // ELF magic
        assert!(elf.len() > 200); // Should be reasonable size
    }
}
