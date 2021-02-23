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
        Self {
            val,
            freq,
            left,
            right,
        }
    }

    #[inline]
    pub fn to_short(self) -> ShortHufTreeNode {
        ShortHufTreeNode {
            val: self.val,
            left: self.left,
            right: self.right,
        }
    }
}

impl ShortHufTreeNode {
    #[inline]
    pub fn new(val: u8, left: i16, right: i16) -> Self {
        Self { val, left, right }
    }
}

#[readonly::make]
#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct ShortHufTreeNode {
    pub val: u8,
    pub left: i16,
    pub right: i16,
}
