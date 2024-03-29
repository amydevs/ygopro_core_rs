use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr::null_mut;

use crate::ffi::{
    OCG_CardData, OCG_CreateDuel, OCG_DestroyDuel,
    OCG_DuelCreationStatus_OCG_DUEL_CREATION_NOT_CREATED,
    OCG_DuelCreationStatus_OCG_DUEL_CREATION_NO_OUTPUT,
    OCG_DuelCreationStatus_OCG_DUEL_CREATION_NULL_DATA_READER,
    OCG_DuelCreationStatus_OCG_DUEL_CREATION_NULL_SCRIPT_READER,
    OCG_DuelCreationStatus_OCG_DUEL_CREATION_SUCCESS, OCG_DuelGetMessage, OCG_DuelNewCard,
    OCG_DuelOptions, OCG_DuelProcess, OCG_DuelQuery, OCG_DuelQueryCount, OCG_DuelQueryField,
    OCG_DuelQueryLocation, OCG_DuelSetResponse, OCG_DuelStatus,
    OCG_DuelStatus_OCG_DUEL_STATUS_AWAITING, OCG_DuelStatus_OCG_DUEL_STATUS_CONTINUE,
    OCG_DuelStatus_OCG_DUEL_STATUS_END, OCG_GetVersion, OCG_LoadScript, OCG_StartDuel,
};

use crate::card::{CardData, NewCardInfo};
use crate::error::DuelError;
use crate::player::Player;
use crate::QueryInfo;

pub trait CardHandler: FnMut(u32) -> CardData + 'static {}
impl<T: FnMut(u32) -> CardData + 'static> CardHandler for T {}
pub trait ScriptHandler:
    FnMut(&Duel, &str) -> Result<(), Box<dyn std::error::Error>> + 'static
{
}
impl<T: FnMut(&Duel, &str) -> Result<(), Box<dyn std::error::Error>> + 'static> ScriptHandler
    for T
{
}
trait ScriptHandlerWrapper: FnMut(*mut c_void, &str) -> i32 + 'static {}
impl<T: FnMut(*mut c_void, &str) -> i32 + 'static> ScriptHandlerWrapper for T {}
pub trait LogHandler: FnMut(&str, i32) + 'static {}
impl<T: FnMut(&str, i32) + 'static> LogHandler for T {}
pub trait CardReadDoneHandler: FnMut(&CardData) + 'static {}
impl<T: FnMut(&CardData) + 'static> CardReadDoneHandler for T {}

pub struct DuelBuilder {
    card_handler: Box<dyn CardHandler>,
    script_handler: Box<dyn ScriptHandler>,
    script_handler_wrapper: Box<dyn ScriptHandlerWrapper>,
    log_handler: Box<dyn LogHandler>,
    card_read_done_handler: Box<dyn CardReadDoneHandler>,
    seed: [u64; 4],
    flags: u64,
    team_1: Player,
    team_2: Player,
    enable_unsafe_libraries: bool,
}

impl Default for DuelBuilder {
    fn default() -> DuelBuilder {
        DuelBuilder {
            card_handler: Box::new(|code| CardData {
                code,
                ..Default::default()
            }),
            script_handler: Box::new(|_, _| Err("No script handler set!")?),
            script_handler_wrapper: Box::new(|_, _| 0),
            log_handler: Box::new(|_, _| ()),
            card_read_done_handler: Box::new(|_| ()),
            seed: [0; 4],
            flags: 0,
            team_1: Player::default(),
            team_2: Player::default(),
            enable_unsafe_libraries: true,
        }
    }
}

impl DuelBuilder {
    pub fn new() -> DuelBuilder {
        DuelBuilder::default()
    }
    /// Sets the card handler for the duel.
    /// By default, the handler creates an empty card with the supplied code from the callback.
    pub fn set_card_handler<F>(&mut self, callback: F)
    where
        F: CardHandler,
        F: 'static,
    {
        self.card_handler = Box::new(callback);
    }
    /// Sets the script handler for the duel.
    /// By default, the handler returns an error to indicated that the script handler has not been set.
    pub fn set_script_handler<F>(&mut self, callback: F)
    where
        F: ScriptHandler,
        F: 'static,
    {
        self.script_handler = Box::new(callback);
    }
    /// Sets the log handler for the duel.
    /// By default, the handler is no-op.
    /// You most likely want to set this to something that will use your logging framework of choice.
    pub fn set_log_handler<F>(&mut self, callback: F)
    where
        F: LogHandler,
        F: 'static,
    {
        self.log_handler = Box::new(callback);
    }
    /// Sets the card read done handler for the duel.
    /// By default, the handler is no-op.
    ///
    /// The cardReaderDone callback is usually used for deallocating the setcodes buffer,
    /// but this will always be done implicitly outside of your provided closure,
    /// so don't worry about it.
    pub fn set_card_read_done_handler<F>(&mut self, callback: F)
    where
        F: CardReadDoneHandler,
        F: 'static,
    {
        self.card_read_done_handler = Box::new(callback);
    }
    extern "C" fn card_handler_ffi(cb: *mut c_void, code: u32, data: *mut OCG_CardData) {
        let closure = unsafe { &mut *(cb as *mut Box<dyn CardHandler>) };
        let card_data: CardData = closure(code);
        unsafe { data.write(card_data.into_ocg_carddata_leaky()) };
    }
    extern "C" fn script_handler_ffi(
        cb: *mut c_void,
        duel_ptr: *mut c_void,
        name: *const i8,
    ) -> i32 {
        let name_str = unsafe { CStr::from_ptr(name) };
        let closure = unsafe { &mut *(cb as *mut Box<dyn ScriptHandlerWrapper>) };
        closure(duel_ptr, name_str.to_str().unwrap())
    }
    extern "C" fn log_handler_ffi(cb: *mut c_void, msg: *const i8, msg_type: i32) {
        let msg_str = unsafe { CStr::from_ptr(msg) };
        let closure = unsafe { &mut *(cb as *mut Box<dyn LogHandler>) };
        closure(msg_str.to_str().unwrap(), msg_type)
    }
    extern "C" fn card_read_done_handler_ffi(cb: *mut c_void, data: *mut OCG_CardData) {
        let closure = unsafe { &mut *(cb as *mut Box<dyn CardReadDoneHandler>) };
        let card_data: CardData = unsafe { data.read().into() };
        // deallocate setcodes once it has been copied to card_data
        let setcode_ptr = unsafe { data.read().setcodes };
        if !setcode_ptr.is_null() {
            let mut iterate_setcode_ptr = setcode_ptr;
            let mut len = 0;
            loop {
                len += 1;
                unsafe {
                    if (*iterate_setcode_ptr) == 0 {
                        break;
                    }
                }
                iterate_setcode_ptr = iterate_setcode_ptr.wrapping_offset(1);
            }
            // Cast into a Vec to bring it into the borrow checked world, dropping it immediately.
            // Capacity SHOULD be the length of the array, we obviously can't guarantee it.
            // But we can assume it is, as the vector capacity is constructed with the length of the array in card.rs.
            drop(unsafe { Vec::from_raw_parts(setcode_ptr, len, len) });
        }
        closure(&card_data)
    }
    pub fn set_seed(&mut self, seed: [u64; 4]) {
        self.seed = seed;
    }
    pub fn set_flags(&mut self, flags: u64) {
        self.flags = flags;
    }
    pub fn set_team_1(&mut self, player: Player) {
        self.team_1 = player;
    }
    pub fn set_team_2(&mut self, player: Player) {
        self.team_2 = player;
    }
    pub fn set_enable_unsafe_libraries(&mut self, enable: bool) {
        self.enable_unsafe_libraries = enable;
    }
    pub fn build(mut self) -> Duel {
        let mut duel = Duel { ptr: null_mut() };
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
                    return (self.script_handler)(mut_ptr_mut, name).is_ok() as i32;
                }
            }
            unsafe {
                let mut_ptr_ref = mut_ptr.as_ref().unwrap();
                (self.script_handler)(mut_ptr_ref, name).is_ok() as i32
            }
        });
        // Double indirection is required for the callback pointers
        let options = OCG_DuelOptions {
            cardReader: Some(Self::card_handler_ffi),
            payload1: Box::into_raw(Box::new(self.card_handler)) as *mut _,
            scriptReader: Some(Self::script_handler_ffi),
            payload2: Box::into_raw(Box::new(self.script_handler_wrapper)) as *mut _,
            logHandler: Some(Self::log_handler_ffi),
            payload3: Box::into_raw(Box::new(self.log_handler)) as *mut _,
            cardReaderDone: Some(Self::card_read_done_handler_ffi),
            payload4: Box::into_raw(Box::new(self.card_read_done_handler)) as *mut _,
            seed: self.seed,
            flags: self.flags,
            team1: self.team_1.into(),
            team2: self.team_2.into(),
            enableUnsafeLibraries: self.enable_unsafe_libraries.into(),
        };
        let res_code: i32 = unsafe { OCG_CreateDuel(&mut duel.ptr, options) };
        #[allow(non_upper_case_globals)]
        match res_code.try_into() {
            Ok(OCG_DuelCreationStatus_OCG_DUEL_CREATION_SUCCESS) => duel,
            // These should never happen if types are abided by, so we panic.
            Ok(OCG_DuelCreationStatus_OCG_DUEL_CREATION_NO_OUTPUT) => {
                panic!("Failed to create duel: OCG_DUEL_CREATION_NO_OUTPUT")
            }
            Ok(OCG_DuelCreationStatus_OCG_DUEL_CREATION_NOT_CREATED) => {
                panic!("Failed to create duel: OCG_DUEL_CREATION_NOT_CREATED")
            }
            Ok(OCG_DuelCreationStatus_OCG_DUEL_CREATION_NULL_DATA_READER) => {
                panic!("Failed to create duel: OCG_DUEL_CREATION_NULL_DATA_READER")
            }
            Ok(OCG_DuelCreationStatus_OCG_DUEL_CREATION_NULL_SCRIPT_READER) => {
                panic!("Failed to create duel: OCG_DUEL_CREATION_NULL_SCRIPT_READER")
            }
            _ => panic!("Failed to create duel to to unknown error"),
        }
    }
}

#[derive(Debug)]
pub enum DuelStatus {
    /// Duel ended
    End = OCG_DuelStatus_OCG_DUEL_STATUS_END as isize,
    /// Player response required
    Awaiting = OCG_DuelStatus_OCG_DUEL_STATUS_AWAITING as isize,
    Continue = OCG_DuelStatus_OCG_DUEL_STATUS_CONTINUE as isize,
}

impl From<OCG_DuelStatus> for DuelStatus {
    fn from(status: OCG_DuelStatus) -> Self {
        #![allow(non_upper_case_globals)]
        match status {
            OCG_DuelStatus_OCG_DUEL_STATUS_END => DuelStatus::End,
            OCG_DuelStatus_OCG_DUEL_STATUS_AWAITING => DuelStatus::Awaiting,
            OCG_DuelStatus_OCG_DUEL_STATUS_CONTINUE => DuelStatus::Continue,
            _ => panic!(
                "Invalid OCG_DuelStatus, this should never happen! Please report this as a bug!"
            ),
        }
    }
}

/// A Duel instance.
///
/// The Duel will be destroyed when it goes out of scope.
/// Make sure that the lifetime of this duel matches whatever you want to use it for.
///
/// Construction of this struct MUST always handled by the DuelBuilder.
/// Otherwise, memory leaks could happen!
#[derive(Debug)]
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
    /// Returns the raw pointer to the duel instance.
    ///
    /// For the sake of simplicity,
    /// the lifetime of the value at the pointer
    /// is tied to the lifetime of the [`Duel`].
    ///
    /// DO NOT CREATE A [`Duel`] FROM THIS POINTER,
    /// IT WILL CAUSE TO DOUBLE FREE ERRORS.
    pub fn get_raw_ptr(&self) -> *mut c_void {
        self.ptr
    }
    // Lifecycle
    /// Add the card specified by info to the duel.
    /// This calls the provided card_handler info.code and script_handler if the card script has not been loaded yet.
    pub fn new_card(&self, info: NewCardInfo) {
        unsafe {
            OCG_DuelNewCard(self.ptr, info.into());
        }
    }
    /// Start the duel simulation and state machine.
    /// Call this after all options and cards for the duel have been loaded.
    pub fn start(&self) {
        unsafe {
            OCG_StartDuel(self.ptr);
        }
    }
    /// Simply calls drop on self.
    /// This will in turn call [`OCG_DestroyDuel`] on the internal pointer,
    /// deallocating the internal duel.
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
    pub fn load_script(&self, src_code: &str, name: &str) -> Result<(), DuelError> {
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
            return Err(DuelError::ScriptLoadFailure(name.to_owned()));
        }
        Ok(())
    }
    // Querying
    /// Returns the number of cards in the specified zone.
    pub fn query_count(&self, team: u8, loc: u32) -> u32 {
        unsafe { OCG_DuelQueryCount(self.ptr, team, loc) }
    }
    /// Returns a copy of an internal buffer for the FIRST card matching the query.
    /// Subsequent calls invalidate previous queries.
    pub fn query(&self, query_info: QueryInfo) -> Option<Vec<u8>> {
        let mut length: u32 = 0;
        let mut ptr =
            unsafe { OCG_DuelQuery(self.ptr, &mut length, query_info.into()) as *const u8 };
        if ptr.is_null() {
            return None;
        }
        let mut result_vec: Vec<u8> = Vec::with_capacity(length as usize);
        let end_rounded_up = ptr.wrapping_offset(length as isize);
        while ptr != end_rounded_up {
            unsafe {
                result_vec.push(*ptr);
            }
            ptr = ptr.wrapping_offset(1);
        }
        Some(result_vec)
    }
    /// Returns a copy of an internal buffer for the ALL cards matching the query.
    /// Subsequent calls invalidate previous queries.
    pub fn query_location(&self, query_info: QueryInfo) -> Option<Vec<u8>> {
        let mut length: u32 = 0;
        let mut ptr =
            unsafe { OCG_DuelQueryLocation(self.ptr, &mut length, query_info.into()) as *const u8 };
        if ptr.is_null() {
            return None;
        }
        let mut result_vec: Vec<u8> = Vec::with_capacity(length as usize);
        let end_rounded_up = ptr.wrapping_offset(length as isize);
        while ptr != end_rounded_up {
            unsafe {
                result_vec.push(*ptr);
            }
            ptr = ptr.wrapping_offset(1);
        }
        Some(result_vec)
    }
    /// Returns a copy of an internal buffer containing card counts for every zone in the game.
    /// Subsequent calls invalidate previous queries.
    pub fn query_field(&self) -> Option<Vec<u8>> {
        let mut length: u32 = 0;
        let mut ptr = unsafe { OCG_DuelQueryField(self.ptr, &mut length) as *const u8 };
        if ptr.is_null() {
            return None;
        }
        let mut result_vec: Vec<u8> = Vec::with_capacity(length as usize);
        let end_rounded_up = ptr.wrapping_offset(length as isize);
        while ptr != end_rounded_up {
            unsafe {
                result_vec.push(*ptr);
            }
            ptr = ptr.wrapping_offset(1);
        }
        Some(result_vec)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{hash_map::RandomState, HashSet};

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
        assert!(!duel.get_message().is_empty());
    }
    #[test]
    fn test_load_script_duel() {
        let mut duel_builder = DuelBuilder::default();
        duel_builder.set_log_handler(|msg, _| {
            println!("{}", msg);
        });
        let duel = duel_builder.build();
        assert!(!duel.ptr.is_null());
        assert!(duel
            .load_script("invalid script", "invalid_script")
            .is_err());
        assert!(duel.load_script("Debug.ReloadFieldEnd()", " ").is_ok());
        assert!(duel
            .load_script("CARD_CYBER_DRAGON = 70095154\nEFFECT_TYPE_FIELD = 0x2\nEFFECT_TYPE_CONTINUOUS = 0x800\nEVENT_STARTUP = 1000", "constants.lua")
            .is_ok());
        assert!(duel
            .load_script(include_str!("../assets/c511005093.lua"), "c511005093.lua")
            .is_ok());
    }
    #[test]
    fn test_new_card_duel() {
        let mut duel_builder = DuelBuilder::default();
        let setcodes: HashSet<u16, RandomState> = HashSet::from_iter([1u16]);
        let card_data = CardData {
            code: 100,
            alias: 1,
            setcodes: setcodes.clone(),
            card_type: 1,
            level: 1,
            attribute: 1,
            race: 1,
            attack: 1,
            defense: 1,
            lscale: 1,
            rscale: 1,
            link_marker: 1,
        };
        let mut card_data_clone = card_data.clone();
        duel_builder.set_card_handler(move |code| {
            card_data_clone.code = code;
            card_data_clone.clone()
        });
        duel_builder.set_card_read_done_handler(move |card| {
            assert!(card.code == card_data.code);
            assert!(card.alias == card_data.alias);
            assert!(card.setcodes.len() == card_data.setcodes.len());
            for setcode in card.setcodes.iter() {
                assert!(&setcodes.contains(setcode))
            }
            assert!(card.card_type == card_data.card_type);
            assert!(card.level == card_data.level);
            assert!(card.attribute == card_data.attribute);
            assert!(card.race == card_data.race);
            assert!(card.attack == card_data.attack);
            assert!(card.defense == card_data.defense);
            assert!(card.lscale == card_data.lscale);
            assert!(card.rscale == card_data.rscale);
            assert!(card.link_marker == card_data.link_marker);
        });
        let duel = duel_builder.build();
        duel.new_card(NewCardInfo {
            team: 1,
            duelist: 1,
            code: card_data.code,
            con: 1,
            loc: 1,
            seq: 1,
            pos: 1,
        });
    }
    #[test]
    fn test_query_count_duel() {
        let mut duel_builder = DuelBuilder::default();
        duel_builder.set_card_handler(|code| CardData {
            code,
            ..Default::default()
        });
        let duel = duel_builder.build();
        assert!(duel.query_count(1, 0x02) == 0);
        // TODO: still need to test this further
    }
    #[test]
    fn test_query_field_duel() {
        let mut duel_builder = DuelBuilder::default();
        duel_builder.set_card_handler(|code| CardData {
            code,
            ..Default::default()
        });
        let duel = duel_builder.build();
        assert!(duel.query_field().is_some());
        // TODO: still need to test this further
    }
    #[test]
    fn test_query_location_duel() {
        let duel_builder = DuelBuilder::default();
        let duel = duel_builder.build();
        assert!(duel.query_location(QueryInfo::default()).is_some());
        // TODO: still need to test this further
    }
    #[test]
    fn test_query_duel() {
        let duel_builder = DuelBuilder::default();
        let duel = duel_builder.build();
        assert!(duel.query(QueryInfo::default()).is_none());
        // TODO: still need to test this further
    }
}
