#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Interrupt Event Register"]
    pub eir: EIR,
    #[doc = "0x08 - Interrupt Mask Register"]
    pub eimr: EIMR,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - Receive Descriptor Active Register"]
    pub rdar: RDAR,
    #[doc = "0x14 - Transmit Descriptor Active Register"]
    pub tdar: TDAR,
    _reserved2: [u8; 12usize],
    #[doc = "0x24 - Ethernet Control Register"]
    pub ecr: ECR,
    _reserved3: [u8; 24usize],
    #[doc = "0x40 - MII Management Frame Register"]
    pub mmfr: MMFR,
    #[doc = "0x44 - MII Speed Control Register"]
    pub mscr: MSCR,
    _reserved4: [u8; 28usize],
    #[doc = "0x64 - MIB Control Register"]
    pub mibc: MIBC,
    _reserved5: [u8; 28usize],
    #[doc = "0x84 - Receive Control Register"]
    pub rcr: RCR,
    _reserved6: [u8; 60usize],
    #[doc = "0xc4 - Transmit Control Register"]
    pub tcr: TCR,
    _reserved7: [u8; 28usize],
    #[doc = "0xe4 - Physical Address Lower Register"]
    pub palr: PALR,
    #[doc = "0xe8 - Physical Address Upper Register"]
    pub paur: PAUR,
    #[doc = "0xec - Opcode/Pause Duration Register"]
    pub opd: OPD,
    #[doc = "0xf0 - Transmit Interrupt Coalescing Register"]
    pub txic: TXIC,
    _reserved8: [u8; 12usize],
    #[doc = "0x100 - Receive Interrupt Coalescing Register"]
    pub rxic: RXIC,
    _reserved9: [u8; 20usize],
    #[doc = "0x118 - Descriptor Individual Upper Address Register"]
    pub iaur: IAUR,
    #[doc = "0x11c - Descriptor Individual Lower Address Register"]
    pub ialr: IALR,
    #[doc = "0x120 - Descriptor Group Upper Address Register"]
    pub gaur: GAUR,
    #[doc = "0x124 - Descriptor Group Lower Address Register"]
    pub galr: GALR,
    _reserved10: [u8; 28usize],
    #[doc = "0x144 - Transmit FIFO Watermark Register"]
    pub tfwr: TFWR,
    _reserved11: [u8; 56usize],
    #[doc = "0x180 - Receive Descriptor Ring Start Register"]
    pub rdsr: RDSR,
    #[doc = "0x184 - Transmit Buffer Descriptor Ring Start Register"]
    pub tdsr: TDSR,
    #[doc = "0x188 - Maximum Receive Buffer Size Register"]
    pub mrbr: MRBR,
    _reserved12: [u8; 4usize],
    #[doc = "0x190 - Receive FIFO Section Full Threshold"]
    pub rsfl: RSFL,
    #[doc = "0x194 - Receive FIFO Section Empty Threshold"]
    pub rsem: RSEM,
    #[doc = "0x198 - Receive FIFO Almost Empty Threshold"]
    pub raem: RAEM,
    #[doc = "0x19c - Receive FIFO Almost Full Threshold"]
    pub rafl: RAFL,
    #[doc = "0x1a0 - Transmit FIFO Section Empty Threshold"]
    pub tsem: TSEM,
    #[doc = "0x1a4 - Transmit FIFO Almost Empty Threshold"]
    pub taem: TAEM,
    #[doc = "0x1a8 - Transmit FIFO Almost Full Threshold"]
    pub tafl: TAFL,
    #[doc = "0x1ac - Transmit Inter-Packet Gap"]
    pub tipg: TIPG,
    #[doc = "0x1b0 - Frame Truncation Length"]
    pub ftrl: FTRL,
    _reserved13: [u8; 12usize],
    #[doc = "0x1c0 - Transmit Accelerator Function Configuration"]
    pub tacc: TACC,
    #[doc = "0x1c4 - Receive Accelerator Function Configuration"]
    pub racc: RACC,
    _reserved14: [u8; 56usize],
    #[doc = "0x200 - Reserved Statistic Register"]
    pub rmon_t_drop: RMON_T_DROP,
    #[doc = "0x204 - Tx Packet Count Statistic Register"]
    pub rmon_t_packets: RMON_T_PACKETS,
    #[doc = "0x208 - Tx Broadcast Packets Statistic Register"]
    pub rmon_t_bc_pkt: RMON_T_BC_PKT,
    #[doc = "0x20c - Tx Multicast Packets Statistic Register"]
    pub rmon_t_mc_pkt: RMON_T_MC_PKT,
    #[doc = "0x210 - Tx Packets with CRC/Align Error Statistic Register"]
    pub rmon_t_crc_align: RMON_T_CRC_ALIGN,
    #[doc = "0x214 - Tx Packets Less Than Bytes and Good CRC Statistic Register"]
    pub rmon_t_undersize: RMON_T_UNDERSIZE,
    #[doc = "0x218 - Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
    pub rmon_t_oversize: RMON_T_OVERSIZE,
    #[doc = "0x21c - Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub rmon_t_frag: RMON_T_FRAG,
    #[doc = "0x220 - Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
    pub rmon_t_jab: RMON_T_JAB,
    #[doc = "0x224 - Tx Collision Count Statistic Register"]
    pub rmon_t_col: RMON_T_COL,
    #[doc = "0x228 - Tx 64-Byte Packets Statistic Register"]
    pub rmon_t_p64: RMON_T_P64,
    #[doc = "0x22c - Tx 65- to 127-byte Packets Statistic Register"]
    pub rmon_t_p65to127: RMON_T_P65TO127,
    #[doc = "0x230 - Tx 128- to 255-byte Packets Statistic Register"]
    pub rmon_t_p128to255: RMON_T_P128TO255,
    #[doc = "0x234 - Tx 256- to 511-byte Packets Statistic Register"]
    pub rmon_t_p256to511: RMON_T_P256TO511,
    #[doc = "0x238 - Tx 512- to 1023-byte Packets Statistic Register"]
    pub rmon_t_p512to1023: RMON_T_P512TO1023,
    #[doc = "0x23c - Tx 1024- to 2047-byte Packets Statistic Register"]
    pub rmon_t_p1024to2047: RMON_T_P1024TO2047,
    #[doc = "0x240 - Tx Packets Greater Than 2048 Bytes Statistic Register"]
    pub rmon_t_p_gte2048: RMON_T_P_GTE2048,
    #[doc = "0x244 - Tx Octets Statistic Register"]
    pub rmon_t_octets: RMON_T_OCTETS,
    #[doc = "0x248 - Reserved Statistic Register"]
    pub ieee_t_drop: IEEE_T_DROP,
    #[doc = "0x24c - Frames Transmitted OK Statistic Register"]
    pub ieee_t_frame_ok: IEEE_T_FRAME_OK,
    #[doc = "0x250 - Frames Transmitted with Single Collision Statistic Register"]
    pub ieee_t_1col: IEEE_T_1COL,
    #[doc = "0x254 - Frames Transmitted with Multiple Collisions Statistic Register"]
    pub ieee_t_mcol: IEEE_T_MCOL,
    #[doc = "0x258 - Frames Transmitted after Deferral Delay Statistic Register"]
    pub ieee_t_def: IEEE_T_DEF,
    #[doc = "0x25c - Frames Transmitted with Late Collision Statistic Register"]
    pub ieee_t_lcol: IEEE_T_LCOL,
    #[doc = "0x260 - Frames Transmitted with Excessive Collisions Statistic Register"]
    pub ieee_t_excol: IEEE_T_EXCOL,
    #[doc = "0x264 - Frames Transmitted with Tx FIFO Underrun Statistic Register"]
    pub ieee_t_macerr: IEEE_T_MACERR,
    #[doc = "0x268 - Frames Transmitted with Carrier Sense Error Statistic Register"]
    pub ieee_t_cserr: IEEE_T_CSERR,
    #[doc = "0x26c - Reserved Statistic Register"]
    pub ieee_t_sqe: IEEE_T_SQE,
    #[doc = "0x270 - Flow Control Pause Frames Transmitted Statistic Register"]
    pub ieee_t_fdxfc: IEEE_T_FDXFC,
    #[doc = "0x274 - Octet Count for Frames Transmitted w/o Error Statistic Register"]
    pub ieee_t_octets_ok: IEEE_T_OCTETS_OK,
    _reserved15: [u8; 12usize],
    #[doc = "0x284 - Rx Packet Count Statistic Register"]
    pub rmon_r_packets: RMON_R_PACKETS,
    #[doc = "0x288 - Rx Broadcast Packets Statistic Register"]
    pub rmon_r_bc_pkt: RMON_R_BC_PKT,
    #[doc = "0x28c - Rx Multicast Packets Statistic Register"]
    pub rmon_r_mc_pkt: RMON_R_MC_PKT,
    #[doc = "0x290 - Rx Packets with CRC/Align Error Statistic Register"]
    pub rmon_r_crc_align: RMON_R_CRC_ALIGN,
    #[doc = "0x294 - Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
    pub rmon_r_undersize: RMON_R_UNDERSIZE,
    #[doc = "0x298 - Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
    pub rmon_r_oversize: RMON_R_OVERSIZE,
    #[doc = "0x29c - Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub rmon_r_frag: RMON_R_FRAG,
    #[doc = "0x2a0 - Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
    pub rmon_r_jab: RMON_R_JAB,
    #[doc = "0x2a4 - Reserved Statistic Register"]
    pub rmon_r_resvd_0: RMON_R_RESVD_0,
    #[doc = "0x2a8 - Rx 64-Byte Packets Statistic Register"]
    pub rmon_r_p64: RMON_R_P64,
    #[doc = "0x2ac - Rx 65- to 127-Byte Packets Statistic Register"]
    pub rmon_r_p65to127: RMON_R_P65TO127,
    #[doc = "0x2b0 - Rx 128- to 255-Byte Packets Statistic Register"]
    pub rmon_r_p128to255: RMON_R_P128TO255,
    #[doc = "0x2b4 - Rx 256- to 511-Byte Packets Statistic Register"]
    pub rmon_r_p256to511: RMON_R_P256TO511,
    #[doc = "0x2b8 - Rx 512- to 1023-Byte Packets Statistic Register"]
    pub rmon_r_p512to1023: RMON_R_P512TO1023,
    #[doc = "0x2bc - Rx 1024- to 2047-Byte Packets Statistic Register"]
    pub rmon_r_p1024to2047: RMON_R_P1024TO2047,
    #[doc = "0x2c0 - Rx Packets Greater than 2048 Bytes Statistic Register"]
    pub rmon_r_p_gte2048: RMON_R_P_GTE2048,
    #[doc = "0x2c4 - Rx Octets Statistic Register"]
    pub rmon_r_octets: RMON_R_OCTETS,
    #[doc = "0x2c8 - Frames not Counted Correctly Statistic Register"]
    pub ieee_r_drop: IEEE_R_DROP,
    #[doc = "0x2cc - Frames Received OK Statistic Register"]
    pub ieee_r_frame_ok: IEEE_R_FRAME_OK,
    #[doc = "0x2d0 - Frames Received with CRC Error Statistic Register"]
    pub ieee_r_crc: IEEE_R_CRC,
    #[doc = "0x2d4 - Frames Received with Alignment Error Statistic Register"]
    pub ieee_r_align: IEEE_R_ALIGN,
    #[doc = "0x2d8 - Receive FIFO Overflow Count Statistic Register"]
    pub ieee_r_macerr: IEEE_R_MACERR,
    #[doc = "0x2dc - Flow Control Pause Frames Received Statistic Register"]
    pub ieee_r_fdxfc: IEEE_R_FDXFC,
    #[doc = "0x2e0 - Octet Count for Frames Received without Error Statistic Register"]
    pub ieee_r_octets_ok: IEEE_R_OCTETS_OK,
    _reserved16: [u8; 284usize],
    #[doc = "0x400 - Adjustable Timer Control Register"]
    pub atcr: ATCR,
    #[doc = "0x404 - Timer Value Register"]
    pub atvr: ATVR,
    #[doc = "0x408 - Timer Offset Register"]
    pub atoff: ATOFF,
    #[doc = "0x40c - Timer Period Register"]
    pub atper: ATPER,
    #[doc = "0x410 - Timer Correction Register"]
    pub atcor: ATCOR,
    #[doc = "0x414 - Time-Stamping Clock Period Register"]
    pub atinc: ATINC,
    #[doc = "0x418 - Timestamp of Last Transmitted Frame"]
    pub atstmp: ATSTMP,
    _reserved17: [u8; 488usize],
    #[doc = "0x604 - Timer Global Status Register"]
    pub tgsr: TGSR,
    #[doc = "0x608 - Timer Control Status Register"]
    pub tcsr0: TCSR,
    #[doc = "0x60c - Timer Compare Capture Register"]
    pub tccr0: TCCR,
    #[doc = "0x610 - Timer Control Status Register"]
    pub tcsr1: TCSR,
    #[doc = "0x614 - Timer Compare Capture Register"]
    pub tccr1: TCCR,
    #[doc = "0x618 - Timer Control Status Register"]
    pub tcsr2: TCSR,
    #[doc = "0x61c - Timer Compare Capture Register"]
    pub tccr2: TCCR,
    #[doc = "0x620 - Timer Control Status Register"]
    pub tcsr3: TCSR,
    #[doc = "0x624 - Timer Compare Capture Register"]
    pub tccr3: TCCR,
}
#[doc = "Interrupt Event Register"]
pub struct EIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Event Register"]
pub mod eir;
#[doc = "Interrupt Mask Register"]
pub struct EIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod eimr;
#[doc = "Receive Descriptor Active Register"]
pub struct RDAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Descriptor Active Register"]
pub mod rdar;
#[doc = "Transmit Descriptor Active Register"]
pub struct TDAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Descriptor Active Register"]
pub mod tdar;
#[doc = "Ethernet Control Register"]
pub struct ECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet Control Register"]
pub mod ecr;
#[doc = "MII Management Frame Register"]
pub struct MMFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Management Frame Register"]
pub mod mmfr;
#[doc = "MII Speed Control Register"]
pub struct MSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Speed Control Register"]
pub mod mscr;
#[doc = "MIB Control Register"]
pub struct MIBC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MIB Control Register"]
pub mod mibc;
#[doc = "Receive Control Register"]
pub struct RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Control Register"]
pub mod rcr;
#[doc = "Transmit Control Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Control Register"]
pub mod tcr;
#[doc = "Physical Address Lower Register"]
pub struct PALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Physical Address Lower Register"]
pub mod palr;
#[doc = "Physical Address Upper Register"]
pub struct PAUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Physical Address Upper Register"]
pub mod paur;
#[doc = "Opcode/Pause Duration Register"]
pub struct OPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Opcode/Pause Duration Register"]
pub mod opd;
#[doc = "Transmit Interrupt Coalescing Register"]
pub struct TXIC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Interrupt Coalescing Register"]
pub mod txic;
#[doc = "Receive Interrupt Coalescing Register"]
pub struct RXIC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Interrupt Coalescing Register"]
pub mod rxic;
#[doc = "Descriptor Individual Upper Address Register"]
pub struct IAUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor Individual Upper Address Register"]
pub mod iaur;
#[doc = "Descriptor Individual Lower Address Register"]
pub struct IALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor Individual Lower Address Register"]
pub mod ialr;
#[doc = "Descriptor Group Upper Address Register"]
pub struct GAUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor Group Upper Address Register"]
pub mod gaur;
#[doc = "Descriptor Group Lower Address Register"]
pub struct GALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor Group Lower Address Register"]
pub mod galr;
#[doc = "Transmit FIFO Watermark Register"]
pub struct TFWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Watermark Register"]
pub mod tfwr;
#[doc = "Receive Descriptor Ring Start Register"]
pub struct RDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Descriptor Ring Start Register"]
pub mod rdsr;
#[doc = "Transmit Buffer Descriptor Ring Start Register"]
pub struct TDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Buffer Descriptor Ring Start Register"]
pub mod tdsr;
#[doc = "Maximum Receive Buffer Size Register"]
pub struct MRBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum Receive Buffer Size Register"]
pub mod mrbr;
#[doc = "Receive FIFO Section Full Threshold"]
pub struct RSFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Section Full Threshold"]
pub mod rsfl;
#[doc = "Receive FIFO Section Empty Threshold"]
pub struct RSEM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Section Empty Threshold"]
pub mod rsem;
#[doc = "Receive FIFO Almost Empty Threshold"]
pub struct RAEM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Almost Empty Threshold"]
pub mod raem;
#[doc = "Receive FIFO Almost Full Threshold"]
pub struct RAFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Almost Full Threshold"]
pub mod rafl;
#[doc = "Transmit FIFO Section Empty Threshold"]
pub struct TSEM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Section Empty Threshold"]
pub mod tsem;
#[doc = "Transmit FIFO Almost Empty Threshold"]
pub struct TAEM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Almost Empty Threshold"]
pub mod taem;
#[doc = "Transmit FIFO Almost Full Threshold"]
pub struct TAFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Almost Full Threshold"]
pub mod tafl;
#[doc = "Transmit Inter-Packet Gap"]
pub struct TIPG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Inter-Packet Gap"]
pub mod tipg;
#[doc = "Frame Truncation Length"]
pub struct FTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame Truncation Length"]
pub mod ftrl;
#[doc = "Transmit Accelerator Function Configuration"]
pub struct TACC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Accelerator Function Configuration"]
pub mod tacc;
#[doc = "Receive Accelerator Function Configuration"]
pub struct RACC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Accelerator Function Configuration"]
pub mod racc;
#[doc = "Reserved Statistic Register"]
pub struct RMON_T_DROP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reserved Statistic Register"]
pub mod rmon_t_drop;
#[doc = "Tx Packet Count Statistic Register"]
pub struct RMON_T_PACKETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Packet Count Statistic Register"]
pub mod rmon_t_packets;
#[doc = "Tx Broadcast Packets Statistic Register"]
pub struct RMON_T_BC_PKT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Broadcast Packets Statistic Register"]
pub mod rmon_t_bc_pkt;
#[doc = "Tx Multicast Packets Statistic Register"]
pub struct RMON_T_MC_PKT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Multicast Packets Statistic Register"]
pub mod rmon_t_mc_pkt;
#[doc = "Tx Packets with CRC/Align Error Statistic Register"]
pub struct RMON_T_CRC_ALIGN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Packets with CRC/Align Error Statistic Register"]
pub mod rmon_t_crc_align;
#[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
pub struct RMON_T_UNDERSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
pub mod rmon_t_undersize;
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
pub struct RMON_T_OVERSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
pub mod rmon_t_oversize;
#[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub struct RMON_T_FRAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod rmon_t_frag;
#[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
pub struct RMON_T_JAB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
pub mod rmon_t_jab;
#[doc = "Tx Collision Count Statistic Register"]
pub struct RMON_T_COL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Collision Count Statistic Register"]
pub mod rmon_t_col;
#[doc = "Tx 64-Byte Packets Statistic Register"]
pub struct RMON_T_P64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx 64-Byte Packets Statistic Register"]
pub mod rmon_t_p64;
#[doc = "Tx 65- to 127-byte Packets Statistic Register"]
pub struct RMON_T_P65TO127 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx 65- to 127-byte Packets Statistic Register"]
pub mod rmon_t_p65to127;
#[doc = "Tx 128- to 255-byte Packets Statistic Register"]
pub struct RMON_T_P128TO255 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx 128- to 255-byte Packets Statistic Register"]
pub mod rmon_t_p128to255;
#[doc = "Tx 256- to 511-byte Packets Statistic Register"]
pub struct RMON_T_P256TO511 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx 256- to 511-byte Packets Statistic Register"]
pub mod rmon_t_p256to511;
#[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
pub struct RMON_T_P512TO1023 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
pub mod rmon_t_p512to1023;
#[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
pub struct RMON_T_P1024TO2047 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
pub mod rmon_t_p1024to2047;
#[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
pub struct RMON_T_P_GTE2048 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
pub mod rmon_t_p_gte2048;
#[doc = "Tx Octets Statistic Register"]
pub struct RMON_T_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Octets Statistic Register"]
pub mod rmon_t_octets;
#[doc = "Reserved Statistic Register"]
pub struct IEEE_T_DROP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reserved Statistic Register"]
pub mod ieee_t_drop;
#[doc = "Frames Transmitted OK Statistic Register"]
pub struct IEEE_T_FRAME_OK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted OK Statistic Register"]
pub mod ieee_t_frame_ok;
#[doc = "Frames Transmitted with Single Collision Statistic Register"]
pub struct IEEE_T_1COL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted with Single Collision Statistic Register"]
pub mod ieee_t_1col;
#[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
pub struct IEEE_T_MCOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
pub mod ieee_t_mcol;
#[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
pub struct IEEE_T_DEF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
pub mod ieee_t_def;
#[doc = "Frames Transmitted with Late Collision Statistic Register"]
pub struct IEEE_T_LCOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted with Late Collision Statistic Register"]
pub mod ieee_t_lcol;
#[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
pub struct IEEE_T_EXCOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
pub mod ieee_t_excol;
#[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
pub struct IEEE_T_MACERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
pub mod ieee_t_macerr;
#[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
pub struct IEEE_T_CSERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
pub mod ieee_t_cserr;
#[doc = "Reserved Statistic Register"]
pub struct IEEE_T_SQE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reserved Statistic Register"]
pub mod ieee_t_sqe;
#[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
pub struct IEEE_T_FDXFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
pub mod ieee_t_fdxfc;
#[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
pub struct IEEE_T_OCTETS_OK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
pub mod ieee_t_octets_ok;
#[doc = "Rx Packet Count Statistic Register"]
pub struct RMON_R_PACKETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Packet Count Statistic Register"]
pub mod rmon_r_packets;
#[doc = "Rx Broadcast Packets Statistic Register"]
pub struct RMON_R_BC_PKT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Broadcast Packets Statistic Register"]
pub mod rmon_r_bc_pkt;
#[doc = "Rx Multicast Packets Statistic Register"]
pub struct RMON_R_MC_PKT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Multicast Packets Statistic Register"]
pub mod rmon_r_mc_pkt;
#[doc = "Rx Packets with CRC/Align Error Statistic Register"]
pub struct RMON_R_CRC_ALIGN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Packets with CRC/Align Error Statistic Register"]
pub mod rmon_r_crc_align;
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
pub struct RMON_R_UNDERSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
pub mod rmon_r_undersize;
#[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
pub struct RMON_R_OVERSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
pub mod rmon_r_oversize;
#[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub struct RMON_R_FRAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod rmon_r_frag;
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
pub struct RMON_R_JAB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
pub mod rmon_r_jab;
#[doc = "Reserved Statistic Register"]
pub struct RMON_R_RESVD_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reserved Statistic Register"]
pub mod rmon_r_resvd_0;
#[doc = "Rx 64-Byte Packets Statistic Register"]
pub struct RMON_R_P64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 64-Byte Packets Statistic Register"]
pub mod rmon_r_p64;
#[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
pub struct RMON_R_P65TO127 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
pub mod rmon_r_p65to127;
#[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
pub struct RMON_R_P128TO255 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
pub mod rmon_r_p128to255;
#[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
pub struct RMON_R_P256TO511 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
pub mod rmon_r_p256to511;
#[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
pub struct RMON_R_P512TO1023 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
pub mod rmon_r_p512to1023;
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
pub struct RMON_R_P1024TO2047 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
pub mod rmon_r_p1024to2047;
#[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
pub struct RMON_R_P_GTE2048 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
pub mod rmon_r_p_gte2048;
#[doc = "Rx Octets Statistic Register"]
pub struct RMON_R_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Octets Statistic Register"]
pub mod rmon_r_octets;
#[doc = "Frames not Counted Correctly Statistic Register"]
pub struct IEEE_R_DROP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames not Counted Correctly Statistic Register"]
pub mod ieee_r_drop;
#[doc = "Frames Received OK Statistic Register"]
pub struct IEEE_R_FRAME_OK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Received OK Statistic Register"]
pub mod ieee_r_frame_ok;
#[doc = "Frames Received with CRC Error Statistic Register"]
pub struct IEEE_R_CRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Received with CRC Error Statistic Register"]
pub mod ieee_r_crc;
#[doc = "Frames Received with Alignment Error Statistic Register"]
pub struct IEEE_R_ALIGN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Received with Alignment Error Statistic Register"]
pub mod ieee_r_align;
#[doc = "Receive FIFO Overflow Count Statistic Register"]
pub struct IEEE_R_MACERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Overflow Count Statistic Register"]
pub mod ieee_r_macerr;
#[doc = "Flow Control Pause Frames Received Statistic Register"]
pub struct IEEE_R_FDXFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flow Control Pause Frames Received Statistic Register"]
pub mod ieee_r_fdxfc;
#[doc = "Octet Count for Frames Received without Error Statistic Register"]
pub struct IEEE_R_OCTETS_OK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Octet Count for Frames Received without Error Statistic Register"]
pub mod ieee_r_octets_ok;
#[doc = "Adjustable Timer Control Register"]
pub struct ATCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Adjustable Timer Control Register"]
pub mod atcr;
#[doc = "Timer Value Register"]
pub struct ATVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Value Register"]
pub mod atvr;
#[doc = "Timer Offset Register"]
pub struct ATOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Offset Register"]
pub mod atoff;
#[doc = "Timer Period Register"]
pub struct ATPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Period Register"]
pub mod atper;
#[doc = "Timer Correction Register"]
pub struct ATCOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Correction Register"]
pub mod atcor;
#[doc = "Time-Stamping Clock Period Register"]
pub struct ATINC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time-Stamping Clock Period Register"]
pub mod atinc;
#[doc = "Timestamp of Last Transmitted Frame"]
pub struct ATSTMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timestamp of Last Transmitted Frame"]
pub mod atstmp;
#[doc = "Timer Global Status Register"]
pub struct TGSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Global Status Register"]
pub mod tgsr;
#[doc = "Timer Control Status Register"]
pub struct TCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Status Register"]
pub mod tcsr;
#[doc = "Timer Compare Capture Register"]
pub struct TCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Compare Capture Register"]
pub mod tccr;
