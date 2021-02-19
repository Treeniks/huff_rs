#[derive(Debug)]
pub struct HufTreeNode {
    val: u8,
    freq: usize,
    left: i16,
    right: i16,
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

    #[inline]
    pub fn val(&self) -> u8 {
        self.val
    }

    #[inline]
    pub fn freq(&self) -> usize {
        self.freq
    }
}
