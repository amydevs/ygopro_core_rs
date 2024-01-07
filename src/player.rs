use crate::ffi::OCG_Player;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    starting_lp: u32,
    starting_draw_count: u32,
    draw_count_per_turn: u32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            starting_lp: 8000,
            starting_draw_count: 5,
            draw_count_per_turn: 1,
        }
    }
}

impl From<Player> for OCG_Player {
    fn from(val: Player) -> Self {
        OCG_Player {
            startingLP: val.starting_lp,
            startingDrawCount: val.starting_draw_count,
            drawCountPerTurn: val.draw_count_per_turn,
        }
    }
}
