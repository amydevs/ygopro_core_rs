#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ygopro_core_rs_sys::*;

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
    pub fn create_duel(options: OCG_DuelOptions) -> OCGDuelInstance {
        let mut pduel: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        unsafe {
            OCG_CreateDuel(&mut pduel, options);
        }
        return OCGDuelInstance {
            ptr: pduel
        }
    }
    pub fn destroy_duel(&self) {
        drop(self);
    }
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
        let player_a = OCG_Player {
            startingLP: 8000,
            startingDrawCount: 1,
            drawCountPerTurn: 1
        };
        let player_b = OCG_Player {
            startingLP: 8000,
            startingDrawCount: 1,
            drawCountPerTurn: 1
        };
        let options = OCG_DuelOptions {
            cardReader: None,
            payload1: std::ptr::null_mut(),
            scriptReader: None,
            payload2: std::ptr::null_mut(),
            logHandler: None,
            payload3: std::ptr::null_mut(),
            cardReaderDone: None,
            payload4: std::ptr::null_mut(),
            seed: [0; 4],
            flags: 0,
            team1: player_a,
            team2: player_b,
            enableUnsafeLibraries: 1
        };
        OCGDuelInstance::create_duel(options);
    }
}