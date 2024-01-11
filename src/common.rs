use bitflags::bitflags;
// Locations
pub const LOCATION_DECK: u32 = 0x01;
pub const LOCATION_HAND: u32 = 0x02;
pub const LOCATION_MZONE: u32 = 0x04;
pub const LOCATION_SZONE: u32 = 0x08;
pub const LOCATION_GRAVE: u32 = 0x10;
pub const LOCATION_REMOVED: u32 = 0x20;
pub const LOCATION_EXTRA: u32 = 0x40;
pub const LOCATION_OVERLAY: u32 = 0x80;
pub const LOCATION_ONFIELD: u32 = 0x0c;
pub const LOCATION_FZONE: u32 = 0x100;
pub const LOCATION_PZONE: u32 = 0x200;
pub const LOCATION_ALL: u32 = 0x3ff;
// For Redirect
/// Return to deck bottom
pub const LOCATION_DECKBOT: u32 = 0x10001; 
/// Return to deck and shuffle
pub const LOCATION_DECKSHF: u32 = 0x20001;
bitflags! {
    pub struct Flags: u32 {
        const Deck = LOCATION_DECK;
        const Hand = LOCATION_HAND;
        const MZone = LOCATION_MZONE;
        const SZone = LOCATION_SZONE;
        const Grave = LOCATION_GRAVE;
        const Removed = LOCATION_REMOVED;
        const Extra = LOCATION_EXTRA;
        const Overlay = LOCATION_OVERLAY;
        const OnField = LOCATION_ONFIELD;
        const FZone = LOCATION_FZONE;
        const PZone = LOCATION_PZONE;
        const All = LOCATION_ALL;
        const DeckBot = LOCATION_DECKBOT;
        const DeckShf = LOCATION_DECKSHF;
    }
}

// Positions
pub const POS_FACEUP_ATTACK: u8 = 0x1;
pub const POS_FACEDOWN_ATTACK: u8 = 0x2;
pub const POS_FACEUP_DEFENSE: u8 = 0x4;
pub const POS_FACEDOWN_DEFENSE: u8 = 0x8;
pub const POS_FACEUP: u8 = 0x5;
pub const POS_FACEDOWN: u8 = 0xa;
pub const POS_ATTACK: u8 = 0x3;
pub const POS_DEFENSE: u8 = 0xc;
#[repr(u8)]
pub enum Position {
    FaceupAttack = POS_FACEUP_ATTACK,
    FacedownAttack = POS_FACEDOWN_ATTACK,
    FaceupDefense = POS_FACEUP_DEFENSE,
    FacedownDefense = POS_FACEDOWN_DEFENSE,
    Faceup = POS_FACEUP,
    Facedown = POS_FACEDOWN,
    Attack = POS_ATTACK,
    Defense = POS_DEFENSE,
}

// Flip effect flags
pub const NO_FLIP_EFFECT: u32 = 0x10000;

// Types of Cards
pub const TYPE_MONSTER: u32 = 0x1;
pub const TYPE_SPELL: u32 = 0x2;
pub const TYPE_TRAP: u32 = 0x4;
pub const TYPE_NORMAL: u32 = 0x10;
pub const TYPE_EFFECT: u32 = 0x20;
pub const TYPE_FUSION: u32 = 0x40;
pub const TYPE_RITUAL: u32 = 0x80;
pub const TYPE_TRAPMONSTER: u32 = 0x100;
pub const TYPE_SPIRIT: u32 = 0x200;
pub const TYPE_UNION: u32 = 0x400;
pub const TYPE_GEMINI: u32 = 0x800;
pub const TYPE_TUNER: u32 = 0x1000;
pub const TYPE_SYNCHRO: u32 = 0x2000;
pub const TYPE_TOKEN: u32 = 0x4000;
pub const TYPE_MAXIMUM: u32 = 0x8000;
pub const TYPE_QUICKPLAY: u32 = 0x10000;
pub const TYPE_CONTINUOUS: u32 = 0x20000;
pub const TYPE_EQUIP: u32 = 0x40000;
pub const TYPE_FIELD: u32 = 0x80000;
pub const TYPE_COUNTER: u32 = 0x100000;
pub const TYPE_FLIP: u32 = 0x200000;
pub const TYPE_TOON: u32 = 0x400000;
pub const TYPE_XYZ: u32 = 0x800000;
pub const TYPE_PENDULUM: u32 = 0x1000000;
pub const TYPE_SPSUMMON: u32 = 0x2000000;
pub const TYPE_LINK: u32 = 0x4000000;
bitflags! {
    pub struct Type: u32 {
        const Monster = TYPE_MONSTER;
        const Spell = TYPE_SPELL;
        const Trap = TYPE_TRAP;
        const Normal = TYPE_NORMAL;
        const Effect = TYPE_EFFECT;
        const Fusion = TYPE_FUSION;
        const Ritual = TYPE_RITUAL;
        const TrapMonster = TYPE_TRAPMONSTER;
        const Spirit = TYPE_SPIRIT;
        const Union = TYPE_UNION;
        const Gemini = TYPE_GEMINI;
        const Tuner = TYPE_TUNER;
        const Synchro = TYPE_SYNCHRO;
        const Token = TYPE_TOKEN;
        const Maximum = TYPE_MAXIMUM;
        const QuickPlay = TYPE_QUICKPLAY;
        const Continuous = TYPE_CONTINUOUS;
        const Equip = TYPE_EQUIP;
        const Field = TYPE_FIELD;
        const Counter = TYPE_COUNTER;
        const Flip = TYPE_FLIP;
        const Toon = TYPE_TOON;
        const Xyz = TYPE_XYZ;
        const Pendulum = TYPE_PENDULUM;
        const SpSummon = TYPE_SPSUMMON;
        const Link = TYPE_LINK;
    }
}

// Attributes
pub const ATTRIBUTE_EARTH: u32 = 0x01;
pub const ATTRIBUTE_WATER: u32 = 0x02;
pub const ATTRIBUTE_FIRE: u32 = 0x04;
pub const ATTRIBUTE_WIND: u32 = 0x08;
pub const ATTRIBUTE_LIGHT: u32 = 0x10;
pub const ATTRIBUTE_DARK: u32 = 0x20;
pub const ATTRIBUTE_DIVINE: u32 = 0x40;
#[repr(u32)]
pub enum Attribute {
    Earth = ATTRIBUTE_EARTH,
    Water = ATTRIBUTE_WATER,
    Fire = ATTRIBUTE_FIRE,
    Wind = ATTRIBUTE_WIND,
    Light = ATTRIBUTE_LIGHT,
    Dark = ATTRIBUTE_DARK,
    Divine = ATTRIBUTE_DIVINE,
}

// Races
pub const RACE_WARRIOR: u32 = 0x1;
pub const RACE_SPELLCASTER: u32 = 0x2;
pub const RACE_FAIRY: u32 = 0x4;
pub const RACE_FIEND: u32 = 0x8;
pub const RACE_ZOMBIE: u32 = 0x10;
pub const RACE_MACHINE: u32 = 0x20;
pub const RACE_AQUA: u32 = 0x40;
pub const RACE_PYRO: u32 = 0x80;
pub const RACE_ROCK: u32 = 0x100;
pub const RACE_WINGEDBEAST: u32 = 0x200;
pub const RACE_PLANT: u32 = 0x400;
pub const RACE_INSECT: u32 = 0x800;
pub const RACE_THUNDER: u32 = 0x1000;
pub const RACE_DRAGON: u32 = 0x2000;
pub const RACE_BEAST: u32 = 0x4000;
pub const RACE_BEASTWARRIOR: u32 = 0x8000;
pub const RACE_DINOSAUR: u32 = 0x10000;
pub const RACE_FISH: u32 = 0x20000;
pub const RACE_SEASERPENT: u32 = 0x40000;
pub const RACE_REPTILE: u32 = 0x80000;
pub const RACE_PSYCHIC: u32 = 0x100000;
pub const RACE_DIVINE: u32 = 0x200000;
pub const RACE_CREATORGOD: u32 = 0x400000;
pub const RACE_WYRM: u32 = 0x800000;
pub const RACE_CYBERSE: u32 = 0x1000000;
pub const RACE_CYBORG: u32 = 0x2000000;
pub const RACE_MAX: u32 = RACE_CYBORG;

// Reasons
pub const REASON_DESTROY: u32 = 0x1;
pub const REASON_RELEASE: u32 = 0x2;
pub const REASON_TEMPORARY: u32 = 0x4;
pub const REASON_MATERIAL: u32 = 0x8;
pub const REASON_SUMMON: u32 = 0x10;
pub const REASON_BATTLE: u32 = 0x20;
pub const REASON_EFFECT: u32 = 0x40;
pub const REASON_COST: u32 = 0x80;
pub const REASON_ADJUST: u32 = 0x100;
pub const REASON_LOST_TARGET: u32 = 0x200;
pub const REASON_RULE: u32 = 0x400;
pub const REASON_SPSUMMON: u32 = 0x800;
pub const REASON_DISSUMMON: u32 = 0x1000;
pub const REASON_FLIP: u32 = 0x2000;
pub const REASON_DISCARD: u32 = 0x4000;
pub const REASON_RDAMAGE: u32 = 0x8000;
pub const REASON_RRECOVER: u32 = 0x10000;
pub const REASON_RETURN: u32 = 0x20000;
pub const REASON_FUSION: u32 = 0x40000;
pub const REASON_SYNCHRO: u32 = 0x80000;
pub const REASON_RITUAL: u32 = 0x100000;
pub const REASON_XYZ: u32 = 0x200000;
pub const REASON_REPLACE: u32 = 0x1000000;
pub const REASON_DRAW: u32 = 0x2000000;
pub const REASON_REDIRECT: u32 = 0x4000000;
// pub const REASON_REVEAL: u32 = 0x8000000;
pub const REASON_LINK: u32 = 0x10000000;

// Status
pub const STATUS_DISABLED: u32 = 0x0001;
pub const STATUS_TO_ENABLE: u32 = 0x0002;
pub const STATUS_TO_DISABLE: u32 = 0x0004;
pub const STATUS_PROC_COMPLETE: u32 = 0x0008;
pub const STATUS_SET_TURN: u32 = 0x0010;
pub const STATUS_NO_LEVEL: u32 = 0x0020;
pub const STATUS_BATTLE_RESULT: u32 = 0x0040;
pub const STATUS_SPSUMMON_STEP: u32 = 0x0080;
pub const STATUS_FORM_CHANGED: u32 = 0x0100;
pub const STATUS_SUMMONING: u32 = 0x0200;
pub const STATUS_EFFECT_ENABLED: u32 = 0x0400;
pub const STATUS_SUMMON_TURN: u32 = 0x0800;
pub const STATUS_DESTROY_CONFIRMED: u32 = 0x1000;
pub const STATUS_LEAVE_CONFIRMED: u32 = 0x2000;
pub const STATUS_BATTLE_DESTROYED: u32 = 0x4000;
pub const STATUS_COPYING_EFFECT: u32 = 0x8000;
pub const STATUS_CHAINING: u32 = 0x10000;
pub const STATUS_SUMMON_DISABLED: u32 = 0x20000;
pub const STATUS_ACTIVATE_DISABLED: u32 = 0x40000;
pub const STATUS_EFFECT_REPLACED: u32 = 0x80000;
pub const STATUS_FUTURE_FUSION: u32 = 0x100000;
pub const STATUS_ATTACK_CANCELED: u32 = 0x200000;
pub const STATUS_INITIALIZING: u32 = 0x400000;
// pub const STATUS_ACTIVATED: u32 = 0x800000;
pub const STATUS_JUST_POS: u32 = 0x1000000;
pub const STATUS_CONTINUOUS_POS: u32 = 0x2000000;
pub const STATUS_FORBIDDEN: u32 = 0x4000000;
pub const STATUS_ACT_FROM_HAND: u32 = 0x8000000;
pub const STATUS_OPPO_BATTLE: u32 = 0x10000000;
pub const STATUS_FLIP_SUMMON_TURN: u32 = 0x20000000;
pub const STATUS_SPSUMMON_TURN: u32 = 0x40000000;

// Query
pub const QUERY_CODE: u32 = 0x1;
pub const QUERY_POSITION: u32 = 0x2;
pub const QUERY_ALIAS: u32 = 0x4;
pub const QUERY_TYPE: u32 = 0x8;
pub const QUERY_LEVEL: u32 = 0x10;
pub const QUERY_RANK: u32 = 0x20;
pub const QUERY_ATTRIBUTE: u32 = 0x40;
pub const QUERY_RACE: u32 = 0x80;
pub const QUERY_ATTACK: u32 = 0x100;
pub const QUERY_DEFENSE: u32 = 0x200;
pub const QUERY_BASE_ATTACK: u32 = 0x400;
pub const QUERY_BASE_DEFENSE: u32 = 0x800;
pub const QUERY_REASON: u32 = 0x1000;
pub const QUERY_REASON_CARD: u32 = 0x2000;
pub const QUERY_EQUIP_CARD: u32 = 0x4000;
pub const QUERY_TARGET_CARD: u32 = 0x8000;
pub const QUERY_OVERLAY_CARD: u32 = 0x10000;
pub const QUERY_COUNTERS: u32 = 0x20000;
pub const QUERY_OWNER: u32 = 0x40000;
pub const QUERY_STATUS: u32 = 0x80000;
pub const QUERY_IS_PUBLIC: u32 = 0x100000;
pub const QUERY_LSCALE: u32 = 0x200000;
pub const QUERY_RSCALE: u32 = 0x400000;
pub const QUERY_LINK: u32 = 0x800000;
pub const QUERY_IS_HIDDEN: u32 = 0x1000000;
pub const QUERY_COVER: u32 = 0x2000000;
pub const QUERY_END: u32 = 0x80000000;

// Link Markers
pub const LINK_MARKER_BOTTOM_LEFT: u32 = 0o001;
pub const LINK_MARKER_BOTTOM: u32 = 0o002;
pub const LINK_MARKER_BOTTOM_RIGHT: u32 = 0o004;
pub const LINK_MARKER_LEFT: u32 = 0o010;
pub const LINK_MARKER_RIGHT: u32 = 0o040;
pub const LINK_MARKER_TOP_LEFT: u32 = 0o100;
pub const LINK_MARKER_TOP: u32 = 0o200;
pub const LINK_MARKER_TOP_RIGHT: u32 = 0o400;

// Messages
pub const MSG_RETRY: u8 = 1;
pub const MSG_HINT: u8 = 2;
pub const MSG_WAITING: u8 = 3;
pub const MSG_START: u8 = 4;
pub const MSG_WIN: u8 = 5;
pub const MSG_UPDATE_DATA: u8 = 6;
pub const MSG_UPDATE_CARD: u8 = 7;
pub const MSG_REQUEST_DECK: u8 = 8;
pub const MSG_SELECT_BATTLECMD: u8 = 10;
pub const MSG_SELECT_IDLECMD: u8 = 11;
pub const MSG_SELECT_EFFECTYN: u8 = 12;
pub const MSG_SELECT_YESNO: u8 = 13;
pub const MSG_SELECT_OPTION: u8 = 14;
pub const MSG_SELECT_CARD: u8 = 15;
pub const MSG_SELECT_CHAIN: u8 = 16;
pub const MSG_SELECT_PLACE: u8 = 18;
pub const MSG_SELECT_POSITION: u8 = 19;
pub const MSG_SELECT_TRIBUTE: u8 = 20;
pub const MSG_SORT_CHAIN: u8 = 21;
pub const MSG_SELECT_COUNTER: u8 = 22;
pub const MSG_SELECT_SUM: u8 = 23;
pub const MSG_SELECT_DISFIELD: u8 = 24;
pub const MSG_SORT_CARD: u8 = 25;
pub const MSG_SELECT_UNSELECT_CARD: u8 = 26;
pub const MSG_CONFIRM_DECKTOP: u8 = 30;
pub const MSG_CONFIRM_CARDS: u8 = 31;
pub const MSG_SHUFFLE_DECK: u8 = 32;
pub const MSG_SHUFFLE_HAND: u8 = 33;
pub const MSG_REFRESH_DECK: u8 = 34;
pub const MSG_SWAP_GRAVE_DECK: u8 = 35;
pub const MSG_SHUFFLE_SET_CARD: u8 = 36;
pub const MSG_REVERSE_DECK: u8 = 37;
pub const MSG_DECK_TOP: u8 = 38;
pub const MSG_SHUFFLE_EXTRA: u8 = 39;
pub const MSG_NEW_TURN: u8 = 40;
pub const MSG_NEW_PHASE: u8 = 41;
pub const MSG_CONFIRM_EXTRATOP: u8 = 42;
pub const MSG_MOVE: u8 = 50;
pub const MSG_POS_CHANGE: u8 = 53;
pub const MSG_SET: u8 = 54;
pub const MSG_SWAP: u8 = 55;
pub const MSG_FIELD_DISABLED: u8 = 56;
pub const MSG_SUMMONING: u8 = 60;
pub const MSG_SUMMONED: u8 = 61;
pub const MSG_SPSUMMONING: u8 = 62;
pub const MSG_SPSUMMONED: u8 = 63;
pub const MSG_FLIPSUMMONING: u8 = 64;
pub const MSG_FLIPSUMMONED: u8 = 65;
pub const MSG_CHAINING: u8 = 70;
pub const MSG_CHAINED: u8 = 71;
pub const MSG_CHAIN_SOLVING: u8 = 72;
pub const MSG_CHAIN_SOLVED: u8 = 73;
pub const MSG_CHAIN_END: u8 = 74;
pub const MSG_CHAIN_NEGATED: u8 = 75;
pub const MSG_CHAIN_DISABLED: u8 = 76;
pub const MSG_CARD_SELECTED: u8 = 80;
pub const MSG_RANDOM_SELECTED: u8 = 81;
pub const MSG_BECOME_TARGET: u8 = 83;
pub const MSG_DRAW: u8 = 90;
pub const MSG_DAMAGE: u8 = 91;
pub const MSG_RECOVER: u8 = 92;
pub const MSG_EQUIP: u8 = 93;
pub const MSG_LPUPDATE: u8 = 94;
pub const MSG_UNEQUIP: u8 = 95;
pub const MSG_CARD_TARGET: u8 = 96;
pub const MSG_CANCEL_TARGET: u8 = 97;
pub const MSG_PAY_LPCOST: u8 = 100;
pub const MSG_ADD_COUNTER: u8 = 101;
pub const MSG_REMOVE_COUNTER: u8 = 102;
pub const MSG_ATTACK: u8 = 110;
pub const MSG_BATTLE: u8 = 111;
pub const MSG_ATTACK_DISABLED: u8 = 112;
pub const MSG_DAMAGE_STEP_START: u8 = 113;
pub const MSG_DAMAGE_STEP_END: u8 = 114;
pub const MSG_MISSED_EFFECT: u8 = 120;
pub const MSG_BE_CHAIN_TARGET: u8 = 121;
pub const MSG_CREATE_RELATION: u8 = 122;
pub const MSG_RELEASE_RELATION: u8 = 123;
pub const MSG_TOSS_COIN: u8 = 130;
pub const MSG_TOSS_DICE: u8 = 131;
pub const MSG_ROCK_PAPER_SCISSORS: u8 = 132;
pub const MSG_HAND_RES: u8 = 133;
pub const MSG_ANNOUNCE_RACE: u8 = 140;
pub const MSG_ANNOUNCE_ATTRIB: u8 = 141;
pub const MSG_ANNOUNCE_CARD: u8 = 142;
pub const MSG_ANNOUNCE_NUMBER: u8 = 143;
pub const MSG_CARD_HINT: u8 = 160;
pub const MSG_TAG_SWAP: u8 = 161;
pub const MSG_RELOAD_FIELD: u8 = 162; // Debug.ReloadFieldEnd()
pub const MSG_AI_NAME: u8 = 163;
pub const MSG_SHOW_HINT: u8 = 164;
pub const MSG_PLAYER_HINT: u8 = 165;
pub const MSG_MATCH_KILL: u8 = 170;
pub const MSG_CUSTOM_MSG: u8 = 180;
pub const MSG_REMOVE_CARDS: u8 = 190;

pub const OLD_REPLAY_MODE: u8 = 231;

// Hints
pub const HINT_EVENT: u8 = 1;
pub const HINT_MESSAGE: u8 = 2;
pub const HINT_SELECTMSG: u8 = 3;
pub const HINT_OPSELECTED: u8 = 4;
pub const HINT_EFFECT: u8 = 5;
pub const HINT_RACE: u8 = 6;
pub const HINT_ATTRIB: u8 = 7;
pub const HINT_CODE: u8 = 8;
pub const HINT_NUMBER: u8 = 9;
pub const HINT_CARD: u8 = 10;
pub const HINT_ZONE: u8 = 11;

pub const CHINT_TURN: u8 = 1;
pub const CHINT_CARD: u8 = 2;
pub const CHINT_RACE: u8 = 3;
pub const CHINT_ATTRIBUTE: u8 = 4;
pub const CHINT_NUMBER: u8 = 5;
pub const CHINT_DESC_ADD: u8 = 6;
pub const CHINT_DESC_REMOVE: u8 = 7;

pub const PHINT_DESC_ADD: u8 = 6;
pub const PHINT_DESC_REMOVE: u8 = 7;

pub const EFFECT_CLIENT_MODE_NORMAL: u8 = 0;
pub const EFFECT_CLIENT_MODE_RESOLVE: u8 = 1;
pub const EFFECT_CLIENT_MODE_RESET: u8 = 2;

pub const OPCODE_ADD: u64 = 0x4000000000000000;
pub const OPCODE_SUB: u64 = 0x4000000100000000;
pub const OPCODE_MUL: u64 = 0x4000000200000000;
pub const OPCODE_DIV: u64 = 0x4000000300000000;
pub const OPCODE_AND: u64 = 0x4000000400000000;
pub const OPCODE_OR: u64 = 0x4000000500000000;
pub const OPCODE_NEG: u64 = 0x4000000600000000;
pub const OPCODE_NOT: u64 = 0x4000000700000000;
pub const OPCODE_BAND: u64 = 0x4000000800000000;
pub const OPCODE_BOR: u64 = 0x4000000900000000;
pub const OPCODE_BNOT: u64 = 0x4000001000000000;
pub const OPCODE_BXOR: u64 = 0x4000001100000000;
pub const OPCODE_LSHIFT: u64 = 0x4000001200000000;
pub const OPCODE_RSHIFT: u64 = 0x4000001300000000;
pub const OPCODE_ALLOW_ALIASES: u64 = 0x4000001400000000;
pub const OPCODE_ALLOW_TOKENS: u64 = 0x4000001500000000;
pub const OPCODE_ISCODE: u64 = 0x4000010000000000;
pub const OPCODE_ISSETCARD: u64 = 0x4000010100000000;
pub const OPCODE_ISTYPE: u64 = 0x4000010200000000;
pub const OPCODE_ISRACE: u64 = 0x4000010300000000;
pub const OPCODE_ISATTRIBUTE: u64 = 0x4000010400000000;
pub const OPCODE_GETCODE: u64 = 0x4000010500000000;
pub const OPCODE_GETSETCARD: u64 = 0x4000010600000000;
pub const OPCODE_GETTYPE: u64 = 0x4000010700000000;
pub const OPCODE_GETRACE: u64 = 0x4000010800000000;
pub const OPCODE_GETATTRIBUTE: u64 = 0x4000010900000000;
#[repr(u64)]
pub enum Opcode {
    Add = OPCODE_ADD,
    Sub = OPCODE_SUB,
    Mul = OPCODE_MUL,
    Div = OPCODE_DIV,
    And = OPCODE_AND,
    Or = OPCODE_OR,
    Neg = OPCODE_NEG,
    Not = OPCODE_NOT,
    Band = OPCODE_BAND,
    Bor = OPCODE_BOR,
    Bnot = OPCODE_BNOT,
    Bxor = OPCODE_BXOR,
    Lshift = OPCODE_LSHIFT,
    Rshift = OPCODE_RSHIFT,
    AllowAliases = OPCODE_ALLOW_ALIASES,
    AllowTokens = OPCODE_ALLOW_TOKENS,
    IsCode = OPCODE_ISCODE,
    IsSetCard = OPCODE_ISSETCARD,
    IsType = OPCODE_ISTYPE,
    IsRace = OPCODE_ISRACE,
    IsAttribute = OPCODE_ISATTRIBUTE,
    GetCode = OPCODE_GETCODE,
    GetSetCard = OPCODE_GETSETCARD,
    GetType = OPCODE_GETTYPE,
    GetRace = OPCODE_GETRACE,
    GetAttribute = OPCODE_GETATTRIBUTE,
}

// Players
pub const PLAYER_NONE: u8 = 2;
pub const PLAYER_ALL: u8 = 3;
pub const PLAYER_SELFDES: u8 = 5;

// Phases
pub const PHASE_DRAW: u16 = 0x01;
pub const PHASE_STANDBY: u16 = 0x02;
pub const PHASE_MAIN1: u16 = 0x04;
pub const PHASE_BATTLE_START: u16 = 0x08;
pub const PHASE_BATTLE_STEP: u16 = 0x10;
pub const PHASE_DAMAGE: u16 = 0x20;
pub const PHASE_DAMAGE_CAL: u16 = 0x40;
pub const PHASE_BATTLE: u16 = 0x80;
pub const PHASE_MAIN2: u16 = 0x100;
pub const PHASE_END: u16 = 0x200;
#[repr(u16)]
pub enum Phase {
    Draw = PHASE_DRAW,
    Standby = PHASE_STANDBY,
    Main1 = PHASE_MAIN1,
    BattleStart = PHASE_BATTLE_START,
    BattleStep = PHASE_BATTLE_STEP,
    Damage = PHASE_DAMAGE,
    DamageCal = PHASE_DAMAGE_CAL,
    Battle = PHASE_BATTLE,
    Main2 = PHASE_MAIN2,
    End = PHASE_END,
}

// Options
pub const DUEL_TEST_MODE: u64 = 0x01;
pub const DUEL_ATTACK_FIRST_TURN: u64 = 0x02;
pub const DUEL_USE_TRAPS_IN_NEW_CHAIN: u64 = 0x04;
pub const DUEL_6_STEP_BATLLE_STEP: u64 = 0x08;
pub const DUEL_PSEUDO_SHUFFLE: u64 = 0x10;
pub const DUEL_TRIGGER_WHEN_PRIVATE_KNOWLEDGE: u64 = 0x20;
pub const DUEL_SIMPLE_AI: u64 = 0x40;
pub const DUEL_RELAY: u64 = 0x80;
pub const DUEL_OBSOLETE_IGNITION: u64 = 0x100;
pub const DUEL_1ST_TURN_DRAW: u64 = 0x200;
pub const DUEL_1_FACEUP_FIELD: u64 = 0x400;
pub const DUEL_PZONE: u64 = 0x800;
pub const DUEL_SEPARATE_PZONE: u64 = 0x1000;
pub const DUEL_EMZONE: u64 = 0x2000;
pub const DUEL_FSX_MMZONE: u64 = 0x4000;
pub const DUEL_TRAP_MONSTERS_NOT_USE_ZONE: u64 = 0x8000;
pub const DUEL_RETURN_TO_DECK_TRIGGERS: u64 = 0x10000;
pub const DUEL_TRIGGER_ONLY_IN_LOCATION: u64 = 0x20000;
pub const DUEL_SPSUMMON_ONCE_OLD_NEGATE: u64 = 0x40000;
pub const DUEL_CANNOT_SUMMON_OATH_OLD: u64 = 0x80000;
pub const DUEL_NO_STANDBY_PHASE: u64 = 0x100000;
pub const DUEL_NO_MAIN_PHASE_2: u64 = 0x200000;
pub const DUEL_3_COLUMNS_FIELD: u64 = 0x400000;
pub const DUEL_DRAW_UNTIL_5: u64 = 0x800000;
pub const DUEL_NO_HAND_LIMIT: u64 = 0x1000000;
pub const DUEL_UNLIMITED_SUMMONS: u64 = 0x2000000;
pub const DUEL_INVERTED_QUICK_PRIORITY: u64 = 0x4000000;
pub const DUEL_EQUIP_NOT_SENT_IF_MISSING_TARGET: u64 = 0x8000000;
pub const DUEL_0_ATK_DESTROYED: u64 = 0x10000000;
pub const DUEL_STORE_ATTACK_REPLAYS: u64 = 0x20000000;
pub const DUEL_SINGLE_CHAIN_IN_DAMAGE_SUBSTEP: u64 = 0x40000000;
pub const DUEL_CAN_REPOS_IF_NON_SUMPLAYER: u64 = 0x80000000;
pub const DUEL_TCG_SEGOC_NONPUBLIC: u64 = 0x100000000;
pub const DUEL_TCG_SEGOC_FIRSTTRIGGER: u64 = 0x200000000;
pub const DUEL_MODE_SPEED: u64 = DUEL_3_COLUMNS_FIELD
    | DUEL_NO_MAIN_PHASE_2
    | DUEL_TRAP_MONSTERS_NOT_USE_ZONE
    | DUEL_TRIGGER_ONLY_IN_LOCATION;
pub const DUEL_MODE_RUSH: u64 = DUEL_3_COLUMNS_FIELD
    | DUEL_NO_MAIN_PHASE_2
    | DUEL_NO_STANDBY_PHASE
    | DUEL_1ST_TURN_DRAW
    | DUEL_INVERTED_QUICK_PRIORITY
    | DUEL_DRAW_UNTIL_5
    | DUEL_NO_HAND_LIMIT
    | DUEL_UNLIMITED_SUMMONS
    | DUEL_TRAP_MONSTERS_NOT_USE_ZONE
    | DUEL_TRIGGER_ONLY_IN_LOCATION;
pub const DUEL_MODE_MR1: u64 = DUEL_OBSOLETE_IGNITION
    | DUEL_1ST_TURN_DRAW
    | DUEL_1_FACEUP_FIELD
    | DUEL_SPSUMMON_ONCE_OLD_NEGATE
    | DUEL_RETURN_TO_DECK_TRIGGERS
    | DUEL_CANNOT_SUMMON_OATH_OLD;
pub const DUEL_MODE_GOAT: u64 = DUEL_MODE_MR1
    | DUEL_USE_TRAPS_IN_NEW_CHAIN
    | DUEL_6_STEP_BATLLE_STEP
    | DUEL_TRIGGER_WHEN_PRIVATE_KNOWLEDGE
    | DUEL_EQUIP_NOT_SENT_IF_MISSING_TARGET
    | DUEL_0_ATK_DESTROYED
    | DUEL_STORE_ATTACK_REPLAYS
    | DUEL_SINGLE_CHAIN_IN_DAMAGE_SUBSTEP
    | DUEL_CAN_REPOS_IF_NON_SUMPLAYER
    | DUEL_TCG_SEGOC_NONPUBLIC
    | DUEL_TCG_SEGOC_FIRSTTRIGGER;
pub const DUEL_MODE_MR2: u64 = DUEL_1ST_TURN_DRAW
    | DUEL_1_FACEUP_FIELD
    | DUEL_SPSUMMON_ONCE_OLD_NEGATE
    | DUEL_RETURN_TO_DECK_TRIGGERS
    | DUEL_CANNOT_SUMMON_OATH_OLD;
pub const DUEL_MODE_MR3: u64 = DUEL_PZONE
    | DUEL_SEPARATE_PZONE
    | DUEL_SPSUMMON_ONCE_OLD_NEGATE
    | DUEL_RETURN_TO_DECK_TRIGGERS
    | DUEL_CANNOT_SUMMON_OATH_OLD;
pub const DUEL_MODE_MR4: u64 = DUEL_PZONE
    | DUEL_EMZONE
    | DUEL_SPSUMMON_ONCE_OLD_NEGATE
    | DUEL_RETURN_TO_DECK_TRIGGERS
    | DUEL_CANNOT_SUMMON_OATH_OLD;
pub const DUEL_MODE_MR5: u64 = DUEL_PZONE
    | DUEL_EMZONE
    | DUEL_FSX_MMZONE
    | DUEL_TRAP_MONSTERS_NOT_USE_ZONE
    | DUEL_TRIGGER_ONLY_IN_LOCATION;
bitflags! {
    pub struct DuelRules: u64 {
        const TestMode = DUEL_TEST_MODE;
        const AttackFirstTurn = DUEL_ATTACK_FIRST_TURN;
        const UseTrapsInNewChain = DUEL_USE_TRAPS_IN_NEW_CHAIN;
        const SixStepBattleStep = DUEL_6_STEP_BATLLE_STEP;
        const PseudoShuffle = DUEL_PSEUDO_SHUFFLE;
        const TriggerWhenPrivateKnowledge = DUEL_TRIGGER_WHEN_PRIVATE_KNOWLEDGE;
        const SimpleAI = DUEL_SIMPLE_AI;
        const Relay = DUEL_RELAY;
        const ObsoleteIgnition = DUEL_OBSOLETE_IGNITION;
        const FirstTurnDraw = DUEL_1ST_TURN_DRAW;
        const OneFaceupField = DUEL_1_FACEUP_FIELD;
        const PZone = DUEL_PZONE;
        const SeparatePZone = DUEL_SEPARATE_PZONE;
        const EMZone = DUEL_EMZONE;
        const FSXMMZone = DUEL_FSX_MMZONE;
        const TrapMonstersNotUseZone = DUEL_TRAP_MONSTERS_NOT_USE_ZONE;
        const ReturnToDeckTriggers = DUEL_RETURN_TO_DECK_TRIGGERS;
        const TriggerOnlyInLocation = DUEL_TRIGGER_ONLY_IN_LOCATION;
        const SPSummonOnceOldNegate = DUEL_SPSUMMON_ONCE_OLD_NEGATE;
        const CannotSummonOathOld = DUEL_CANNOT_SUMMON_OATH_OLD;
        const NoStandbyPhase = DUEL_NO_STANDBY_PHASE;
        const NoMainPhase2 = DUEL_NO_MAIN_PHASE_2;
        const ThreeColumnsField = DUEL_3_COLUMNS_FIELD;
        const DrawUntil5 = DUEL_DRAW_UNTIL_5;
        const NoHandLimit = DUEL_NO_HAND_LIMIT;
        const UnlimitedSummons = DUEL_UNLIMITED_SUMMONS;
        const InvertedQuickPriority = DUEL_INVERTED_QUICK_PRIORITY;
        const EquipNotSentIfMissingTarget = DUEL_EQUIP_NOT_SENT_IF_MISSING_TARGET;
        const ZeroAtkDestroyed = DUEL_0_ATK_DESTROYED;
        const StoreAttackReplays = DUEL_STORE_ATTACK_REPLAYS;
        const SingleChainInDamageSubstep = DUEL_SINGLE_CHAIN_IN_DAMAGE_SUBSTEP;
        const CanReposIfNonSumplayer = DUEL_CAN_REPOS_IF_NON_SUMPLAYER;
        const TCGSegocNonpublic = DUEL_TCG_SEGOC_NONPUBLIC;
        const TCGSegocFirsttrigger = DUEL_TCG_SEGOC_FIRSTTRIGGER;
        const ModeSpeed = DUEL_MODE_SPEED;
        const ModeRush = DUEL_MODE_RUSH;
        const ModeMR1 = DUEL_MODE_MR1;
        const ModeGoat = DUEL_MODE_GOAT;
        const ModeMR2 = DUEL_MODE_MR2;
        const ModeMR3 = DUEL_MODE_MR3;
        const ModeMR4 = DUEL_MODE_MR4;
        const ModeMR5 = DUEL_MODE_MR5;
    }
}
pub const DUEL_MODE_MR1_FORB: u32 = TYPE_XYZ | TYPE_PENDULUM | TYPE_LINK;
pub const DUEL_MODE_MR2_FORB: u32 = TYPE_PENDULUM | TYPE_LINK;
pub const DUEL_MODE_MR3_FORB: u32 = TYPE_LINK;
pub const DUEL_MODE_MR4_FORB: u32 = 0;
pub const DUEL_MODE_MR5_FORB: u32 = 0;
bitflags! {
    pub struct DuelRuleFORB: u32 {
        const ModeMR1 = DUEL_MODE_MR1_FORB;
        const ModeMR2 = DUEL_MODE_MR2_FORB;
        const ModeMR3 = DUEL_MODE_MR3_FORB;
        const ModeMR4 = DUEL_MODE_MR4_FORB;
        const ModeMR5 = DUEL_MODE_MR5_FORB;
    }
}

#[repr(u8)]
pub enum ActivityType {
    Summon = 1,
    NormalSummon = 2,
    SpSummon = 3,
    FlipSummon = 4,
    Attack = 5,
    BattlePhase = 6,
    Chain = 7,
}
