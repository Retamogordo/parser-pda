#[macro_use]
extern crate fi_night;

pub mod defs;
pub mod parser_pda;
pub mod parser_nondet_pda;

use crate::defs::{ ParserCtx, fsm_code_to_file };

pub fn create_parser_pda_instance() -> crate::parser_pda::LittleParser {
    crate::parser_pda::LittleParser::new(
        crate::parser_pda::LittleParserStates::Idle, 
        ParserCtx {
            segments: std::collections::BinaryHeap::new(),
            index: 0,
    })
}
pub fn create_parser_nondet_pda_instance() -> crate::parser_nondet_pda::ParserNonDetPDA {
    crate::parser_nondet_pda::ParserNonDetPDA::new(
        crate::parser_nondet_pda::ParserNonDetPDAStates::Idle, 
        ParserCtx {
            segments: std::collections::BinaryHeap::new(),
            index: 0,
    })
}

pub fn parser_pda_test() {
    use crate::parser_pda::{LittleParser, LittleParserTrait, LittleParserStates::*, LittleParserSignals::*,
        LITTLE_PARSER_GEN_CODE};

    crate::fsm_code_to_file("parser_pda", "target/fsm", LITTLE_PARSER_GEN_CODE);

    let mut parser = crate::parser_pda::LittleParser::new(
        Idle, 
        ParserCtx {
            segments: std::collections::BinaryHeap::new(),
            index: 0,
    });

    let text = "Fif[teen] men on a dead man's [[chest]]. []]Yo [[ho [[ho. And [the [bottle] ]of [[[RUM]]] ";

    parser.start();

    for ch in text.chars() {
            match ch {
            '[' => parser.next(&LeftBracket),
            ']' => parser.next(&RightBracket),
            '.' => parser.next(&EndOfSentence),
            ch @ _ => parser.next(&Letter(ch)),
        }
    }
    parser.stop();   

    display_output(&text, &mut parser);

    fn display_output(text: &str, parser: &mut LittleParser) {
        use substring::Substring;
        println!("input: {}\n", text);
        while let Some(seg) = parser.data_mut().segments.pop() {
            let tabbed = "\t".repeat(seg.rank);
    
            println!("{} {}[{} {}) -> {}", tabbed, seg.tp, seg.seg.0, seg.seg.1,
            text.substring(seg.seg.0, seg.seg.1));
        }   
    }
}

pub fn parser_nondet_pda_test() {
    use crate::parser_nondet_pda::{ParserNonDetPDA, ParserNonDetPDATrait, ParserNonDetPDAStates::*, ParserNonDetPDASignals::*,
        PARSER_NON_DET_PDA_GEN_CODE};

    crate::fsm_code_to_file("parser_nondet_pda", "target/fsm", PARSER_NON_DET_PDA_GEN_CODE);
    let mut parser = crate::parser_nondet_pda::ParserNonDetPDA::new(
        Idle, 
        ParserCtx {
            segments: std::collections::BinaryHeap::new(),
            index: 0,
    });

    let text = "Fif[teen] men on a dead man's [[chest]]. []]Yo [[ho [[ho. And [the [bottle] ]of [[[RUM]]] ";

    parser.start();

    for ch in text.chars() {
            match ch {
            '[' => parser.next(&LeftBracket),
            ']' => parser.next(&RightBracket),
            '.' => parser.next(&EndOfSentence),
            ch @ _ => parser.next(&Letter(ch)),
        }
    }
    parser.stop();   

    display_output(&text, &mut parser);

    fn display_output(text: &str, parser: &mut ParserNonDetPDA) {
        use substring::Substring;
        println!("input: {}\n", text);
        while let Some(seg) = parser.data_mut().segments.pop() {
            let tabbed = "\t".repeat(seg.rank);
    
            println!("{} {}[{} {}) -> {}", tabbed, seg.tp, seg.seg.0, seg.seg.1,
            text.substring(seg.seg.0, seg.seg.1));
        }   
    }
}
