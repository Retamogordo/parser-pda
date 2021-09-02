
use crate::defs::{ SegmentDelimSyms::*, SegmentTypes, 
    ParserCtx, PDAStackCtx, ParsedSegment };

fsm!( ParserNonDetPDA<ParserCtx, PDAStackCtx> [
    Idle on Start          
        => NewSentence on LeftBracket  => PlainSegment,
    
    NewSentence on RightBracket => InvalidSegment,
    NewSentence on Letter(char) 
        => PlainSegment on LeftBracket 
        => PlainSegment on RightBracket => ?,

    PlainSegment on EndOfSentence => NewSentence,   
    _ on EndOfSentence => NewSentence,
    _ on EndOfInput => Idle,
    ],
    [
        Idle entry, NewSentence exit
    ]
);

on_state_handler!( Idle entry for ParserNonDetPDA {
    data.index = 0;
});
on_state_handler!( NewSentence exit for ParserNonDetPDA {
    stack.push(PDAStackCtx { sym: SentenceStart, seg_start: data.index} );
});
    
transition_handler!( Idle on Start for ParserNonDetPDA {
});

transition_handler!( NewSentence on Letter(char) for ParserNonDetPDA {
});

transition_handler!( NewSentence on LeftBracket for ParserNonDetPDA {
    stack.push(PDAStackCtx { sym: Bracket, seg_start: data.index } );
});

transition_handler!( NewSentence on RightBracket for ParserNonDetPDA {
});


transition_handler!( PlainSegment on LeftBracket for ParserNonDetPDA {
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

conditional_handler!(PlainSegment on RightBracket for ParserNonDetPDA {
    if &Bracket == stack.last().unwrap() {
        let top = stack.pop().unwrap();
        data.segments.push(ParsedSegment { 
            tp: SegmentTypes::Bracketed, 
            seg: (top.seg_start + 1, data.index),
            rank: stack.len(), 
        });
        PlainSegment            
    } else {
        let top = stack.last().unwrap();
        data.segments.push(ParsedSegment { 
            tp: SegmentTypes::Plain, 
            seg: (top.seg_start, data.index),
            rank: stack.len(),
        });
        stack.push(PDAStackCtx { sym: SegmentStart, seg_start: data.index } );
        InvalidSegment                
    }
});

transition_handler!(PlainSegment on EndOfSentence for ParserNonDetPDA {
    if let Some(_) = stack.last() {
        let top = stack.pop().unwrap();
        let tp = if SentenceStart == top {
            SegmentTypes::Sentence
        } else if SegmentStart == top {
            SegmentTypes::Plain
        } else { unimplemented!(); };
        data.segments.push(ParsedSegment { 
            tp, 
            seg: (top.seg_start, data.index),
            rank: stack.len(),
        });        
    }
});

transition_handler!(_ on EndOfSentence for ParserNonDetPDA {
    Self::drain_unbalanced(data, stack);
});

transition_handler!( _ on EndOfInput for ParserNonDetPDA {
    if let Some(top) = stack.last() {
        if &SentenceStart == top {
            data.segments.push(ParsedSegment { 
                tp: SegmentTypes::Tail, 
                seg: (stack.pop().unwrap().seg_start, data.index),
                rank: stack.len(),
            });
        } else {
            Self::drain_unbalanced(data, stack);
        }
    }
});

transition_handler!( NewSentence on EndOfSentence for ParserNonDetPDA {
});

impl ParserNonDetPDA {
    pub fn start(&mut self) {
        self.signal(&Start);
    }
    pub fn next(&mut self, signal: &ParserNonDetPDASignals) {
        self.signal(signal);
        self.data.index += 1;
    }
    pub fn stop(&mut self) {
        self.signal(&EndOfInput);
    }

    fn drain_unbalanced(
        data: &mut <Self as ParserNonDetPDATrait>::DataItem, 
        stack: &mut Vec<<Self as ParserNonDetPDATrait>::StackSymbolItem>,
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
        
