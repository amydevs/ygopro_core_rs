#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ygopro_core_rs_sys::*;

pub struct OCGPlayer {
    starting_lp: u32,
    starting_draw_count: u32,
    draw_count_per_turn: u32
}

impl Default for OCGPlayer {
    fn default() -> OCGPlayer {
        OCGPlayer {
            starting_lp: 8000,
            starting_draw_count: 5,
            draw_count_per_turn: 1
        }
    }
}

impl Into<OCG_Player> for OCGPlayer {
    fn into(self) -> OCG_Player {
        OCG_Player {
            startingLP: self.starting_lp,
            startingDrawCount: self.starting_draw_count,
            drawCountPerTurn: self.draw_count_per_turn
        }
    }
}

pub struct OCGDuelBuilder {
    card_handler: Option<Box<Box<dyn FnMut(u32, *mut OCG_CardData) + 'static>>>,
    script_handler: Option<Box<dyn FnMut(OCGDuelInstance, &str) + 'static>>,
    log_handler: Option<Box<dyn FnMut(*const i8, i32) + 'static>>,
    card_reader_done_handler: Option<Box<dyn FnMut(*mut OCG_CardData) + 'static>>,
    seed: [u64; 4],
    flags: u64,
    team_1: OCGPlayer,
    team_2: OCGPlayer,
    enable_unsafe_libraries: bool
}

impl Default for OCGDuelBuilder {
    fn default() -> OCGDuelBuilder {
        OCGDuelBuilder {
            card_handler: None,
            script_handler: None,
            log_handler: None,
            card_reader_done_handler: None,
            seed: [0; 4],
            flags: 0,
            team_1: OCGPlayer::default(),
            team_2: OCGPlayer::default(),
            enable_unsafe_libraries: true,
        }
    }
}

impl OCGDuelBuilder {
    pub fn set_card_handler<F>(&mut self, callback: F)
        where F: FnMut(u32, *mut OCG_CardData),
              F: 'static
    {
        self.card_handler = Some(Box::new(Box::new(callback)));
    }
    pub fn unset_card_handler(&mut self) {
        self.card_handler = None;
    }
    pub fn set_script_handler<F>(&mut self, callback: F)
        where F: FnMut(OCGDuelInstance, &str),
              F: 'static
    {
        self.script_handler = Some(Box::new(callback));
    }
    extern "C" fn card_handler_raw(cb: *mut ::std::os::raw::c_void, code: u32, data: *mut OCG_CardData) {
        let closure: &mut Box<dyn FnMut(u32, *mut OCG_CardData)> = unsafe { ::std::mem::transmute(cb) };
        closure(code, data)
    }
    extern "C" fn script_handler_raw(cb: *mut ::std::os::raw::c_void, _ocg_duel: *mut ::std::os::raw::c_void, name: *const i8) -> i32 {
        let nameStr = unsafe {
            ::std::ffi::CStr::from_ptr(name)
        };
        let closure: &mut Box<dyn FnMut(&str) -> i32> = unsafe { ::std::mem::transmute(cb) };
        closure(nameStr.to_str().unwrap())
    }
    pub fn build(self) -> OCGDuelInstance {
        let options = OCG_DuelOptions {
            cardReader: self.card_handler.as_ref().and(Some(Self::card_handler_raw)),
            payload1: self.card_handler.map_or(std::ptr::null_mut(), |cb| Box::into_raw(cb) as *mut _),
            scriptReader: None,
            payload2: std::ptr::null_mut(),
            logHandler: None,
            payload3: std::ptr::null_mut(),
            cardReaderDone: None,
            payload4: std::ptr::null_mut(),
            seed: self.seed,
            flags: self.flags,
            team1: self.team_1.into(),
            team2: self.team_2.into(),
            enableUnsafeLibraries: self.enable_unsafe_libraries.into()
        };
        let mut pduel: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        unsafe {
            OCG_CreateDuel(&mut pduel, options);
        }
        return OCGDuelInstance {
            ptr: pduel
        };
    } 
}

pub struct OCGDuelInstance {
    ptr: *mut ::std::os::raw::c_void
}

impl Drop for OCGDuelInstance {
    fn drop(&mut self) {
        unsafe {
            OCG_DestroyDuel(self.ptr);
        }
    }
}

impl OCGDuelInstance {
    // Informative
    pub fn get_version() -> [i32; 2] {
        let mut major_version: i32 = 0;
        let mut minor_version: i32 = 0;
        unsafe {
            OCG_GetVersion(&mut major_version, &mut minor_version);
        }
        return [major_version, minor_version];
    }
    // Lifecycle
    pub fn new_card(&self, info: OCG_NewCardInfo) {
        unsafe {
            OCG_DuelNewCard(self.ptr, info);
        }
    }
    pub fn start_duel(&self) {
        unsafe {
            OCG_StartDuel(self.ptr);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        let version = OCGDuelInstance::get_version();
        assert!(version[0] >= 0);
        assert!(version[1] >= 0);
    }
    #[test]
    fn test_create_duel() {
    }
}