fn main() {
    let mut ram: [u8; 4096] = [0; 4096];
    let mut reg: [i32; 4] = [0b00, 0b00, 0b00, 0b00];
    let mut ports: [i32; 16] = [0b00; 256];

    execute_line("0000101100000101".to_string(), &mut ram, &mut reg, &mut ports);
    execute_line("0000000100000001".to_string(), &mut ram, &mut reg, &mut ports);
    execute_line("0000001100100000".to_string(), &mut ram, &mut reg, &mut ports);
    execute_line("0000110000010010".to_string(), &mut ram, &mut reg, &mut ports);

    println!("Registers: {:?}", reg);
    println!("Port 1: {}", ports[1]);
}


fn execute_line(line: String, _ram: &mut [u8; 4096], reg: &mut [i32; 4], ports: &mut [i32; 16]) {
    let (operation, operand) = line.split_at(8);
    let (op1, op2) = operand.split_at(4);

    if operation == "00000001" { //SUM
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();

        reg[idx_dest] = reg[idx_dest] + reg[idx_src];
    } else if operation == "00000010" { //SUB
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();

        reg[idx_dest] = reg[idx_dest] - reg[idx_src];
    } else if operation == "00000011" { //MUL
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();

        reg[idx_dest] = reg[idx_dest] * reg[idx_src];
    } else if operation == "00000100" { //DIV
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();
        if reg[idx_src] != 0 {
            reg[idx_dest] = ((reg[idx_dest] as f64) / (reg[idx_src] as f64)).floor() as i32;
        }
    } else if operation == "00000101" { //NAND
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();

        reg[idx_dest] = !(reg[idx_dest] & reg[idx_src]);
    } else if operation == "00000110" { //OR
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();

        reg[idx_dest] = reg[idx_dest] | reg[idx_src];
    } else if operation == "00000111" { //XOR
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();

        reg[idx_dest] = reg[idx_dest] ^ reg[idx_src];
    } else if operation == "00001000" { //NOT
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();

        reg[idx_dest] = !reg[idx_dest];
    } else if operation == "00001001" { //AND
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();

        reg[idx_dest] = reg[idx_dest] & reg[idx_src];
    } else if operation == "00001010" { //NOR
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();

        reg[idx_dest] = !(reg[idx_dest] | reg[idx_src]);
    } else if operation == "00001011" { //MOV
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        reg[idx_dest] = usize::from_str_radix(op2, 2).unwrap() as i32;
    } else if operation == "00001100" { //PSN
        let idx_port = usize::from_str_radix(op1, 2).unwrap();
        let idx_src = usize::from_str_radix(op2, 2).unwrap();

        ports[idx_port] = reg[idx_src];
    } else if operation == "00001101" { //PRD
        let idx_dest = usize::from_str_radix(op1, 2).unwrap();
        let idx_port = usize::from_str_radix(op2, 2).unwrap();

        reg[idx_dest] = ports[idx_port];
    }
}
