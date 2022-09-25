#![feature(trait_alias)]

mod ast;
mod fmt;
mod lexer;
mod parser;

use langbox::*;
use lexer::*;
use std::rc::Rc;

type SharedString = Rc<str>;

const TEST_FILE: &'static str = r#"
struct WritePort<ADDR_BITS, DATA_BITS> {
    clk: bit,
    en: bit,
    addr: bits<ADDR_BITS>,
    data: bits<DATA_BITS>,
}

struct ReadPort<ADDR_BITS> {
    clk: bit,
    en: bit,
    addr: bits<ADDR_BITS>,
}

mod Bram<ADDR_BITS, DATA_BITS>(
    in sig wport: WritePort<ADDR_BITS, DATA_BITS>,
    
    in sig rport: ReadPort<ADDR_BITS>,
    out reg rdata: bits<DATA_BITS>,
) {
    reg mem: [u<DATA_BITS>; 1 << ADDR_BITS];
    
    proc rising(wport.clk) {
        if wport.en {
            mem[wport.addr] = wport.data;
        }
    }
    
    proc rising(rport.clk) {
        if rport.en {
            rdata = mem[rport.addr];
        }
    }
}

fn clog2(bits) {
    let log = 0;
    while bits != 0 {
        bits >>= 1;
        log += 1;
    }
    log
}

enum AluOp: bits<4> {
    Add,
    AddC,
    Sub,
    SubB,
    And,
    Or,
    Xor,
    Shl,
    Lsr,
    Asr,
    Mul,
}

struct Flags {
    c: bit,
    z: bit,
    s: bit,
    o: bit,
}

mod Adder<BITS>(
    in sig lhs: bits<BITS>,
    in sig rhs: bits<BITS>,
    out sig result: bits<BITS>,
    
    in sig c_in: bit,
    out sig c_out: bit,
) {
    sig sum: bits<{BITS + 1}>;
    
    comb {
        sum = (0 as bit)@lhs + (0 as bit)@rhs + (0 as bits<BITS>)@c_in;
        result = sum[0..BITS];
        c_out = sum[BITS];
    }
}

mod Alu<BITS>(
    in sig lhs: bits<BITS>,
    in sig rhs: bits<BITS>,
    out sig result: bits<BITS>,
    
    in sig flags_in: Flags,
    out sig flags_out: Flags,
    
    in sig op: AluOp,
) {
    let adder: Adder<BITS>;
    comb {
        adder.lhs = lhs;
        adder.rhs = if (op == AluOp::Sub) | (op == AluOp::SubB) {
            !rhs
        } else {
            rhs
        };
        adder.c_in = match op {
            AluOp::Add => 0 as bit,
            AluOp::AddC => flags_in.c,
            AluOp::Sub => 1 as bit,
            AluOp::SubB => flags_in.c,
            _ => 0 as bit,
        };
    }
    
    const SHIFT_AMT_BITS = clog2(BITS);
    sig shift_amt: bits<SHIFT_AMT_BITS>;
    
    comb {
        shift_amt = rhs[0..SHIFT_AMT_BITS];
        
        result = match op {
            AluOp::Add => adder.result,
            AluOp::AddC => adder.result,
            AluOp::Sub => adder.result,
            AluOp::SubB => adder.result,
            AluOp::And => lhs & rhs,
            AluOp::Or => lhs | rhs,
            AluOp::Xor => lhs ^ rhs,
            AluOp::Shl => lhs << shift_amt,
            AluOp::Lsr => lhs >> shift_amt,
            AluOp::Asr => lhs >>> shift_amt,
            AluOp::Mul => lhs * rhs,
        };
    }
    
    sig lhs_s: bit;
    sig rhs_s: bit;
    
    sig z: bit;
    sig s: bit;
    sig o: bit;
    
    comb {
        lhs_s = lhs[BITS - 1];
        rhs_s = rhs[BITS - 1];

        z = result == 0 as bits<BITS>;
        s = result[BITS - 1];
        o = (lhs_s == rhs_s) & (lhs_s == s);

        flags_out = match op {
            AluOp::Add | AluOp::AddC | AluOp::Sub | AluOp::SubB => Flags {
                c: adder.c_out,
                z: z,
                s: s,
                o: o,
            },
            AluOp::And | AluOp::Or | AluOp::Xor | AluOp::Shl | AluOp::Lsr | AluOp::Asr => Flags {
                c: flags_in.c,
                z: z,
                s: flags_in.s,
                o: flags_in.o,
            },
            AluOp::Mul => Flags {
                c: flags_in.c,
                z: z,
                s: s,
                o: o,
            },
        };
    }
}
"#;

fn main() -> std::io::Result<()> {
    let mut file_server = FileServer::new();
    //let file = file_server.register_file("C:/Users/Mathis/Desktop/test.qrz")?;
    let file = file_server.register_file_memory("<test>", TEST_FILE);

    let lexer = QuartzLexer::new(file, &file_server);
    let tokens: Vec<_> = lexer
        .filter(|t| {
            if let QuartzToken::Comment(_) = &t.kind {
                false
            } else {
                true
            }
        })
        .collect();

    match parser::design().run(TokenStream::new(&tokens)) {
        ParseResult::Match { value: design, .. } => {
            for (i, item) in design.iter().enumerate() {
                if i > 0 {
                    print!("\n\n");
                }
                print!("{}", item);
            }
        }
        ParseResult::NoMatch => panic!("input did not match"),
        ParseResult::Err(err) => panic!("parse error: {:?}", err),
    }

    Ok(())
}
