use std::env::args;
use std::fmt::format;
use std::fs::read_to_string;
use std::io::{Result, Write};
use std::ptr::write;
use koopa::back::KoopaGenerator;
use koopa::ir::{Value, ValueKind};
use lalrpop_util::lalrpop_mod;
mod ast;
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    match mode.as_str() {
        "-koopa" => koopa_ir_gen(input, output)?,
        "-riscv" => riscv_ir_gen(input, output)?,
        _ => println!("错误：请输入正确的参数(-koopa/-riscv)"),
    }
    Ok(())
}

fn koopa_ir_gen(input: String, output: String) -> Result<()> {
    let input = read_to_string(input)?;

    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

    let mut output_file = std::fs::File::create(&output).unwrap();
    let koopa_string = format!("{}", ast);
    output_file.write_all(koopa_string.as_bytes()).expect("write fail..."); // 生成IR
    Ok(())
}

fn riscv_ir_gen(input: String, output: String) -> Result<()> {
    let input = read_to_string(input)?;
    let mut output_file = std::fs::File::create(&output).unwrap();
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();
    let koopa_string = format!("{}", ast);
    let driver = koopa::front::Driver::from(koopa_string);
    let program = driver.generate_program().unwrap();
    // todo: 如何将koopa-ir结果转化到RISCV
    for &func in program.func_layout() {
        let func_data = program.func(func);
        output_file.write(format!("{}:\n",func_data.name().replace("@", "")).as_bytes())?;
        for (&bb, node) in func_data.layout().bbs() {
            // println!("{:?}", bb);
            for &inst in node.insts().keys() {
                let value_data = func_data.dfg().value(inst);
                // println!("{:?}", value_data.kind());
                match value_data.kind() {
                    ValueKind::Return(v) => {
                        match v.value() {
                            Some(vel)=> {
                                let ret_value_data = func_data.dfg().value(vel);
                                match ret_value_data.kind() {
                                    ValueKind::Integer(v) => {
                                        output_file.write(format!("\tli a0, {}\n", v.value()).as_bytes())?;
                                    },
                                    _ => (),
                                }
                            },
                            None => (),
                        }
                        output_file.write("\tret\n".as_bytes())?;
                    },
                    _ => (),
                }
            }
        }
    }
    Ok(())
}
