#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Control 1 Register"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Free Running Timer Register"]
    pub timer: TIMER,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Rx Mailboxes Global Mask Register"]
    pub rxmgmask: RXMGMASK,
    #[doc = "0x14 - Rx Buffer 14 Mask Register"]
    pub rx14mask: RX14MASK,
    #[doc = "0x18 - Rx Buffer 15 Mask Register"]
    pub rx15mask: RX15MASK,
    #[doc = "0x1c - Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x20 - Error and Status 1 Register"]
    pub esr1: ESR1,
    #[doc = "0x24 - Interrupt Masks 2 Register"]
    pub imask2: IMASK2,
    #[doc = "0x28 - Interrupt Masks 1 Register"]
    pub imask1: IMASK1,
    #[doc = "0x2c - Interrupt Flags 2 Register"]
    pub iflag2: IFLAG2,
    #[doc = "0x30 - Interrupt Flags 1 Register"]
    pub iflag1: IFLAG1,
    #[doc = "0x34 - Control 2 Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x38 - Error and Status 2 Register"]
    pub esr2: ESR2,
    _reserved1: [u8; 8usize],
    #[doc = "0x44 - CRC Register"]
    pub crcr: CRCR,
    #[doc = "0x48 - Rx FIFO Global Mask Register"]
    pub rxfgmask: RXFGMASK,
    #[doc = "0x4c - Rx FIFO Information Register"]
    pub rxfir: RXFIR,
    _reserved2: [u8; 48usize],
    #[doc = "0x80 - Message Buffer 0 CS Register"]
    pub cs0: CS0,
    #[doc = "0x84 - Message Buffer 0 ID Register"]
    pub id0: ID0,
    #[doc = "0x88 - Message Buffer 0 WORD0 Register"]
    pub word00: WORD00,
    #[doc = "0x8c - Message Buffer 0 WORD1 Register"]
    pub word10: WORD10,
    #[doc = "0x90 - Message Buffer 1 CS Register"]
    pub cs1: CS1,
    #[doc = "0x94 - Message Buffer 1 ID Register"]
    pub id1: ID1,
    #[doc = "0x98 - Message Buffer 1 WORD0 Register"]
    pub word01: WORD01,
    #[doc = "0x9c - Message Buffer 1 WORD1 Register"]
    pub word11: WORD11,
    #[doc = "0xa0 - Message Buffer 2 CS Register"]
    pub cs2: CS2,
    #[doc = "0xa4 - Message Buffer 2 ID Register"]
    pub id2: ID2,
    #[doc = "0xa8 - Message Buffer 2 WORD0 Register"]
    pub word02: WORD02,
    #[doc = "0xac - Message Buffer 2 WORD1 Register"]
    pub word12: WORD12,
    #[doc = "0xb0 - Message Buffer 3 CS Register"]
    pub cs3: CS3,
    #[doc = "0xb4 - Message Buffer 3 ID Register"]
    pub id3: ID3,
    #[doc = "0xb8 - Message Buffer 3 WORD0 Register"]
    pub word03: WORD03,
    #[doc = "0xbc - Message Buffer 3 WORD1 Register"]
    pub word13: WORD13,
    #[doc = "0xc0 - Message Buffer 4 CS Register"]
    pub cs4: CS4,
    #[doc = "0xc4 - Message Buffer 4 ID Register"]
    pub id4: ID4,
    #[doc = "0xc8 - Message Buffer 4 WORD0 Register"]
    pub word04: WORD04,
    #[doc = "0xcc - Message Buffer 4 WORD1 Register"]
    pub word14: WORD14,
    #[doc = "0xd0 - Message Buffer 5 CS Register"]
    pub cs5: CS5,
    #[doc = "0xd4 - Message Buffer 5 ID Register"]
    pub id5: ID5,
    #[doc = "0xd8 - Message Buffer 5 WORD0 Register"]
    pub word05: WORD05,
    #[doc = "0xdc - Message Buffer 5 WORD1 Register"]
    pub word15: WORD15,
    #[doc = "0xe0 - Message Buffer 6 CS Register"]
    pub cs6: CS6,
    #[doc = "0xe4 - Message Buffer 6 ID Register"]
    pub id6: ID6,
    #[doc = "0xe8 - Message Buffer 6 WORD0 Register"]
    pub word06: WORD06,
    #[doc = "0xec - Message Buffer 6 WORD1 Register"]
    pub word16: WORD16,
    #[doc = "0xf0 - Message Buffer 7 CS Register"]
    pub cs7: CS7,
    #[doc = "0xf4 - Message Buffer 7 ID Register"]
    pub id7: ID7,
    #[doc = "0xf8 - Message Buffer 7 WORD0 Register"]
    pub word07: WORD07,
    #[doc = "0xfc - Message Buffer 7 WORD1 Register"]
    pub word17: WORD17,
    #[doc = "0x100 - Message Buffer 8 CS Register"]
    pub cs8: CS8,
    #[doc = "0x104 - Message Buffer 8 ID Register"]
    pub id8: ID8,
    #[doc = "0x108 - Message Buffer 8 WORD0 Register"]
    pub word08: WORD08,
    #[doc = "0x10c - Message Buffer 8 WORD1 Register"]
    pub word18: WORD18,
    #[doc = "0x110 - Message Buffer 9 CS Register"]
    pub cs9: CS9,
    #[doc = "0x114 - Message Buffer 9 ID Register"]
    pub id9: ID9,
    #[doc = "0x118 - Message Buffer 9 WORD0 Register"]
    pub word09: WORD09,
    #[doc = "0x11c - Message Buffer 9 WORD1 Register"]
    pub word19: WORD19,
    #[doc = "0x120 - Message Buffer 10 CS Register"]
    pub cs10: CS10,
    #[doc = "0x124 - Message Buffer 10 ID Register"]
    pub id10: ID10,
    #[doc = "0x128 - Message Buffer 10 WORD0 Register"]
    pub word010: WORD010,
    #[doc = "0x12c - Message Buffer 10 WORD1 Register"]
    pub word110: WORD110,
    #[doc = "0x130 - Message Buffer 11 CS Register"]
    pub cs11: CS11,
    #[doc = "0x134 - Message Buffer 11 ID Register"]
    pub id11: ID11,
    #[doc = "0x138 - Message Buffer 11 WORD0 Register"]
    pub word011: WORD011,
    #[doc = "0x13c - Message Buffer 11 WORD1 Register"]
    pub word111: WORD111,
    #[doc = "0x140 - Message Buffer 12 CS Register"]
    pub cs12: CS12,
    #[doc = "0x144 - Message Buffer 12 ID Register"]
    pub id12: ID12,
    #[doc = "0x148 - Message Buffer 12 WORD0 Register"]
    pub word012: WORD012,
    #[doc = "0x14c - Message Buffer 12 WORD1 Register"]
    pub word112: WORD112,
    #[doc = "0x150 - Message Buffer 13 CS Register"]
    pub cs13: CS13,
    #[doc = "0x154 - Message Buffer 13 ID Register"]
    pub id13: ID13,
    #[doc = "0x158 - Message Buffer 13 WORD0 Register"]
    pub word013: WORD013,
    #[doc = "0x15c - Message Buffer 13 WORD1 Register"]
    pub word113: WORD113,
    #[doc = "0x160 - Message Buffer 14 CS Register"]
    pub cs14: CS14,
    #[doc = "0x164 - Message Buffer 14 ID Register"]
    pub id14: ID14,
    #[doc = "0x168 - Message Buffer 14 WORD0 Register"]
    pub word014: WORD014,
    #[doc = "0x16c - Message Buffer 14 WORD1 Register"]
    pub word114: WORD114,
    #[doc = "0x170 - Message Buffer 15 CS Register"]
    pub cs15: CS15,
    #[doc = "0x174 - Message Buffer 15 ID Register"]
    pub id15: ID15,
    #[doc = "0x178 - Message Buffer 15 WORD0 Register"]
    pub word015: WORD015,
    #[doc = "0x17c - Message Buffer 15 WORD1 Register"]
    pub word115: WORD115,
    #[doc = "0x180 - Message Buffer 16 CS Register"]
    pub cs16: CS16,
    #[doc = "0x184 - Message Buffer 16 ID Register"]
    pub id16: ID16,
    #[doc = "0x188 - Message Buffer 16 WORD0 Register"]
    pub word016: WORD016,
    #[doc = "0x18c - Message Buffer 16 WORD1 Register"]
    pub word116: WORD116,
    #[doc = "0x190 - Message Buffer 17 CS Register"]
    pub cs17: CS17,
    #[doc = "0x194 - Message Buffer 17 ID Register"]
    pub id17: ID17,
    #[doc = "0x198 - Message Buffer 17 WORD0 Register"]
    pub word017: WORD017,
    #[doc = "0x19c - Message Buffer 17 WORD1 Register"]
    pub word117: WORD117,
    #[doc = "0x1a0 - Message Buffer 18 CS Register"]
    pub cs18: CS18,
    #[doc = "0x1a4 - Message Buffer 18 ID Register"]
    pub id18: ID18,
    #[doc = "0x1a8 - Message Buffer 18 WORD0 Register"]
    pub word018: WORD018,
    #[doc = "0x1ac - Message Buffer 18 WORD1 Register"]
    pub word118: WORD118,
    #[doc = "0x1b0 - Message Buffer 19 CS Register"]
    pub cs19: CS19,
    #[doc = "0x1b4 - Message Buffer 19 ID Register"]
    pub id19: ID19,
    #[doc = "0x1b8 - Message Buffer 19 WORD0 Register"]
    pub word019: WORD019,
    #[doc = "0x1bc - Message Buffer 19 WORD1 Register"]
    pub word119: WORD119,
    #[doc = "0x1c0 - Message Buffer 20 CS Register"]
    pub cs20: CS20,
    #[doc = "0x1c4 - Message Buffer 20 ID Register"]
    pub id20: ID20,
    #[doc = "0x1c8 - Message Buffer 20 WORD0 Register"]
    pub word020: WORD020,
    #[doc = "0x1cc - Message Buffer 20 WORD1 Register"]
    pub word120: WORD120,
    #[doc = "0x1d0 - Message Buffer 21 CS Register"]
    pub cs21: CS21,
    #[doc = "0x1d4 - Message Buffer 21 ID Register"]
    pub id21: ID21,
    #[doc = "0x1d8 - Message Buffer 21 WORD0 Register"]
    pub word021: WORD021,
    #[doc = "0x1dc - Message Buffer 21 WORD1 Register"]
    pub word121: WORD121,
    #[doc = "0x1e0 - Message Buffer 22 CS Register"]
    pub cs22: CS22,
    #[doc = "0x1e4 - Message Buffer 22 ID Register"]
    pub id22: ID22,
    #[doc = "0x1e8 - Message Buffer 22 WORD0 Register"]
    pub word022: WORD022,
    #[doc = "0x1ec - Message Buffer 22 WORD1 Register"]
    pub word122: WORD122,
    #[doc = "0x1f0 - Message Buffer 23 CS Register"]
    pub cs23: CS23,
    #[doc = "0x1f4 - Message Buffer 23 ID Register"]
    pub id23: ID23,
    #[doc = "0x1f8 - Message Buffer 23 WORD0 Register"]
    pub word023: WORD023,
    #[doc = "0x1fc - Message Buffer 23 WORD1 Register"]
    pub word123: WORD123,
    #[doc = "0x200 - Message Buffer 24 CS Register"]
    pub cs24: CS24,
    #[doc = "0x204 - Message Buffer 24 ID Register"]
    pub id24: ID24,
    #[doc = "0x208 - Message Buffer 24 WORD0 Register"]
    pub word024: WORD024,
    #[doc = "0x20c - Message Buffer 24 WORD1 Register"]
    pub word124: WORD124,
    #[doc = "0x210 - Message Buffer 25 CS Register"]
    pub cs25: CS25,
    #[doc = "0x214 - Message Buffer 25 ID Register"]
    pub id25: ID25,
    #[doc = "0x218 - Message Buffer 25 WORD0 Register"]
    pub word025: WORD025,
    #[doc = "0x21c - Message Buffer 25 WORD1 Register"]
    pub word125: WORD125,
    #[doc = "0x220 - Message Buffer 26 CS Register"]
    pub cs26: CS26,
    #[doc = "0x224 - Message Buffer 26 ID Register"]
    pub id26: ID26,
    #[doc = "0x228 - Message Buffer 26 WORD0 Register"]
    pub word026: WORD026,
    #[doc = "0x22c - Message Buffer 26 WORD1 Register"]
    pub word126: WORD126,
    #[doc = "0x230 - Message Buffer 27 CS Register"]
    pub cs27: CS27,
    #[doc = "0x234 - Message Buffer 27 ID Register"]
    pub id27: ID27,
    #[doc = "0x238 - Message Buffer 27 WORD0 Register"]
    pub word027: WORD027,
    #[doc = "0x23c - Message Buffer 27 WORD1 Register"]
    pub word127: WORD127,
    #[doc = "0x240 - Message Buffer 28 CS Register"]
    pub cs28: CS28,
    #[doc = "0x244 - Message Buffer 28 ID Register"]
    pub id28: ID28,
    #[doc = "0x248 - Message Buffer 28 WORD0 Register"]
    pub word028: WORD028,
    #[doc = "0x24c - Message Buffer 28 WORD1 Register"]
    pub word128: WORD128,
    #[doc = "0x250 - Message Buffer 29 CS Register"]
    pub cs29: CS29,
    #[doc = "0x254 - Message Buffer 29 ID Register"]
    pub id29: ID29,
    #[doc = "0x258 - Message Buffer 29 WORD0 Register"]
    pub word029: WORD029,
    #[doc = "0x25c - Message Buffer 29 WORD1 Register"]
    pub word129: WORD129,
    #[doc = "0x260 - Message Buffer 30 CS Register"]
    pub cs30: CS30,
    #[doc = "0x264 - Message Buffer 30 ID Register"]
    pub id30: ID30,
    #[doc = "0x268 - Message Buffer 30 WORD0 Register"]
    pub word030: WORD030,
    #[doc = "0x26c - Message Buffer 30 WORD1 Register"]
    pub word130: WORD130,
    #[doc = "0x270 - Message Buffer 31 CS Register"]
    pub cs31: CS31,
    #[doc = "0x274 - Message Buffer 31 ID Register"]
    pub id31: ID31,
    #[doc = "0x278 - Message Buffer 31 WORD0 Register"]
    pub word031: WORD031,
    #[doc = "0x27c - Message Buffer 31 WORD1 Register"]
    pub word131: WORD131,
    #[doc = "0x280 - Message Buffer 32 CS Register"]
    pub cs32: CS32,
    #[doc = "0x284 - Message Buffer 32 ID Register"]
    pub id32: ID32,
    #[doc = "0x288 - Message Buffer 32 WORD0 Register"]
    pub word032: WORD032,
    #[doc = "0x28c - Message Buffer 32 WORD1 Register"]
    pub word132: WORD132,
    #[doc = "0x290 - Message Buffer 33 CS Register"]
    pub cs33: CS33,
    #[doc = "0x294 - Message Buffer 33 ID Register"]
    pub id33: ID33,
    #[doc = "0x298 - Message Buffer 33 WORD0 Register"]
    pub word033: WORD033,
    #[doc = "0x29c - Message Buffer 33 WORD1 Register"]
    pub word133: WORD133,
    #[doc = "0x2a0 - Message Buffer 34 CS Register"]
    pub cs34: CS34,
    #[doc = "0x2a4 - Message Buffer 34 ID Register"]
    pub id34: ID34,
    #[doc = "0x2a8 - Message Buffer 34 WORD0 Register"]
    pub word034: WORD034,
    #[doc = "0x2ac - Message Buffer 34 WORD1 Register"]
    pub word134: WORD134,
    #[doc = "0x2b0 - Message Buffer 35 CS Register"]
    pub cs35: CS35,
    #[doc = "0x2b4 - Message Buffer 35 ID Register"]
    pub id35: ID35,
    #[doc = "0x2b8 - Message Buffer 35 WORD0 Register"]
    pub word035: WORD035,
    #[doc = "0x2bc - Message Buffer 35 WORD1 Register"]
    pub word135: WORD135,
    #[doc = "0x2c0 - Message Buffer 36 CS Register"]
    pub cs36: CS36,
    #[doc = "0x2c4 - Message Buffer 36 ID Register"]
    pub id36: ID36,
    #[doc = "0x2c8 - Message Buffer 36 WORD0 Register"]
    pub word036: WORD036,
    #[doc = "0x2cc - Message Buffer 36 WORD1 Register"]
    pub word136: WORD136,
    #[doc = "0x2d0 - Message Buffer 37 CS Register"]
    pub cs37: CS37,
    #[doc = "0x2d4 - Message Buffer 37 ID Register"]
    pub id37: ID37,
    #[doc = "0x2d8 - Message Buffer 37 WORD0 Register"]
    pub word037: WORD037,
    #[doc = "0x2dc - Message Buffer 37 WORD1 Register"]
    pub word137: WORD137,
    #[doc = "0x2e0 - Message Buffer 38 CS Register"]
    pub cs38: CS38,
    #[doc = "0x2e4 - Message Buffer 38 ID Register"]
    pub id38: ID38,
    #[doc = "0x2e8 - Message Buffer 38 WORD0 Register"]
    pub word038: WORD038,
    #[doc = "0x2ec - Message Buffer 38 WORD1 Register"]
    pub word138: WORD138,
    #[doc = "0x2f0 - Message Buffer 39 CS Register"]
    pub cs39: CS39,
    #[doc = "0x2f4 - Message Buffer 39 ID Register"]
    pub id39: ID39,
    #[doc = "0x2f8 - Message Buffer 39 WORD0 Register"]
    pub word039: WORD039,
    #[doc = "0x2fc - Message Buffer 39 WORD1 Register"]
    pub word139: WORD139,
    #[doc = "0x300 - Message Buffer 40 CS Register"]
    pub cs40: CS40,
    #[doc = "0x304 - Message Buffer 40 ID Register"]
    pub id40: ID40,
    #[doc = "0x308 - Message Buffer 40 WORD0 Register"]
    pub word040: WORD040,
    #[doc = "0x30c - Message Buffer 40 WORD1 Register"]
    pub word140: WORD140,
    #[doc = "0x310 - Message Buffer 41 CS Register"]
    pub cs41: CS41,
    #[doc = "0x314 - Message Buffer 41 ID Register"]
    pub id41: ID41,
    #[doc = "0x318 - Message Buffer 41 WORD0 Register"]
    pub word041: WORD041,
    #[doc = "0x31c - Message Buffer 41 WORD1 Register"]
    pub word141: WORD141,
    #[doc = "0x320 - Message Buffer 42 CS Register"]
    pub cs42: CS42,
    #[doc = "0x324 - Message Buffer 42 ID Register"]
    pub id42: ID42,
    #[doc = "0x328 - Message Buffer 42 WORD0 Register"]
    pub word042: WORD042,
    #[doc = "0x32c - Message Buffer 42 WORD1 Register"]
    pub word142: WORD142,
    #[doc = "0x330 - Message Buffer 43 CS Register"]
    pub cs43: CS43,
    #[doc = "0x334 - Message Buffer 43 ID Register"]
    pub id43: ID43,
    #[doc = "0x338 - Message Buffer 43 WORD0 Register"]
    pub word043: WORD043,
    #[doc = "0x33c - Message Buffer 43 WORD1 Register"]
    pub word143: WORD143,
    #[doc = "0x340 - Message Buffer 44 CS Register"]
    pub cs44: CS44,
    #[doc = "0x344 - Message Buffer 44 ID Register"]
    pub id44: ID44,
    #[doc = "0x348 - Message Buffer 44 WORD0 Register"]
    pub word044: WORD044,
    #[doc = "0x34c - Message Buffer 44 WORD1 Register"]
    pub word144: WORD144,
    #[doc = "0x350 - Message Buffer 45 CS Register"]
    pub cs45: CS45,
    #[doc = "0x354 - Message Buffer 45 ID Register"]
    pub id45: ID45,
    #[doc = "0x358 - Message Buffer 45 WORD0 Register"]
    pub word045: WORD045,
    #[doc = "0x35c - Message Buffer 45 WORD1 Register"]
    pub word145: WORD145,
    #[doc = "0x360 - Message Buffer 46 CS Register"]
    pub cs46: CS46,
    #[doc = "0x364 - Message Buffer 46 ID Register"]
    pub id46: ID46,
    #[doc = "0x368 - Message Buffer 46 WORD0 Register"]
    pub word046: WORD046,
    #[doc = "0x36c - Message Buffer 46 WORD1 Register"]
    pub word146: WORD146,
    #[doc = "0x370 - Message Buffer 47 CS Register"]
    pub cs47: CS47,
    #[doc = "0x374 - Message Buffer 47 ID Register"]
    pub id47: ID47,
    #[doc = "0x378 - Message Buffer 47 WORD0 Register"]
    pub word047: WORD047,
    #[doc = "0x37c - Message Buffer 47 WORD1 Register"]
    pub word147: WORD147,
    #[doc = "0x380 - Message Buffer 48 CS Register"]
    pub cs48: CS48,
    #[doc = "0x384 - Message Buffer 48 ID Register"]
    pub id48: ID48,
    #[doc = "0x388 - Message Buffer 48 WORD0 Register"]
    pub word048: WORD048,
    #[doc = "0x38c - Message Buffer 48 WORD1 Register"]
    pub word148: WORD148,
    #[doc = "0x390 - Message Buffer 49 CS Register"]
    pub cs49: CS49,
    #[doc = "0x394 - Message Buffer 49 ID Register"]
    pub id49: ID49,
    #[doc = "0x398 - Message Buffer 49 WORD0 Register"]
    pub word049: WORD049,
    #[doc = "0x39c - Message Buffer 49 WORD1 Register"]
    pub word149: WORD149,
    #[doc = "0x3a0 - Message Buffer 50 CS Register"]
    pub cs50: CS50,
    #[doc = "0x3a4 - Message Buffer 50 ID Register"]
    pub id50: ID50,
    #[doc = "0x3a8 - Message Buffer 50 WORD0 Register"]
    pub word050: WORD050,
    #[doc = "0x3ac - Message Buffer 50 WORD1 Register"]
    pub word150: WORD150,
    #[doc = "0x3b0 - Message Buffer 51 CS Register"]
    pub cs51: CS51,
    #[doc = "0x3b4 - Message Buffer 51 ID Register"]
    pub id51: ID51,
    #[doc = "0x3b8 - Message Buffer 51 WORD0 Register"]
    pub word051: WORD051,
    #[doc = "0x3bc - Message Buffer 51 WORD1 Register"]
    pub word151: WORD151,
    #[doc = "0x3c0 - Message Buffer 52 CS Register"]
    pub cs52: CS52,
    #[doc = "0x3c4 - Message Buffer 52 ID Register"]
    pub id52: ID52,
    #[doc = "0x3c8 - Message Buffer 52 WORD0 Register"]
    pub word052: WORD052,
    #[doc = "0x3cc - Message Buffer 52 WORD1 Register"]
    pub word152: WORD152,
    #[doc = "0x3d0 - Message Buffer 53 CS Register"]
    pub cs53: CS53,
    #[doc = "0x3d4 - Message Buffer 53 ID Register"]
    pub id53: ID53,
    #[doc = "0x3d8 - Message Buffer 53 WORD0 Register"]
    pub word053: WORD053,
    #[doc = "0x3dc - Message Buffer 53 WORD1 Register"]
    pub word153: WORD153,
    #[doc = "0x3e0 - Message Buffer 54 CS Register"]
    pub cs54: CS54,
    #[doc = "0x3e4 - Message Buffer 54 ID Register"]
    pub id54: ID54,
    #[doc = "0x3e8 - Message Buffer 54 WORD0 Register"]
    pub word054: WORD054,
    #[doc = "0x3ec - Message Buffer 54 WORD1 Register"]
    pub word154: WORD154,
    #[doc = "0x3f0 - Message Buffer 55 CS Register"]
    pub cs55: CS55,
    #[doc = "0x3f4 - Message Buffer 55 ID Register"]
    pub id55: ID55,
    #[doc = "0x3f8 - Message Buffer 55 WORD0 Register"]
    pub word055: WORD055,
    #[doc = "0x3fc - Message Buffer 55 WORD1 Register"]
    pub word155: WORD155,
    #[doc = "0x400 - Message Buffer 56 CS Register"]
    pub cs56: CS56,
    #[doc = "0x404 - Message Buffer 56 ID Register"]
    pub id56: ID56,
    #[doc = "0x408 - Message Buffer 56 WORD0 Register"]
    pub word056: WORD056,
    #[doc = "0x40c - Message Buffer 56 WORD1 Register"]
    pub word156: WORD156,
    #[doc = "0x410 - Message Buffer 57 CS Register"]
    pub cs57: CS57,
    #[doc = "0x414 - Message Buffer 57 ID Register"]
    pub id57: ID57,
    #[doc = "0x418 - Message Buffer 57 WORD0 Register"]
    pub word057: WORD057,
    #[doc = "0x41c - Message Buffer 57 WORD1 Register"]
    pub word157: WORD157,
    #[doc = "0x420 - Message Buffer 58 CS Register"]
    pub cs58: CS58,
    #[doc = "0x424 - Message Buffer 58 ID Register"]
    pub id58: ID58,
    #[doc = "0x428 - Message Buffer 58 WORD0 Register"]
    pub word058: WORD058,
    #[doc = "0x42c - Message Buffer 58 WORD1 Register"]
    pub word158: WORD158,
    #[doc = "0x430 - Message Buffer 59 CS Register"]
    pub cs59: CS59,
    #[doc = "0x434 - Message Buffer 59 ID Register"]
    pub id59: ID59,
    #[doc = "0x438 - Message Buffer 59 WORD0 Register"]
    pub word059: WORD059,
    #[doc = "0x43c - Message Buffer 59 WORD1 Register"]
    pub word159: WORD159,
    #[doc = "0x440 - Message Buffer 60 CS Register"]
    pub cs60: CS60,
    #[doc = "0x444 - Message Buffer 60 ID Register"]
    pub id60: ID60,
    #[doc = "0x448 - Message Buffer 60 WORD0 Register"]
    pub word060: WORD060,
    #[doc = "0x44c - Message Buffer 60 WORD1 Register"]
    pub word160: WORD160,
    #[doc = "0x450 - Message Buffer 61 CS Register"]
    pub cs61: CS61,
    #[doc = "0x454 - Message Buffer 61 ID Register"]
    pub id61: ID61,
    #[doc = "0x458 - Message Buffer 61 WORD0 Register"]
    pub word061: WORD061,
    #[doc = "0x45c - Message Buffer 61 WORD1 Register"]
    pub word161: WORD161,
    #[doc = "0x460 - Message Buffer 62 CS Register"]
    pub cs62: CS62,
    #[doc = "0x464 - Message Buffer 62 ID Register"]
    pub id62: ID62,
    #[doc = "0x468 - Message Buffer 62 WORD0 Register"]
    pub word062: WORD062,
    #[doc = "0x46c - Message Buffer 62 WORD1 Register"]
    pub word162: WORD162,
    #[doc = "0x470 - Message Buffer 63 CS Register"]
    pub cs63: CS63,
    #[doc = "0x474 - Message Buffer 63 ID Register"]
    pub id63: ID63,
    #[doc = "0x478 - Message Buffer 63 WORD0 Register"]
    pub word063: WORD063,
    #[doc = "0x47c - Message Buffer 63 WORD1 Register"]
    pub word163: WORD163,
    _reserved3: [u8; 1024usize],
    #[doc = "0x880 - Rx Individual Mask Registers"]
    pub rximr: [RXIMR; 64],
    _reserved4: [u8; 96usize],
    #[doc = "0x9e0 - Glitch Filter Width Registers"]
    pub gfwr: GFWR,
}
#[doc = "Module Configuration Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "Control 1 Register"]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 1 Register"]
pub mod ctrl1;
#[doc = "Free Running Timer Register"]
pub struct TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Free Running Timer Register"]
pub mod timer;
#[doc = "Rx Mailboxes Global Mask Register"]
pub struct RXMGMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "Rx Buffer 14 Mask Register"]
pub struct RX14MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Buffer 14 Mask Register"]
pub mod rx14mask;
#[doc = "Rx Buffer 15 Mask Register"]
pub struct RX15MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Buffer 15 Mask Register"]
pub mod rx15mask;
#[doc = "Error Counter Register"]
pub struct ECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "Error and Status 1 Register"]
pub struct ESR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error and Status 1 Register"]
pub mod esr1;
#[doc = "Interrupt Masks 2 Register"]
pub struct IMASK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Masks 2 Register"]
pub mod imask2;
#[doc = "Interrupt Masks 1 Register"]
pub struct IMASK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Masks 1 Register"]
pub mod imask1;
#[doc = "Interrupt Flags 2 Register"]
pub struct IFLAG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flags 2 Register"]
pub mod iflag2;
#[doc = "Interrupt Flags 1 Register"]
pub struct IFLAG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flags 1 Register"]
pub mod iflag1;
#[doc = "Control 2 Register"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 2 Register"]
pub mod ctrl2;
#[doc = "Error and Status 2 Register"]
pub struct ESR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error and Status 2 Register"]
pub mod esr2;
#[doc = "CRC Register"]
pub struct CRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "Rx FIFO Global Mask Register"]
pub struct RXFGMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO Global Mask Register"]
pub mod rxfgmask;
#[doc = "Rx FIFO Information Register"]
pub struct RXFIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "Message Buffer 0 CS Register"]
pub struct CS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 0 CS Register"]
pub mod cs0;
#[doc = "Message Buffer 0 ID Register"]
pub struct ID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 0 ID Register"]
pub mod id0;
#[doc = "Message Buffer 0 WORD0 Register"]
pub struct WORD00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 0 WORD0 Register"]
pub mod word00;
#[doc = "Message Buffer 0 WORD1 Register"]
pub struct WORD10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 0 WORD1 Register"]
pub mod word10;
#[doc = "Message Buffer 1 CS Register"]
pub struct CS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 1 CS Register"]
pub mod cs1;
#[doc = "Message Buffer 1 ID Register"]
pub struct ID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 1 ID Register"]
pub mod id1;
#[doc = "Message Buffer 1 WORD0 Register"]
pub struct WORD01 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 1 WORD0 Register"]
pub mod word01;
#[doc = "Message Buffer 1 WORD1 Register"]
pub struct WORD11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 1 WORD1 Register"]
pub mod word11;
#[doc = "Message Buffer 2 CS Register"]
pub struct CS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 2 CS Register"]
pub mod cs2;
#[doc = "Message Buffer 2 ID Register"]
pub struct ID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 2 ID Register"]
pub mod id2;
#[doc = "Message Buffer 2 WORD0 Register"]
pub struct WORD02 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 2 WORD0 Register"]
pub mod word02;
#[doc = "Message Buffer 2 WORD1 Register"]
pub struct WORD12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 2 WORD1 Register"]
pub mod word12;
#[doc = "Message Buffer 3 CS Register"]
pub struct CS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 3 CS Register"]
pub mod cs3;
#[doc = "Message Buffer 3 ID Register"]
pub struct ID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 3 ID Register"]
pub mod id3;
#[doc = "Message Buffer 3 WORD0 Register"]
pub struct WORD03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 3 WORD0 Register"]
pub mod word03;
#[doc = "Message Buffer 3 WORD1 Register"]
pub struct WORD13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 3 WORD1 Register"]
pub mod word13;
#[doc = "Message Buffer 4 CS Register"]
pub struct CS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 4 CS Register"]
pub mod cs4;
#[doc = "Message Buffer 4 ID Register"]
pub struct ID4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 4 ID Register"]
pub mod id4;
#[doc = "Message Buffer 4 WORD0 Register"]
pub struct WORD04 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 4 WORD0 Register"]
pub mod word04;
#[doc = "Message Buffer 4 WORD1 Register"]
pub struct WORD14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 4 WORD1 Register"]
pub mod word14;
#[doc = "Message Buffer 5 CS Register"]
pub struct CS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 5 CS Register"]
pub mod cs5;
#[doc = "Message Buffer 5 ID Register"]
pub struct ID5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 5 ID Register"]
pub mod id5;
#[doc = "Message Buffer 5 WORD0 Register"]
pub struct WORD05 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 5 WORD0 Register"]
pub mod word05;
#[doc = "Message Buffer 5 WORD1 Register"]
pub struct WORD15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 5 WORD1 Register"]
pub mod word15;
#[doc = "Message Buffer 6 CS Register"]
pub struct CS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 6 CS Register"]
pub mod cs6;
#[doc = "Message Buffer 6 ID Register"]
pub struct ID6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 6 ID Register"]
pub mod id6;
#[doc = "Message Buffer 6 WORD0 Register"]
pub struct WORD06 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 6 WORD0 Register"]
pub mod word06;
#[doc = "Message Buffer 6 WORD1 Register"]
pub struct WORD16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 6 WORD1 Register"]
pub mod word16;
#[doc = "Message Buffer 7 CS Register"]
pub struct CS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 7 CS Register"]
pub mod cs7;
#[doc = "Message Buffer 7 ID Register"]
pub struct ID7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 7 ID Register"]
pub mod id7;
#[doc = "Message Buffer 7 WORD0 Register"]
pub struct WORD07 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 7 WORD0 Register"]
pub mod word07;
#[doc = "Message Buffer 7 WORD1 Register"]
pub struct WORD17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 7 WORD1 Register"]
pub mod word17;
#[doc = "Message Buffer 8 CS Register"]
pub struct CS8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 8 CS Register"]
pub mod cs8;
#[doc = "Message Buffer 8 ID Register"]
pub struct ID8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 8 ID Register"]
pub mod id8;
#[doc = "Message Buffer 8 WORD0 Register"]
pub struct WORD08 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 8 WORD0 Register"]
pub mod word08;
#[doc = "Message Buffer 8 WORD1 Register"]
pub struct WORD18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 8 WORD1 Register"]
pub mod word18;
#[doc = "Message Buffer 9 CS Register"]
pub struct CS9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 9 CS Register"]
pub mod cs9;
#[doc = "Message Buffer 9 ID Register"]
pub struct ID9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 9 ID Register"]
pub mod id9;
#[doc = "Message Buffer 9 WORD0 Register"]
pub struct WORD09 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 9 WORD0 Register"]
pub mod word09;
#[doc = "Message Buffer 9 WORD1 Register"]
pub struct WORD19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 9 WORD1 Register"]
pub mod word19;
#[doc = "Message Buffer 10 CS Register"]
pub struct CS10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 10 CS Register"]
pub mod cs10;
#[doc = "Message Buffer 10 ID Register"]
pub struct ID10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 10 ID Register"]
pub mod id10;
#[doc = "Message Buffer 10 WORD0 Register"]
pub struct WORD010 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 10 WORD0 Register"]
pub mod word010;
#[doc = "Message Buffer 10 WORD1 Register"]
pub struct WORD110 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 10 WORD1 Register"]
pub mod word110;
#[doc = "Message Buffer 11 CS Register"]
pub struct CS11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 11 CS Register"]
pub mod cs11;
#[doc = "Message Buffer 11 ID Register"]
pub struct ID11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 11 ID Register"]
pub mod id11;
#[doc = "Message Buffer 11 WORD0 Register"]
pub struct WORD011 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 11 WORD0 Register"]
pub mod word011;
#[doc = "Message Buffer 11 WORD1 Register"]
pub struct WORD111 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 11 WORD1 Register"]
pub mod word111;
#[doc = "Message Buffer 12 CS Register"]
pub struct CS12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 12 CS Register"]
pub mod cs12;
#[doc = "Message Buffer 12 ID Register"]
pub struct ID12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 12 ID Register"]
pub mod id12;
#[doc = "Message Buffer 12 WORD0 Register"]
pub struct WORD012 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 12 WORD0 Register"]
pub mod word012;
#[doc = "Message Buffer 12 WORD1 Register"]
pub struct WORD112 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 12 WORD1 Register"]
pub mod word112;
#[doc = "Message Buffer 13 CS Register"]
pub struct CS13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 13 CS Register"]
pub mod cs13;
#[doc = "Message Buffer 13 ID Register"]
pub struct ID13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 13 ID Register"]
pub mod id13;
#[doc = "Message Buffer 13 WORD0 Register"]
pub struct WORD013 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 13 WORD0 Register"]
pub mod word013;
#[doc = "Message Buffer 13 WORD1 Register"]
pub struct WORD113 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 13 WORD1 Register"]
pub mod word113;
#[doc = "Message Buffer 14 CS Register"]
pub struct CS14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 14 CS Register"]
pub mod cs14;
#[doc = "Message Buffer 14 ID Register"]
pub struct ID14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 14 ID Register"]
pub mod id14;
#[doc = "Message Buffer 14 WORD0 Register"]
pub struct WORD014 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 14 WORD0 Register"]
pub mod word014;
#[doc = "Message Buffer 14 WORD1 Register"]
pub struct WORD114 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 14 WORD1 Register"]
pub mod word114;
#[doc = "Message Buffer 15 CS Register"]
pub struct CS15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 15 CS Register"]
pub mod cs15;
#[doc = "Message Buffer 15 ID Register"]
pub struct ID15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 15 ID Register"]
pub mod id15;
#[doc = "Message Buffer 15 WORD0 Register"]
pub struct WORD015 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 15 WORD0 Register"]
pub mod word015;
#[doc = "Message Buffer 15 WORD1 Register"]
pub struct WORD115 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 15 WORD1 Register"]
pub mod word115;
#[doc = "Message Buffer 16 CS Register"]
pub struct CS16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 16 CS Register"]
pub mod cs16;
#[doc = "Message Buffer 16 ID Register"]
pub struct ID16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 16 ID Register"]
pub mod id16;
#[doc = "Message Buffer 16 WORD0 Register"]
pub struct WORD016 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 16 WORD0 Register"]
pub mod word016;
#[doc = "Message Buffer 16 WORD1 Register"]
pub struct WORD116 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 16 WORD1 Register"]
pub mod word116;
#[doc = "Message Buffer 17 CS Register"]
pub struct CS17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 17 CS Register"]
pub mod cs17;
#[doc = "Message Buffer 17 ID Register"]
pub struct ID17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 17 ID Register"]
pub mod id17;
#[doc = "Message Buffer 17 WORD0 Register"]
pub struct WORD017 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 17 WORD0 Register"]
pub mod word017;
#[doc = "Message Buffer 17 WORD1 Register"]
pub struct WORD117 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 17 WORD1 Register"]
pub mod word117;
#[doc = "Message Buffer 18 CS Register"]
pub struct CS18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 18 CS Register"]
pub mod cs18;
#[doc = "Message Buffer 18 ID Register"]
pub struct ID18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 18 ID Register"]
pub mod id18;
#[doc = "Message Buffer 18 WORD0 Register"]
pub struct WORD018 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 18 WORD0 Register"]
pub mod word018;
#[doc = "Message Buffer 18 WORD1 Register"]
pub struct WORD118 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 18 WORD1 Register"]
pub mod word118;
#[doc = "Message Buffer 19 CS Register"]
pub struct CS19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 19 CS Register"]
pub mod cs19;
#[doc = "Message Buffer 19 ID Register"]
pub struct ID19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 19 ID Register"]
pub mod id19;
#[doc = "Message Buffer 19 WORD0 Register"]
pub struct WORD019 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 19 WORD0 Register"]
pub mod word019;
#[doc = "Message Buffer 19 WORD1 Register"]
pub struct WORD119 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 19 WORD1 Register"]
pub mod word119;
#[doc = "Message Buffer 20 CS Register"]
pub struct CS20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 20 CS Register"]
pub mod cs20;
#[doc = "Message Buffer 20 ID Register"]
pub struct ID20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 20 ID Register"]
pub mod id20;
#[doc = "Message Buffer 20 WORD0 Register"]
pub struct WORD020 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 20 WORD0 Register"]
pub mod word020;
#[doc = "Message Buffer 20 WORD1 Register"]
pub struct WORD120 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 20 WORD1 Register"]
pub mod word120;
#[doc = "Message Buffer 21 CS Register"]
pub struct CS21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 21 CS Register"]
pub mod cs21;
#[doc = "Message Buffer 21 ID Register"]
pub struct ID21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 21 ID Register"]
pub mod id21;
#[doc = "Message Buffer 21 WORD0 Register"]
pub struct WORD021 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 21 WORD0 Register"]
pub mod word021;
#[doc = "Message Buffer 21 WORD1 Register"]
pub struct WORD121 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 21 WORD1 Register"]
pub mod word121;
#[doc = "Message Buffer 22 CS Register"]
pub struct CS22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 22 CS Register"]
pub mod cs22;
#[doc = "Message Buffer 22 ID Register"]
pub struct ID22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 22 ID Register"]
pub mod id22;
#[doc = "Message Buffer 22 WORD0 Register"]
pub struct WORD022 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 22 WORD0 Register"]
pub mod word022;
#[doc = "Message Buffer 22 WORD1 Register"]
pub struct WORD122 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 22 WORD1 Register"]
pub mod word122;
#[doc = "Message Buffer 23 CS Register"]
pub struct CS23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 23 CS Register"]
pub mod cs23;
#[doc = "Message Buffer 23 ID Register"]
pub struct ID23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 23 ID Register"]
pub mod id23;
#[doc = "Message Buffer 23 WORD0 Register"]
pub struct WORD023 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 23 WORD0 Register"]
pub mod word023;
#[doc = "Message Buffer 23 WORD1 Register"]
pub struct WORD123 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 23 WORD1 Register"]
pub mod word123;
#[doc = "Message Buffer 24 CS Register"]
pub struct CS24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 24 CS Register"]
pub mod cs24;
#[doc = "Message Buffer 24 ID Register"]
pub struct ID24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 24 ID Register"]
pub mod id24;
#[doc = "Message Buffer 24 WORD0 Register"]
pub struct WORD024 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 24 WORD0 Register"]
pub mod word024;
#[doc = "Message Buffer 24 WORD1 Register"]
pub struct WORD124 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 24 WORD1 Register"]
pub mod word124;
#[doc = "Message Buffer 25 CS Register"]
pub struct CS25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 25 CS Register"]
pub mod cs25;
#[doc = "Message Buffer 25 ID Register"]
pub struct ID25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 25 ID Register"]
pub mod id25;
#[doc = "Message Buffer 25 WORD0 Register"]
pub struct WORD025 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 25 WORD0 Register"]
pub mod word025;
#[doc = "Message Buffer 25 WORD1 Register"]
pub struct WORD125 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 25 WORD1 Register"]
pub mod word125;
#[doc = "Message Buffer 26 CS Register"]
pub struct CS26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 26 CS Register"]
pub mod cs26;
#[doc = "Message Buffer 26 ID Register"]
pub struct ID26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 26 ID Register"]
pub mod id26;
#[doc = "Message Buffer 26 WORD0 Register"]
pub struct WORD026 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 26 WORD0 Register"]
pub mod word026;
#[doc = "Message Buffer 26 WORD1 Register"]
pub struct WORD126 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 26 WORD1 Register"]
pub mod word126;
#[doc = "Message Buffer 27 CS Register"]
pub struct CS27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 27 CS Register"]
pub mod cs27;
#[doc = "Message Buffer 27 ID Register"]
pub struct ID27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 27 ID Register"]
pub mod id27;
#[doc = "Message Buffer 27 WORD0 Register"]
pub struct WORD027 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 27 WORD0 Register"]
pub mod word027;
#[doc = "Message Buffer 27 WORD1 Register"]
pub struct WORD127 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 27 WORD1 Register"]
pub mod word127;
#[doc = "Message Buffer 28 CS Register"]
pub struct CS28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 28 CS Register"]
pub mod cs28;
#[doc = "Message Buffer 28 ID Register"]
pub struct ID28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 28 ID Register"]
pub mod id28;
#[doc = "Message Buffer 28 WORD0 Register"]
pub struct WORD028 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 28 WORD0 Register"]
pub mod word028;
#[doc = "Message Buffer 28 WORD1 Register"]
pub struct WORD128 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 28 WORD1 Register"]
pub mod word128;
#[doc = "Message Buffer 29 CS Register"]
pub struct CS29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 29 CS Register"]
pub mod cs29;
#[doc = "Message Buffer 29 ID Register"]
pub struct ID29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 29 ID Register"]
pub mod id29;
#[doc = "Message Buffer 29 WORD0 Register"]
pub struct WORD029 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 29 WORD0 Register"]
pub mod word029;
#[doc = "Message Buffer 29 WORD1 Register"]
pub struct WORD129 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 29 WORD1 Register"]
pub mod word129;
#[doc = "Message Buffer 30 CS Register"]
pub struct CS30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 30 CS Register"]
pub mod cs30;
#[doc = "Message Buffer 30 ID Register"]
pub struct ID30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 30 ID Register"]
pub mod id30;
#[doc = "Message Buffer 30 WORD0 Register"]
pub struct WORD030 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 30 WORD0 Register"]
pub mod word030;
#[doc = "Message Buffer 30 WORD1 Register"]
pub struct WORD130 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 30 WORD1 Register"]
pub mod word130;
#[doc = "Message Buffer 31 CS Register"]
pub struct CS31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 31 CS Register"]
pub mod cs31;
#[doc = "Message Buffer 31 ID Register"]
pub struct ID31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 31 ID Register"]
pub mod id31;
#[doc = "Message Buffer 31 WORD0 Register"]
pub struct WORD031 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 31 WORD0 Register"]
pub mod word031;
#[doc = "Message Buffer 31 WORD1 Register"]
pub struct WORD131 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 31 WORD1 Register"]
pub mod word131;
#[doc = "Message Buffer 32 CS Register"]
pub struct CS32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 32 CS Register"]
pub mod cs32;
#[doc = "Message Buffer 32 ID Register"]
pub struct ID32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 32 ID Register"]
pub mod id32;
#[doc = "Message Buffer 32 WORD0 Register"]
pub struct WORD032 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 32 WORD0 Register"]
pub mod word032;
#[doc = "Message Buffer 32 WORD1 Register"]
pub struct WORD132 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 32 WORD1 Register"]
pub mod word132;
#[doc = "Message Buffer 33 CS Register"]
pub struct CS33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 33 CS Register"]
pub mod cs33;
#[doc = "Message Buffer 33 ID Register"]
pub struct ID33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 33 ID Register"]
pub mod id33;
#[doc = "Message Buffer 33 WORD0 Register"]
pub struct WORD033 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 33 WORD0 Register"]
pub mod word033;
#[doc = "Message Buffer 33 WORD1 Register"]
pub struct WORD133 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 33 WORD1 Register"]
pub mod word133;
#[doc = "Message Buffer 34 CS Register"]
pub struct CS34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 34 CS Register"]
pub mod cs34;
#[doc = "Message Buffer 34 ID Register"]
pub struct ID34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 34 ID Register"]
pub mod id34;
#[doc = "Message Buffer 34 WORD0 Register"]
pub struct WORD034 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 34 WORD0 Register"]
pub mod word034;
#[doc = "Message Buffer 34 WORD1 Register"]
pub struct WORD134 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 34 WORD1 Register"]
pub mod word134;
#[doc = "Message Buffer 35 CS Register"]
pub struct CS35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 35 CS Register"]
pub mod cs35;
#[doc = "Message Buffer 35 ID Register"]
pub struct ID35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 35 ID Register"]
pub mod id35;
#[doc = "Message Buffer 35 WORD0 Register"]
pub struct WORD035 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 35 WORD0 Register"]
pub mod word035;
#[doc = "Message Buffer 35 WORD1 Register"]
pub struct WORD135 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 35 WORD1 Register"]
pub mod word135;
#[doc = "Message Buffer 36 CS Register"]
pub struct CS36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 36 CS Register"]
pub mod cs36;
#[doc = "Message Buffer 36 ID Register"]
pub struct ID36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 36 ID Register"]
pub mod id36;
#[doc = "Message Buffer 36 WORD0 Register"]
pub struct WORD036 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 36 WORD0 Register"]
pub mod word036;
#[doc = "Message Buffer 36 WORD1 Register"]
pub struct WORD136 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 36 WORD1 Register"]
pub mod word136;
#[doc = "Message Buffer 37 CS Register"]
pub struct CS37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 37 CS Register"]
pub mod cs37;
#[doc = "Message Buffer 37 ID Register"]
pub struct ID37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 37 ID Register"]
pub mod id37;
#[doc = "Message Buffer 37 WORD0 Register"]
pub struct WORD037 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 37 WORD0 Register"]
pub mod word037;
#[doc = "Message Buffer 37 WORD1 Register"]
pub struct WORD137 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 37 WORD1 Register"]
pub mod word137;
#[doc = "Message Buffer 38 CS Register"]
pub struct CS38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 38 CS Register"]
pub mod cs38;
#[doc = "Message Buffer 38 ID Register"]
pub struct ID38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 38 ID Register"]
pub mod id38;
#[doc = "Message Buffer 38 WORD0 Register"]
pub struct WORD038 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 38 WORD0 Register"]
pub mod word038;
#[doc = "Message Buffer 38 WORD1 Register"]
pub struct WORD138 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 38 WORD1 Register"]
pub mod word138;
#[doc = "Message Buffer 39 CS Register"]
pub struct CS39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 39 CS Register"]
pub mod cs39;
#[doc = "Message Buffer 39 ID Register"]
pub struct ID39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 39 ID Register"]
pub mod id39;
#[doc = "Message Buffer 39 WORD0 Register"]
pub struct WORD039 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 39 WORD0 Register"]
pub mod word039;
#[doc = "Message Buffer 39 WORD1 Register"]
pub struct WORD139 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 39 WORD1 Register"]
pub mod word139;
#[doc = "Message Buffer 40 CS Register"]
pub struct CS40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 40 CS Register"]
pub mod cs40;
#[doc = "Message Buffer 40 ID Register"]
pub struct ID40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 40 ID Register"]
pub mod id40;
#[doc = "Message Buffer 40 WORD0 Register"]
pub struct WORD040 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 40 WORD0 Register"]
pub mod word040;
#[doc = "Message Buffer 40 WORD1 Register"]
pub struct WORD140 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 40 WORD1 Register"]
pub mod word140;
#[doc = "Message Buffer 41 CS Register"]
pub struct CS41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 41 CS Register"]
pub mod cs41;
#[doc = "Message Buffer 41 ID Register"]
pub struct ID41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 41 ID Register"]
pub mod id41;
#[doc = "Message Buffer 41 WORD0 Register"]
pub struct WORD041 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 41 WORD0 Register"]
pub mod word041;
#[doc = "Message Buffer 41 WORD1 Register"]
pub struct WORD141 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 41 WORD1 Register"]
pub mod word141;
#[doc = "Message Buffer 42 CS Register"]
pub struct CS42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 42 CS Register"]
pub mod cs42;
#[doc = "Message Buffer 42 ID Register"]
pub struct ID42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 42 ID Register"]
pub mod id42;
#[doc = "Message Buffer 42 WORD0 Register"]
pub struct WORD042 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 42 WORD0 Register"]
pub mod word042;
#[doc = "Message Buffer 42 WORD1 Register"]
pub struct WORD142 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 42 WORD1 Register"]
pub mod word142;
#[doc = "Message Buffer 43 CS Register"]
pub struct CS43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 43 CS Register"]
pub mod cs43;
#[doc = "Message Buffer 43 ID Register"]
pub struct ID43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 43 ID Register"]
pub mod id43;
#[doc = "Message Buffer 43 WORD0 Register"]
pub struct WORD043 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 43 WORD0 Register"]
pub mod word043;
#[doc = "Message Buffer 43 WORD1 Register"]
pub struct WORD143 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 43 WORD1 Register"]
pub mod word143;
#[doc = "Message Buffer 44 CS Register"]
pub struct CS44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 44 CS Register"]
pub mod cs44;
#[doc = "Message Buffer 44 ID Register"]
pub struct ID44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 44 ID Register"]
pub mod id44;
#[doc = "Message Buffer 44 WORD0 Register"]
pub struct WORD044 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 44 WORD0 Register"]
pub mod word044;
#[doc = "Message Buffer 44 WORD1 Register"]
pub struct WORD144 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 44 WORD1 Register"]
pub mod word144;
#[doc = "Message Buffer 45 CS Register"]
pub struct CS45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 45 CS Register"]
pub mod cs45;
#[doc = "Message Buffer 45 ID Register"]
pub struct ID45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 45 ID Register"]
pub mod id45;
#[doc = "Message Buffer 45 WORD0 Register"]
pub struct WORD045 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 45 WORD0 Register"]
pub mod word045;
#[doc = "Message Buffer 45 WORD1 Register"]
pub struct WORD145 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 45 WORD1 Register"]
pub mod word145;
#[doc = "Message Buffer 46 CS Register"]
pub struct CS46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 46 CS Register"]
pub mod cs46;
#[doc = "Message Buffer 46 ID Register"]
pub struct ID46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 46 ID Register"]
pub mod id46;
#[doc = "Message Buffer 46 WORD0 Register"]
pub struct WORD046 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 46 WORD0 Register"]
pub mod word046;
#[doc = "Message Buffer 46 WORD1 Register"]
pub struct WORD146 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 46 WORD1 Register"]
pub mod word146;
#[doc = "Message Buffer 47 CS Register"]
pub struct CS47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 47 CS Register"]
pub mod cs47;
#[doc = "Message Buffer 47 ID Register"]
pub struct ID47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 47 ID Register"]
pub mod id47;
#[doc = "Message Buffer 47 WORD0 Register"]
pub struct WORD047 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 47 WORD0 Register"]
pub mod word047;
#[doc = "Message Buffer 47 WORD1 Register"]
pub struct WORD147 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 47 WORD1 Register"]
pub mod word147;
#[doc = "Message Buffer 48 CS Register"]
pub struct CS48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 48 CS Register"]
pub mod cs48;
#[doc = "Message Buffer 48 ID Register"]
pub struct ID48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 48 ID Register"]
pub mod id48;
#[doc = "Message Buffer 48 WORD0 Register"]
pub struct WORD048 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 48 WORD0 Register"]
pub mod word048;
#[doc = "Message Buffer 48 WORD1 Register"]
pub struct WORD148 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 48 WORD1 Register"]
pub mod word148;
#[doc = "Message Buffer 49 CS Register"]
pub struct CS49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 49 CS Register"]
pub mod cs49;
#[doc = "Message Buffer 49 ID Register"]
pub struct ID49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 49 ID Register"]
pub mod id49;
#[doc = "Message Buffer 49 WORD0 Register"]
pub struct WORD049 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 49 WORD0 Register"]
pub mod word049;
#[doc = "Message Buffer 49 WORD1 Register"]
pub struct WORD149 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 49 WORD1 Register"]
pub mod word149;
#[doc = "Message Buffer 50 CS Register"]
pub struct CS50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 50 CS Register"]
pub mod cs50;
#[doc = "Message Buffer 50 ID Register"]
pub struct ID50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 50 ID Register"]
pub mod id50;
#[doc = "Message Buffer 50 WORD0 Register"]
pub struct WORD050 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 50 WORD0 Register"]
pub mod word050;
#[doc = "Message Buffer 50 WORD1 Register"]
pub struct WORD150 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 50 WORD1 Register"]
pub mod word150;
#[doc = "Message Buffer 51 CS Register"]
pub struct CS51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 51 CS Register"]
pub mod cs51;
#[doc = "Message Buffer 51 ID Register"]
pub struct ID51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 51 ID Register"]
pub mod id51;
#[doc = "Message Buffer 51 WORD0 Register"]
pub struct WORD051 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 51 WORD0 Register"]
pub mod word051;
#[doc = "Message Buffer 51 WORD1 Register"]
pub struct WORD151 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 51 WORD1 Register"]
pub mod word151;
#[doc = "Message Buffer 52 CS Register"]
pub struct CS52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 52 CS Register"]
pub mod cs52;
#[doc = "Message Buffer 52 ID Register"]
pub struct ID52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 52 ID Register"]
pub mod id52;
#[doc = "Message Buffer 52 WORD0 Register"]
pub struct WORD052 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 52 WORD0 Register"]
pub mod word052;
#[doc = "Message Buffer 52 WORD1 Register"]
pub struct WORD152 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 52 WORD1 Register"]
pub mod word152;
#[doc = "Message Buffer 53 CS Register"]
pub struct CS53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 53 CS Register"]
pub mod cs53;
#[doc = "Message Buffer 53 ID Register"]
pub struct ID53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 53 ID Register"]
pub mod id53;
#[doc = "Message Buffer 53 WORD0 Register"]
pub struct WORD053 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 53 WORD0 Register"]
pub mod word053;
#[doc = "Message Buffer 53 WORD1 Register"]
pub struct WORD153 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 53 WORD1 Register"]
pub mod word153;
#[doc = "Message Buffer 54 CS Register"]
pub struct CS54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 54 CS Register"]
pub mod cs54;
#[doc = "Message Buffer 54 ID Register"]
pub struct ID54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 54 ID Register"]
pub mod id54;
#[doc = "Message Buffer 54 WORD0 Register"]
pub struct WORD054 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 54 WORD0 Register"]
pub mod word054;
#[doc = "Message Buffer 54 WORD1 Register"]
pub struct WORD154 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 54 WORD1 Register"]
pub mod word154;
#[doc = "Message Buffer 55 CS Register"]
pub struct CS55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 55 CS Register"]
pub mod cs55;
#[doc = "Message Buffer 55 ID Register"]
pub struct ID55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 55 ID Register"]
pub mod id55;
#[doc = "Message Buffer 55 WORD0 Register"]
pub struct WORD055 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 55 WORD0 Register"]
pub mod word055;
#[doc = "Message Buffer 55 WORD1 Register"]
pub struct WORD155 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 55 WORD1 Register"]
pub mod word155;
#[doc = "Message Buffer 56 CS Register"]
pub struct CS56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 56 CS Register"]
pub mod cs56;
#[doc = "Message Buffer 56 ID Register"]
pub struct ID56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 56 ID Register"]
pub mod id56;
#[doc = "Message Buffer 56 WORD0 Register"]
pub struct WORD056 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 56 WORD0 Register"]
pub mod word056;
#[doc = "Message Buffer 56 WORD1 Register"]
pub struct WORD156 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 56 WORD1 Register"]
pub mod word156;
#[doc = "Message Buffer 57 CS Register"]
pub struct CS57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 57 CS Register"]
pub mod cs57;
#[doc = "Message Buffer 57 ID Register"]
pub struct ID57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 57 ID Register"]
pub mod id57;
#[doc = "Message Buffer 57 WORD0 Register"]
pub struct WORD057 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 57 WORD0 Register"]
pub mod word057;
#[doc = "Message Buffer 57 WORD1 Register"]
pub struct WORD157 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 57 WORD1 Register"]
pub mod word157;
#[doc = "Message Buffer 58 CS Register"]
pub struct CS58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 58 CS Register"]
pub mod cs58;
#[doc = "Message Buffer 58 ID Register"]
pub struct ID58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 58 ID Register"]
pub mod id58;
#[doc = "Message Buffer 58 WORD0 Register"]
pub struct WORD058 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 58 WORD0 Register"]
pub mod word058;
#[doc = "Message Buffer 58 WORD1 Register"]
pub struct WORD158 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 58 WORD1 Register"]
pub mod word158;
#[doc = "Message Buffer 59 CS Register"]
pub struct CS59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 59 CS Register"]
pub mod cs59;
#[doc = "Message Buffer 59 ID Register"]
pub struct ID59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 59 ID Register"]
pub mod id59;
#[doc = "Message Buffer 59 WORD0 Register"]
pub struct WORD059 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 59 WORD0 Register"]
pub mod word059;
#[doc = "Message Buffer 59 WORD1 Register"]
pub struct WORD159 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 59 WORD1 Register"]
pub mod word159;
#[doc = "Message Buffer 60 CS Register"]
pub struct CS60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 60 CS Register"]
pub mod cs60;
#[doc = "Message Buffer 60 ID Register"]
pub struct ID60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 60 ID Register"]
pub mod id60;
#[doc = "Message Buffer 60 WORD0 Register"]
pub struct WORD060 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 60 WORD0 Register"]
pub mod word060;
#[doc = "Message Buffer 60 WORD1 Register"]
pub struct WORD160 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 60 WORD1 Register"]
pub mod word160;
#[doc = "Message Buffer 61 CS Register"]
pub struct CS61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 61 CS Register"]
pub mod cs61;
#[doc = "Message Buffer 61 ID Register"]
pub struct ID61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 61 ID Register"]
pub mod id61;
#[doc = "Message Buffer 61 WORD0 Register"]
pub struct WORD061 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 61 WORD0 Register"]
pub mod word061;
#[doc = "Message Buffer 61 WORD1 Register"]
pub struct WORD161 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 61 WORD1 Register"]
pub mod word161;
#[doc = "Message Buffer 62 CS Register"]
pub struct CS62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 62 CS Register"]
pub mod cs62;
#[doc = "Message Buffer 62 ID Register"]
pub struct ID62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 62 ID Register"]
pub mod id62;
#[doc = "Message Buffer 62 WORD0 Register"]
pub struct WORD062 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 62 WORD0 Register"]
pub mod word062;
#[doc = "Message Buffer 62 WORD1 Register"]
pub struct WORD162 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 62 WORD1 Register"]
pub mod word162;
#[doc = "Message Buffer 63 CS Register"]
pub struct CS63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 63 CS Register"]
pub mod cs63;
#[doc = "Message Buffer 63 ID Register"]
pub struct ID63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 63 ID Register"]
pub mod id63;
#[doc = "Message Buffer 63 WORD0 Register"]
pub struct WORD063 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 63 WORD0 Register"]
pub mod word063;
#[doc = "Message Buffer 63 WORD1 Register"]
pub struct WORD163 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Buffer 63 WORD1 Register"]
pub mod word163;
#[doc = "Rx Individual Mask Registers"]
pub struct RXIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Individual Mask Registers"]
pub mod rximr;
#[doc = "Glitch Filter Width Registers"]
pub struct GFWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Glitch Filter Width Registers"]
pub mod gfwr;
