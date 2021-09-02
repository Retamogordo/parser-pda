# parser-pda

 This crate provides a demo of using [fi_night](https://github.com/Retamogordo/fi-night) crate for implementation 
 of a simple parser capable of indexing nested bracketed segments of text.
 The parser distinguishes sentences separated by '.' and ```[bracketed]``` text 
 segments.
 The automaton implementation can be seen in modules
 parser_pda.rs and parser_nondet_pda.rs which implement two
 versions providing the same functionality. 
 
 Here is an example:
```rust 
 use parser_pda::{create_parser_pda_instance} ;
 use parser_pda::parser_pda::{LittleParser, LittleParserTrait, LittleParserStates::*, LittleParserSignals::*,
     LITTLE_PARSER_GEN_CODE};
 
 fn main() {
     let mut parser = create_parser_pda_instance();
 
     fsm_code_to_file("parser_automaton", "target/fsm", LITTLE_PARSER_GEN_CODE);
     
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
 
 use std::fs::{File, create_dir};
 use std::io::{Seek, Write};
 
 fn fsm_code_to_file(fname: &str, path: &str, gen_code: &str) {
 
     let _ = create_dir(path);
 
     File::create(&format!("{}/{}.rs", path, fname))
         .and_then(|mut file| {
             file.seek(std::io::SeekFrom::End(0))?;
             file.write_all(gen_code.to_string().as_bytes())?;
             file.flush()
         });
 }
 ```
 
 The code running stores the rust code of the automaton under
 /target/fsm/parser_automaton.rs
 and emits the following output: 
 ```text
 input: Fif[teen] men on a dead man's [[chest]]. []]Yo [[ho [[ho. And [the [bottle] ]of [[[RUM]]] 

 Sentence[0 39) -> Fif[teen] men on a dead man's [[chest]]
     Plain[0 30) -> Fif[teen] men on a dead man's 
     Plain[0 3) -> Fif
     Bracketed[4 8) -> teen
     Bracketed[31 38) -> [chest]
         Bracketed[32 37) -> chest
 InvalidSentence[40 56) ->  []]Yo [[ho [[ho
     Plain[40 43) ->  []
     Plain[40 41) ->  
     Bracketed[42 42) -> 
     UnbalancedRightSth[43 56) -> ]Yo [[ho [[ho
 Tail[57 90) ->  And [the [bottle] ]of [[[RUM]]] 
     Plain[57 80) ->  And [the [bottle] ]of 
     Plain[57 62) ->  And 
     Bracketed[63 76) -> the [bottle] 
         Bracketed[68 74) -> bottle
     Bracketed[81 88) -> [[RUM]]
         Bracketed[82 87) -> [RUM]
             Bracketed[83 86) -> RUM
```
