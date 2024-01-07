use std::collections::HashSet;

use crate::ffi::{OCG_CardData, OCG_NewCardInfo};

#[derive(Debug, Clone, Default)]
pub struct NewCardInfo {
    pub team: u8,
    pub duelist: u8,
    pub code: u32,
    pub con: u8,
    pub loc: u32,
    pub seq: u32,
    pub pos: u32,
}

impl Into<OCG_NewCardInfo> for NewCardInfo {
    fn into(self) -> OCG_NewCardInfo {
        OCG_NewCardInfo {
            team: self.team,
            duelist: self.duelist,
            code: self.code,
            con: self.con,
            loc: self.loc,
            seq: self.seq,
            pos: self.pos,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CardData {
    pub code: u32,
    pub alias: u32,
    /// # Warning
    /// Any values of 0 in the hashset are ignored,
    /// as it is the sentinal value used to indicate the end of the buffer converted and not a valid setcode anyway.
    pub setcodes: HashSet<u16>,
    pub card_type: u32,
    pub level: u32,
    pub attribute: u32,
    pub race: u64,
    pub attack: i32,
    pub defense: i32,
    pub lscale: u32,
    pub rscale: u32,
    pub link_marker: u32,
}

impl From<OCG_CardData> for CardData {
    fn from(value: OCG_CardData) -> Self {
        let mut setcodes = HashSet::new();
        let mut ptr = value.setcodes;
        while !ptr.is_null() {
            unsafe {
                if (*ptr) == 0 {
                    break;
                }
                setcodes.insert(*ptr);
            }
            ptr = ptr.wrapping_offset(1);
        }
        Self {
            code: value.code,
            alias: value.alias,
            setcodes,
            card_type: value.type_,
            level: value.level,
            attribute: value.attribute,
            race: value.race,
            attack: value.attack,
            defense: value.defense,
            lscale: value.lscale,
            rscale: value.rscale,
            link_marker: value.link_marker,
        }
    }
}

impl CardData {
    fn into_ocg_carddata_internal(self, leaky: bool) -> OCG_CardData {
        let mut setcodes = Vec::with_capacity(self.setcodes.len() + 1);
        for setcode in self.setcodes.into_iter() {
            if setcode != 0 {
                setcodes.push(setcode);
            }
        }
        setcodes.push(0);
        setcodes.shrink_to_fit();
        let ptr = setcodes.as_mut_ptr();
        if leaky {
            std::mem::forget(setcodes);
        }
        OCG_CardData {
            code: self.code,
            alias: self.alias,
            setcodes: ptr,
            type_: self.card_type,
            level: self.level,
            attribute: self.attribute,
            race: self.race,
            attack: self.attack,
            defense: self.defense,
            lscale: self.lscale,
            rscale: self.rscale,
            link_marker: self.link_marker,
        }
    }
    /// # Warning
    /// This method will make the internal Vec<u16> created for the setcode pointer to be forgotten by the Rust borrow checker.
    /// This should only be used internally, and in cases where there is a deallocation mecanism set.
    /// Deallocation of setcodes should be done in set_card_read_done_handler.
    /// https://stackoverflow.com/questions/39224904/how-to-expose-a-rust-vect-to-ffi
    pub fn into_ocg_carddata_leaky(self) -> OCG_CardData {
        self.into_ocg_carddata_internal(true)
    }
}

impl From<CardData> for OCG_CardData {
    fn from(val: CardData) -> Self {
        val.into_ocg_carddata_internal(false)
    }
}
