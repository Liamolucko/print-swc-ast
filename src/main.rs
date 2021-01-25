use std::io::Read;

use anyhow::Error;
use swc_common::BytePos;
use swc_ecmascript::parser::Parser;
use swc_ecmascript::parser::StringInput;
use swc_ecmascript::parser::Syntax;
use swc_ecmascript::parser::TsConfig;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = StringInput::new(&input, BytePos(0), BytePos(input.len() as u32));
    let mut parser = Parser::new(Syntax::Typescript(TsConfig::default()), input, None);
    println!(
        "{:#?}",
        parser
            .parse_program()
            .map_err(|e| Error::msg(e.kind().msg()))?
    );
    Ok(())
}
