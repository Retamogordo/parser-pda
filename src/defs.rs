use std::collections::BinaryHeap;
use strum_macros::Display;

use std::fs::{File, create_dir};
use std::io::{Seek, Write};

#[derive(Display, Clone, PartialEq)]
pub enum SegmentDelimSyms {
    SentenceStart,
    SegmentStart,
    Bracket,
}

impl PartialEq<PDAStackCtx> for SegmentDelimSyms {
    fn eq(&self, other: &PDAStackCtx) -> bool {
        match self {
            Self::SentenceStart => Self::SentenceStart == other.sym,
            Self::SegmentStart => Self::SegmentStart == other.sym,
            Self::Bracket => Self::Bracket == other.sym,
        }
    }
}  

pub struct PDAStackCtx {
    pub(crate) sym: SegmentDelimSyms,
    pub(crate) seg_start: usize,
}

impl PartialEq for PDAStackCtx {
    fn eq(&self, other: &PDAStackCtx) -> bool {
        self.sym == other.sym
    }
}  

#[derive(Display)]
pub enum SegmentTypes {
    Sentence,
    Tail,
    Plain,
    Bracketed,
    InvalidSentence,
    UnbalancedLeftBracket,
    UnbalancedRightSth,
}

pub struct ParsedSegment {
    pub tp: SegmentTypes,
    pub seg: (usize, usize),
    pub rank: usize,
}

impl std::cmp::PartialEq for ParsedSegment {
    fn eq(&self, other: &Self) -> bool {
        self.seg.0 == other.seg.0 && self.seg.1 == other.seg.1 && self.rank == other.rank
    }
}
impl std::cmp::Eq for ParsedSegment {}
impl std::cmp::PartialOrd for ParsedSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.seg.0.partial_cmp(&other.seg.0) {
            Some(std::cmp::Ordering::Less) => Some(std::cmp::Ordering::Greater),
            Some(std::cmp::Ordering::Equal) => self.seg.1.partial_cmp(&other.seg.1),
            Some(std::cmp::Ordering::Greater) => Some(std::cmp::Ordering::Less),
            _ => None,
        }            
    }
}
impl Ord for ParsedSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
    
pub struct ParserCtx {
    pub(crate) segments: BinaryHeap<ParsedSegment>,
    pub(crate) index: usize,
}

pub(crate) fn fsm_code_to_file(fname: &str, path: &str, gen_code: &str) {

    let _res = create_dir(path);

    File::create(&format!("{}/{}.rs", path, fname))
        .and_then(|mut file| {
            file.seek(std::io::SeekFrom::End(0))?;
            file.write_all(gen_code.to_string().as_bytes())?;
            file.flush()
        })
        .expect("file error");
}


