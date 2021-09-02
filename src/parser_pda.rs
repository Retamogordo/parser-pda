
use crate::defs::{ SegmentDelimSyms::*, SegmentTypes,
    ParserCtx, PDAStackCtx, ParsedSegment };

    
fsm!( LittleParser<ParserCtx, PDAStackCtx> [
    Idle on Start          
        => NewSentence on LeftBracket  => PlainSegment,
    
    NewSentence on RightBracket => InvalidSegment,
    NewSentence on Letter(char) 
        => PlainSegment on LeftBracket 
        => PlainSegment on RightBracket peek_cmp Bracket
        => PlainSegment on RightBracket => InvalidSegment, 

    PlainSegment on EndOfSentence peek_cmp SentenceStart => NewSentence,   
    PlainSegment on EndOfSentence peek_cmp SegmentStart => NewSentence,   
    _ on EndOfSentence => NewSentence,
    _ on EndOfInput peek_cmp SentenceStart => Idle,          
    _ on EndOfInput => Idle,
],
[
    Idle entry, NewSentence exit
]);
  

on_state_handler!( Idle entry for LittleParser {
    data.index = 0;
});
on_state_handler!( NewSentence exit for LittleParser {
    stack.push(PDAStackCtx { sym: SentenceStart, seg_start: data.index} );
});

transition_handler!( Idle on Start for LittleParser {
});

transition_handler!( NewSentence on Letter(char) for LittleParser {
});

transition_handler!( NewSentence on LeftBracket for LittleParser {
    stack.push(PDAStackCtx { sym: Bracket, seg_start: data.index } );
});

transition_handler!( NewSentence on RightBracket for LittleParser {
});

transition_handler!( PlainSegment on LeftBracket for LittleParser {
    let top = stack.last().unwrap();
    if top.sym == SegmentStart || top.sym == SentenceStart {
        data.segments.push(ParsedSegment { 
            tp: SegmentTypes::Plain, 
            seg: (top.seg_start, data.index),
            rank: stack.len(), 
        });
    }
    stack.push(PDAStackCtx { sym: Bracket, seg_start: data.index} );
});

transition_handler!(PlainSegment on RightBracket peek_cmp Bracket for LittleParser {
    let top = stack.pop().unwrap();
    data.segments.push(ParsedSegment { 
        tp: SegmentTypes::Bracketed, 
        seg: (top.seg_start + 1, data.index),
        rank: stack.len(), 
    });
});

transition_handler!(PlainSegment on RightBracket for LittleParser {
    let top = stack.last().unwrap();
    data.segments.push(ParsedSegment { 
        tp: SegmentTypes::Plain, 
        seg: (top.seg_start, data.index),
        rank: stack.len(),
    });
    stack.push(PDAStackCtx { sym: SegmentStart, seg_start: data.index } );
});

transition_handler!(PlainSegment on EndOfSentence peek_cmp SentenceStart for LittleParser {
    let top = stack.pop().unwrap();
    data.segments.push(ParsedSegment { 
        tp: SegmentTypes::Sentence, 
        seg: (top.seg_start, data.index),
        rank: stack.len(),
    });
});

transition_handler!(PlainSegment on EndOfSentence peek_cmp SegmentStart for LittleParser {
    let top = stack.pop().unwrap();
    data.segments.push(ParsedSegment { 
        tp: SegmentTypes::Plain, 
        seg: (top.seg_start, data.index),
        rank: stack.len(),
    });
});

transition_handler!(_ on EndOfSentence for LittleParser {
    Self::drain_unbalanced(data, stack);
});

transition_handler!( _ on EndOfInput peek_cmp SentenceStart for LittleParser {
    data.segments.push(ParsedSegment { 
        tp: SegmentTypes::Tail, 
        seg: (stack.pop().unwrap().seg_start, data.index),
        rank: stack.len(),
    });
});
    
transition_handler!( _ on EndOfInput for LittleParser {
    Self::drain_unbalanced(data, stack);
});

transition_handler!( NewSentence on EndOfSentence for LittleParser {
});
    
impl LittleParser {
    pub fn start(&mut self) {
        self.signal(&Start);
    }
    pub fn next(&mut self, signal: &LittleParserSignals) {
        self.signal(signal);
        self.data.index += 1;
    }
    pub fn stop(&mut self) {
        self.signal(&EndOfInput);
    }

    fn drain_unbalanced(
        data: &mut <Self as LittleParserTrait>::DataItem, 
        stack: &mut Vec<<Self as LittleParserTrait>::StackSymbolItem>,
    ) {
        while let Some(top) = stack.pop() {
            let tp = 
                if Bracket == top {  SegmentTypes::UnbalancedLeftBracket } 
                else if SegmentStart == top { SegmentTypes::UnbalancedRightSth}
                else if SentenceStart == top { SegmentTypes::InvalidSentence}
                else {
                    unimplemented!();
                };
            data.segments.push(ParsedSegment { 
                tp, 
                seg: (top.seg_start, data.index),
                rank: stack.len(),
            });
        }
    }
}

    


