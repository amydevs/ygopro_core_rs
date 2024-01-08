use ffi::OCG_QueryInfo;

#[derive(Debug, Clone, Copy, Default)]
pub struct QueryInfo {
    pub flags: u32,
    pub con: u8,
    pub loc: u32,
    pub seq: u32,
    pub overlay_seq: u32,
}

impl From<QueryInfo> for OCG_QueryInfo {
    fn from(val: QueryInfo) -> Self {
        OCG_QueryInfo {
            flags: val.flags,
            con: val.con,
            loc: val.loc,
            seq: val.seq,
            overlay_seq: val.overlay_seq,
        }
    }
}
