use std::os::raw::c_void;
use std::ptr::null_mut;
use std::ffi::{CStr, CString};

use crate::ffi::{
    OCG_CardData,
    OCG_DuelOptions,
    OCG_DuelStatus,
    OCG_NewCardInfo,
    OCG_QueryInfo,
    OCG_DuelStatus_OCG_DUEL_STATUS_END, 
    OCG_DuelStatus_OCG_DUEL_STATUS_AWAITING,
    OCG_DuelStatus_OCG_DUEL_STATUS_CONTINUE,
    OCG_GetVersion,
    OCG_CreateDuel,
    OCG_DestroyDuel,
    OCG_DuelNewCard,
    OCG_DuelProcess,
    OCG_StartDuel,
    OCG_DuelGetMessage,
    OCG_DuelSetResponse,
    OCG_LoadScript,
    OCG_DuelQueryCount,
    OCG_DuelQuery,
    OCG_DuelQueryLocation,
    OCG_DuelQueryField,
};
use crate::{Player, OCGDuelError};

pub struct DuelBuilder {
    card_handler: Box<dyn FnMut(u32, *mut OCG_CardData) + 'static>,
    script_handler: Box<dyn FnMut(&Duel, &str) -> i32 + 'static>,
    script_handler_wrapper: Box<dyn FnMut(*mut c_void, &str) -> i32 + 'static>,
    log_handler: Option<Box<dyn FnMut(&str, i32) + 'static>>,
    card_read_done_handler: Option<Box<dyn FnMut(*mut OCG_CardData) + 'static>>,
    seed: [u64; 4],
    flags: u64,
    team_1: Player,
    team_2: Player,
    enable_unsafe_libraries: bool,
}

impl Default for DuelBuilder {
    fn default() -> DuelBuilder {
        DuelBuilder {
            card_handler: Box::new(|_, _| ()),
            script_handler: Box::new(|_, _| 0),
            script_handler_wrapper: Box::new(|_, _| 0),
            log_handler: None,
            card_read_done_handler: None,
            seed: [0; 4],
            flags: 0,
            team_1: Player::default(),
            team_2: Player::default(),
            enable_unsafe_libraries: true,
        }
    }
}

impl DuelBuilder {
    pub fn set_card_handler<F>(&mut self, callback: F)
    where
        F: FnMut(u32, *mut OCG_CardData),
        F: 'static,
    {
        self.card_handler = Box::new(callback);
    }
    pub fn set_script_handler<F>(&mut self, callback: F)
    where
        F: FnMut(&Duel, &str) -> i32,
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
    extern "C" fn card_handler_ffi(cb: *mut c_void, code: u32, data: *mut OCG_CardData) {
        let closure = unsafe { &mut *(cb as *mut Box<dyn FnMut(u32, *mut OCG_CardData)>) };
        closure(code, data)
    }
    extern "C" fn script_handler_ffi(
        cb: *mut c_void,
        duel_ptr: *mut c_void,
        name: *const i8,
    ) -> i32 {
        let nameStr = unsafe { CStr::from_ptr(name) };
        let closure = unsafe {
            &mut *(cb as *mut Box<dyn for<'a> FnMut(*mut c_void, &'a str) -> i32>)
        };
        closure(duel_ptr, nameStr.to_str().unwrap())
    }
    extern "C" fn log_handler_ffi(cb: *mut c_void, msg: *const i8, msg_type: i32) {
        let msgStr = unsafe { CStr::from_ptr(msg) };
        let closure = unsafe { &mut *(cb as *mut Box<dyn for<'a> FnMut(&'a str, i32)>) };
        closure(msgStr.to_str().unwrap(), msg_type)
    }
    extern "C" fn card_read_done_handler_ffi(cb: *mut c_void, data: *mut OCG_CardData) {
        let closure = unsafe { &mut *(cb as *mut Box<dyn FnMut(*mut OCG_CardData)>) };
        closure(data)
    }
    pub fn build(mut self) -> Duel {
        let mut duel = Duel {
            ptr: null_mut(),
        };
        // This needs to be done so that the script_handler is able to access the duel instance
        // We can assume that the duel instance will be valid for the lifetime of the script_handler,
        // as this is only called on OCGDuelInstance::new_card and OCG_CreateDuel.
        // Because this is called on OCG_CreateDuel, we need to set the pointer here,
        // so that the instance will have the correct pointer when OCG_CreateDuel is called.
        // `duel` should never go out of scope during construction.
        // If it does, this would become a memory leak, as the pointer would be lost before it is properly set in OCG_CreateDuel.
        let mut_ptr = &mut duel as *mut Duel;
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
                .map_or(null_mut(), |cb| Box::into_raw(cb) as *mut _),
            cardReaderDone: self
                .card_read_done_handler
                .as_ref()
                .and(Some(Self::card_read_done_handler_ffi)),
            payload4: self
                .card_read_done_handler
                .map_or(null_mut(), |cb| Box::into_raw(cb) as *mut _),
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
pub enum DuelStatus {
    End = OCG_DuelStatus_OCG_DUEL_STATUS_END as isize,
    Awaiting = OCG_DuelStatus_OCG_DUEL_STATUS_AWAITING as isize,
    Continue = OCG_DuelStatus_OCG_DUEL_STATUS_CONTINUE as isize,
}

impl From<OCG_DuelStatus> for DuelStatus {
    fn from(status: OCG_DuelStatus) -> Self {
        match status {
            OCG_DuelStatus_OCG_DUEL_STATUS_END => DuelStatus::End,
            OCG_DuelStatus_OCG_DUEL_STATUS_AWAITING => DuelStatus::Awaiting,
            OCG_DuelStatus_OCG_DUEL_STATUS_CONTINUE => DuelStatus::Continue,
            _ => panic!("Invalid OCG_DuelStatus"),
        }
    }
}

pub struct Duel {
    ptr: *mut c_void,
}

impl Drop for Duel {
    fn drop(&mut self) {
        unsafe {
            OCG_DestroyDuel(self.ptr);
        }
    }
}

impl Duel {
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
    /// Start the duel simulation and state machine.
    /// Call this after all options and cards for the duel have been loaded.
    pub fn start(&self) {
        unsafe {
            OCG_StartDuel(self.ptr);
        }
    }
    pub fn destroy(self) {
        drop(self);
    }
    // Processing
    /// Runs the state machine to start the duel or after a waiting state requiring a player response.
    pub fn process(&self) -> DuelStatus {
        unsafe { (OCG_DuelProcess(self.ptr) as OCG_DuelStatus).into() }
    }
    /// The main interface to the simulation.
    /// Returns a copy of the internal buffer containing all messages from the duel simulation.
    /// Subsequent calls invalidate previous buffers.
    pub fn get_message(&self) -> Vec<u8> {
        let mut length: u32 = 0;
        let mut ptr = unsafe { OCG_DuelGetMessage(self.ptr, &mut length) as *const u8 };
        let mut message_vec: Vec<u8> = Vec::with_capacity(length as usize);
        let end_rounded_up = ptr.wrapping_offset(length as isize);
        while ptr != end_rounded_up {
            unsafe {
                message_vec.push(*ptr);
            }
            ptr = ptr.wrapping_offset(1);
        }
        message_vec
    }
    /// Sets the next player response for the duel simulation.
    /// Subsequent calls overwrite previous responses if [`process`](#method.process) has not been called after.
    /// The contents of the provided buffer are copied internally.
    pub fn set_response(&self, response: &[u8]) {
        unsafe {
            OCG_DuelSetResponse(
                self.ptr,
                response.as_ptr() as *const _,
                response.len() as u32,
            );
        }
    }
    /// Load a Lua card script or supporting script for the specified duel.
    /// Generally you do not call this directly except to load global scripts;
    /// instead you want to call this from your handler provided to [`set_script_handler`](struct.DuelBuilder.html#method.set_script_handler).
    pub fn load_script(
        &self,
        src_code: &str,
        name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let src_code = CString::new(src_code)?;
        let name_cstr = CString::new(name)?;
        let result = unsafe {
            OCG_LoadScript(
                self.ptr,
                src_code.as_ptr(),
                src_code.to_bytes().len() as u32,
                name_cstr.as_ptr(),
            )
        };
        if result == 0 {
            return Err(Box::new(OCGDuelError::ScriptLoadFailure(name.to_owned())));
        }
        Ok(())
    }
    // Querying
    /// Returns the number of cards in the specified zone.
    pub fn query_count(&self, player: u8, location: u32) -> u32 {
        unsafe { OCG_DuelQueryCount(self.ptr, player, location) }
    }
    pub fn query(&self, query_info: OCG_QueryInfo) -> Vec<u8> {
        let mut length: u32 = 0;
        let mut ptr = unsafe { OCG_DuelQuery(self.ptr, &mut length, query_info) as *const u8 };
        let mut result_vec: Vec<u8> = Vec::with_capacity(length as usize);
        let end_rounded_up = ptr.wrapping_offset(length as isize);
        while ptr != end_rounded_up {
            unsafe {
                result_vec.push(*ptr);
            }
            ptr = ptr.wrapping_offset(1);
        }
        result_vec
    }
    pub fn query_location(&self, query_info: OCG_QueryInfo) -> Vec<u8> {
        let mut length: u32 = 0;
        let mut ptr =
            unsafe { OCG_DuelQueryLocation(self.ptr, &mut length, query_info) as *const u8 };
        let mut result_vec: Vec<u8> = Vec::with_capacity(length as usize);
        let end_rounded_up = ptr.wrapping_offset(length as isize);
        while ptr != end_rounded_up {
            unsafe {
                result_vec.push(*ptr);
            }
            ptr = ptr.wrapping_offset(1);
        }
        result_vec
    }
    pub fn query_field(&self) -> Vec<u8> {
        let mut length: u32 = 0;
        let mut ptr = unsafe { OCG_DuelQueryField(self.ptr, &mut length) as *const u8 };
        let mut result_vec: Vec<u8> = Vec::with_capacity(length as usize);
        let end_rounded_up = ptr.wrapping_offset(length as isize);
        while ptr != end_rounded_up {
            unsafe {
                result_vec.push(*ptr);
            }
            ptr = ptr.wrapping_offset(1);
        }
        result_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        let version = Duel::get_version();
        assert!(version[0] == 10);
        assert!(version[1] == 0);
    }
    #[test]
    fn test_create_duel() {
        let duel_builder = DuelBuilder::default();
        let duel = duel_builder.build();
        assert!(!duel.ptr.is_null());
    }
    #[test]
    fn test_start_duel() {
        let duel_builder = DuelBuilder::default();
        let duel = duel_builder.build();
        assert!(!duel.ptr.is_null());
        duel.start();
    }
    #[test]
    fn test_process_duel() {
        let duel_builder = DuelBuilder::default();
        let duel = duel_builder.build();
        assert!(!duel.ptr.is_null());
        duel.start();
        duel.process();
    }
    #[test]
    fn test_get_message_duel() {
        let duel_builder = DuelBuilder::default();
        let duel = duel_builder.build();
        assert!(!duel.ptr.is_null());
        duel.start();
        duel.process();
        println!("{:?}", duel.get_message());
    }
    #[test]
    fn test_load_script_duel() {
        let duel_builder = DuelBuilder::default();
        let duel = duel_builder.build();
        assert!(!duel.ptr.is_null());
        assert!(duel
            .load_script("invalid script", "invalid_script")
            .is_err());
    }
}
