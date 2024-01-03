use crate::ffi::OCG_CardData;

#[derive(Debug, Clone, Default)]
pub struct CardData {
    pub code: u32,
    pub alias: u32,
    pub setcode: u16,
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
        // let mut setcodes = HashSet::new();
        // let mut ptr = value.setcodes;
        // while !ptr.is_null() {
        //     unsafe {
        //         if (*ptr) == 0 {
        //             break;
        //         }
        //         setcodes.insert(*ptr);
        //     }
        //     ptr = ptr.wrapping_offset(1);
        // }
        Self {
            code: value.code,
            alias: value.alias,
            setcode: unsafe { *value.setcodes },
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

impl Into<OCG_CardData> for CardData {
    fn into(self) -> OCG_CardData {
        OCG_CardData {
            code: self.code,
            alias: self.alias,
            setcodes: Box::into_raw(Box::new(self.setcode)),
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
}
