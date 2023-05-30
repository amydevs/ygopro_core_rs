#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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
    pub fn create_duel(options: OCG_DuelOptions) -> OCGDuelInstance {
        let mut pduel: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        unsafe {
            OCG_CreateDuel(&mut pduel, options);
        }
        return OCGDuelInstance {
            ptr: pduel
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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