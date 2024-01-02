#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::*;
use ygopro_core_rs_sys::*;

pub struct OCGPlayer {
    starting_lp: u32,
    starting_draw_count: u32,
    draw_count_per_turn: u32,
}

impl Default for OCGPlayer {
    fn default() -> OCGPlayer {
        OCGPlayer {
            starting_lp: 8000,
            starting_draw_count: 5,
            draw_count_per_turn: 1,
        }
    }
}

impl From<OCGPlayer> for OCG_Player {
    fn from(val: OCGPlayer) -> Self {
        OCG_Player {
            startingLP: val.starting_lp,
            startingDrawCount: val.starting_draw_count,
            drawCountPerTurn: val.draw_count_per_turn,
        }
    }
}

pub struct OCGDuelBuilder {
    card_handler: Box<dyn FnMut(u32, *mut OCG_CardData) + 'static>,
    script_handler: Box<dyn FnMut(&OCGDuelInstance, &str) -> i32 + 'static>,
    script_handler_wrapper: Box<dyn FnMut(*mut os::raw::c_void, &str) -> i32 + 'static>,
    log_handler: Option<Box<dyn FnMut(&str, i32) + 'static>>,
    card_read_done_handler: Option<Box<dyn FnMut(*mut OCG_CardData) + 'static>>,
    seed: [u64; 4],
    flags: u64,
    team_1: OCGPlayer,
    team_2: OCGPlayer,
    enable_unsafe_libraries: bool,
}

impl Default for OCGDuelBuilder {
    fn default() -> OCGDuelBuilder {
        OCGDuelBuilder {
            card_handler: Box::new(|_, _| ()),
            script_handler: Box::new(|_, _| 0),
            script_handler_wrapper: Box::new(|_, _| 0),
            log_handler: None,
            card_read_done_handler: None,
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
    where
        F: FnMut(u32, *mut OCG_CardData),
        F: 'static,
    {
        self.card_handler = Box::new(callback);
    }
    pub fn set_script_handler<F>(&mut self, callback: F)
    where
        F: FnMut(&OCGDuelInstance, &str) -> i32,
        F: 'static,
    {
        self.script_handler = Box::new(callback);
    }
    pub fn set_log_handler<F>(&mut self, callback: F)
    where
        F: FnMut(&str, i32),
        F: 'static,
    {
        self.log_handler = Some(Box::new(callback));
    }
    extern "C" fn card_handler_ffi(cb: *mut os::raw::c_void, code: u32, data: *mut OCG_CardData) {
        let closure = unsafe { &mut *(cb as *mut Box<dyn FnMut(u32, *mut OCG_CardData)>) };
        closure(code, data)
    }
    extern "C" fn script_handler_ffi(
        cb: *mut os::raw::c_void,
        duel_ptr: *mut os::raw::c_void,
        name: *const i8,
    ) -> i32 {
        let nameStr = unsafe { ffi::CStr::from_ptr(name) };
        let closure = unsafe {
            &mut *(cb as *mut Box<dyn for<'a> FnMut(*mut std::ffi::c_void, &'a str) -> i32>)
        };
        closure(duel_ptr, nameStr.to_str().unwrap())
    }
    extern "C" fn log_handler_ffi(cb: *mut os::raw::c_void, msg: *const i8, msg_type: i32) {
        let msgStr = unsafe { ffi::CStr::from_ptr(msg) };
        let closure = unsafe { &mut *(cb as *mut Box<dyn for<'a> FnMut(&'a str, i32)>) };
        closure(msgStr.to_str().unwrap(), msg_type)
    }
    extern "C" fn card_read_done_handler_ffi(cb: *mut os::raw::c_void, data: *mut OCG_CardData) {
        let closure = unsafe { &mut *(cb as *mut Box<dyn FnMut(*mut OCG_CardData)>) };
        closure(data)
    }
    pub fn build(mut self) -> OCGDuelInstance {
        let mut duel = OCGDuelInstance {
            ptr: std::ptr::null_mut(),
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
        // Double indirection is required for the callback pointers
        let options = OCG_DuelOptions {
            cardReader: Some(Self::card_handler_ffi),
            payload1: Box::into_raw(Box::new(self.card_handler)) as *mut _,
            scriptReader: Some(Self::script_handler_ffi),
            payload2: Box::into_raw(Box::new(self.script_handler_wrapper)) as *mut _,
            logHandler: self.log_handler.as_ref().and(Some(Self::log_handler_ffi)),
            payload3: self
                .log_handler
                .map_or(std::ptr::null_mut(), |cb| Box::into_raw(cb) as *mut _),
            cardReaderDone: self
                .card_read_done_handler
                .as_ref()
                .and(Some(Self::card_read_done_handler_ffi)),
            payload4: self
                .card_read_done_handler
                .map_or(std::ptr::null_mut(), |cb| Box::into_raw(cb) as *mut _),
            seed: self.seed,
            flags: self.flags,
            team1: self.team_1.into(),
            team2: self.team_2.into(),
            enableUnsafeLibraries: self.enable_unsafe_libraries.into(),
        };
        unsafe {
            OCG_CreateDuel(&mut duel.ptr, options);
        }
        duel
    }
}

#[derive(Debug)]
pub enum OCGDuelStatus {
    End = OCG_DuelStatus_OCG_DUEL_STATUS_END as isize,
    Awaiting = OCG_DuelStatus_OCG_DUEL_STATUS_AWAITING as isize,
    Continue = OCG_DuelStatus_OCG_DUEL_STATUS_CONTINUE as isize,
}

impl From<OCG_DuelStatus> for OCGDuelStatus {
    fn from(status: OCG_DuelStatus) -> Self {
        match status {
            OCG_DuelStatus_OCG_DUEL_STATUS_END => OCGDuelStatus::End,
            OCG_DuelStatus_OCG_DUEL_STATUS_AWAITING => OCGDuelStatus::Awaiting,
            OCG_DuelStatus_OCG_DUEL_STATUS_CONTINUE => OCGDuelStatus::Continue,
            _ => panic!("Invalid OCG_DuelStatus"),
        }
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
        [major_version, minor_version]
    }
    // Lifecycle
    pub fn new_card(&self, info: OCG_NewCardInfo) {
        unsafe {
            OCG_DuelNewCard(self.ptr, info);
        }
    }
    pub fn start(&self) {
        unsafe {
            OCG_StartDuel(self.ptr);
        }
    }
    pub fn destroy(self) {
        drop(self);
    }
    // Processing
    pub fn process(&self) -> OCGDuelStatus {
        unsafe { (OCG_DuelProcess(self.ptr) as OCG_DuelStatus).into() }
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
        let duel_builder = OCGDuelBuilder::default();
        let duel = duel_builder.build();
        assert!(!duel.ptr.is_null());
        // duel.new_card(OCG_NewCardInfo { team: 0, duelist: 1, code: 1, con: 0, loc: 0, seq: 0, pos: 0 });
    }
}
