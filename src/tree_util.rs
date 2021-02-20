#[readonly::make]
#[derive(Debug, Clone, Copy)]
pub struct HufTreeNode {
    pub val: u8,
    pub freq: usize,
    pub left: i16,
    pub right: i16,
}

impl HufTreeNode {
    #[inline]
    pub fn new(val: u8, freq: usize, left: i16, right: i16) -> Self {
        HufTreeNode {
            val,
            freq,
            left,
            right,
        }
    }
}
