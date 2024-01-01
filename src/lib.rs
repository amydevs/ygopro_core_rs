#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::*;
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
    card_handler: Box<dyn FnMut(u32, *mut OCG_CardData) + 'static>,
    script_handler: Box<dyn FnMut(&OCGDuelInstance, &str) -> i32 + 'static>,
    script_handler_wrapper: Box<dyn FnMut(*mut os::raw::c_void, &str) -> i32 + 'static>,
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
            card_handler: Box::new(|_, _| {}),
            script_handler: Box::new(|_, _| {
                0
            }),
            script_handler_wrapper: Box::new(|_, _| {
                0
            }),
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
        self.card_handler = Box::new(callback);
    }
    pub fn set_script_handler<F>(&mut self, callback: F)
        where F: FnMut(&OCGDuelInstance, &str) -> i32,
              F: 'static
    {
        self.script_handler = Box::new(callback);
    }
    extern "C" fn card_handler_raw(cb: *mut os::raw::c_void, code: u32, data: *mut OCG_CardData) {
        let closure: &mut Box<dyn FnMut(u32, *mut OCG_CardData)> = unsafe { mem::transmute(cb) };
        closure(code, data)
    }
    extern "C" fn script_handler_raw(cb: *mut os::raw::c_void, duel_ptr: *mut os::raw::c_void, name: *const i8) -> i32 {
        let nameStr = unsafe {
            ffi::CStr::from_ptr(name)
        };
        let closure: &mut Box<dyn FnMut(*mut os::raw::c_void, &str) -> i32> = unsafe { mem::transmute(cb) };
        closure(duel_ptr, nameStr.to_str().unwrap())
    }
    pub fn build(mut self) -> OCGDuelInstance {
        let mut duel = OCGDuelInstance {
            ptr: std::ptr::null_mut()
        };
        // This needs to be done so that the script_handler is able to access the duel instance
        // We can assume that the duel instance will be valid for the lifetime of the script_handler,
        // as this is only called on OCGDuelInstance::new_card and OCG_CreateDuel.
        // Because this is called on OCG_CreateDuel, we need to set the pointer here,
        // so that the instance will have the correct pointer when OCG_CreateDuel is called.
        let mut_ptr = &mut duel as *mut OCGDuelInstance;
        self.script_handler_wrapper = Box::new(move |duel_ptr, name| {
            if duel_ptr != mut_ptr as *mut _ {
                unsafe {
                    let mut_ptr_mut = mut_ptr.as_mut().unwrap();
                    mut_ptr_mut.ptr = duel_ptr;
                    return (self.script_handler)(mut_ptr_mut, name);
                }
            }
            unsafe {
                let mut_ptr_ref = mut_ptr.as_ref().unwrap();
                (self.script_handler)(mut_ptr_ref, name)
            }
        });
        // cardReader: self.card_handler.as_ref().and(Some(Self::card_handler_raw)),
        // payload1: self.card_handler.map_or(std::ptr::null_mut(), |cb| Box::into_raw(cb) as *mut _)
        // Double indirection is required for the callback pointers
        let options = OCG_DuelOptions {
            cardReader: Some(Self::card_handler_raw),
            payload1: Box::into_raw(Box::new(self.card_handler)) as *mut _,
            scriptReader: Some(Self::script_handler_raw),
            payload2: Box::into_raw(Box::new(self.script_handler_wrapper)) as *mut _,
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
        unsafe {
            OCG_CreateDuel(&mut duel.ptr, options);
        }
        return duel;
    } 
}

pub struct OCGDuelInstance {
    ptr: *mut os::raw::c_void,
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
        let mut duel_builder = OCGDuelBuilder::default();
        duel_builder.set_script_handler(|duel, name| {
            println!("{:?} {}", duel.ptr, name);
            1
        });
        let duel = duel_builder.build();
        assert!(!duel.ptr.is_null());
        // duel.new_card(OCG_NewCardInfo { team: 0, duelist: 1, code: 1, con: 0, loc: 0, seq: 0, pos: 0 });
    }
}