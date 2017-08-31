// Everything in this module comes from the datasheet.
// Refer to that for the best documentation.
#![allow(missing_docs)]

//! Addresses and bitflag definitions for registers.

/// A macro to declare a bunch of u8 constants
macro_rules! register_addresses {
    ( $($address:expr => $name:ident;)* ) => {
        $( pub const $name: u8 = $address; )*
    }
}

// Declare all register addresses
// Refer to Table 17 of the L3GD20H datasheet.

register_addresses! {
    // 0x00 - 0x0E => reserved
    0x0F => WHO_AM_I;
    // 0x10 - 0x1F => reserved
    0x20 => CTRL_1;
    0x21 => CTRL_2;
    0x22 => CTRL_3;
    0x23 => CTRL_4;
    0x24 => CTRL_5;
    0x25 => REFERENCE;
    0x26 => OUT_TEMP;
    0x27 => STATUS;
    0x28 => OUT_X_L;
    0x29 => OUT_X_H;
    0x2A => OUT_Y_L;
    0x2B => OUT_Y_H;
    0x2C => OUT_Z_L;
    0x2D => OUT_Z_H;
    0x2E => FIFO_CTRL;
    0x2F => FIFO_SRC;
    0x30 => IG_CFG;
    0x31 => IG_SRC;
    0x32 => IG_THS_XH;
    0x33 => IG_THS_XL;
    0x34 => IG_THS_YH;
    0x35 => IG_THS_YL;
    0x36 => IG_THS_ZH;
    0x37 => IG_THS_ZL;
    0x38 => IG_DURATION;
    0x39 => LOW_ODR;
}


/// Declare multiple bitflags using an abbreviated syntax.
///
/// All of the registers are 8 bits, with each flag being a single bit.
macro_rules! define_registers {
    (
        $(
            $name:ident {
                $( $shift:expr,$b:ident | )*
            }
        )*
    ) => {
        $(
            bitflags!{
                pub struct $name: u8 {
                    $(
                        #[allow(non_upper_case_globals)]
                        const $b = 1 << $shift;
                    )*
                }
            }
        )*
    }
}

// Define bitflags for all configuration registers.
//
// Output registers are not defined here,
// since no bit-level operations are necessary on them.

define_registers! {
    Ctrl1 {
        7, DR1         | 6, DR0         | 5, BW1         | 4, BW0         |
        3, PD          | 2, Zen         | 1, Yen         | 0, Xen         |
    }
    Ctrl2 {
        7, EXTRen      | 6, LVLen       | 5, HPM1        | 4, HPM0        |
        3, HPCF3       | 2, HPCF2       | 1, HPCF1       | 0, HPCF0       |
    }
    Ctrl3 {
        7, INT1_IG     | 6, INT1_Boot   | 5, H_Lactive   | 4, PP_OD       |
        3, INT2_DRDY   | 2, INT2_FTH    | 1, INT2_ORun   | 0, INT2_Empty  |
    }
    Ctrl4 {
        7, BDU         | 6, BLE         | 5, FS1         | 4, FS0         |
        3, IMPen       | 2, ST2         | 1, ST1         | 0, SIM         |
    }
    Ctrl5 {
        7, BOOT        | 6, FIFO_EN     | 5, StopOnFTH   | 4, HPen        |
        3, IG_Sel1     | 2, IG_Sel0     | 1, Out_Sel1    | 0, Out_Sel0    |
    }
    Status {
        7, ZYXOR       | 6, ZOR         | 5, YOR         | 4, XOR         |
        3, ZYXDA       | 2, ZDA         | 1, YDA         | 0, XDA         |
    }
    FifoCtrl {
        7, FM2         | 6, FM1         | 5, FM0         | 4, FTH4        |
        3, FTH3        | 2, FTH2        | 1, FTH1        | 0, FTH0        |
    }
    FifoSrc {
        7, FTH         | 6, OVRN        | 5, EMPTY       | 4, FSS4        |
        3, FSS3        | 2, FSS2        | 1, FSS1        | 0, FSS0        |
    }
    IgCfg {
        7, AND_OR      | 6, LIR         | 5, ZHIE        | 4, ZLIE        |
        3, YHIE        | 2, YLIE        | 1, XHIE        | 0, XLIE        |
    }
    IgSrc {
        /* ---------- */ 6, IA          | 5, ZH          | 4, ZL          |
        3, YH          | 2, YL          | 1, XH          | 0, XL          |
    }
    IgThsXH {
        7, DCRM        | 6, THSX14      | 5, THSX13      | 4, THSX12      |
        3, THSX11      | 2, THSX10      | 1, THSX9       | 0, THSX8       |
    }
    IgThsXL {
        7, THSX7       | 6, THSX6       | 5, THSX5       | 4, THSX4       |
        3, THSX3       | 2, THSX2       | 1, THSX1       | 0, THSX0       |
    }
    IgThsYH {
        /*----------- */ 6, THSY14      | 5, THSY13      | 4, THSY12      |
        3, THSY11      | 2, THSY10      | 1, THSY9       | 0, THSY8       |
    }
    IgThsYL {
        7, THSY7       | 6, THSY6       | 5, THSY5       | 4, THSY4       |
        3, THSY3       | 2, THSY2       | 1, THSY1       | 0, THSY0       |
    }
    IgThsZH {
        /*----------- */ 6, THSZ14      | 5, THSZ13      | 4, THSZ12      |
        3, THSZ11      | 2, THSZ10      | 1, THSZ9       | 0, THSZ8       |
    }
    IgThsZL {
        7, THSZ7       | 6, THSZ6       | 5, THSZ5       | 4, THSZ4       |
        3, THSZ3       | 2, THSZ2       | 1, THSZ1       | 0, THSZ0       |
    }
    IgDuration {
        7, WAIT        | 6, D6          | 5, D5          | 4, D4          |
        3, D3          | 2, D2          | 1, D1          | 0, D0          |
    }
    LowOdr {
        /* ----------- | ------------- */ 5, DRDY_HL     | /* ---------- */
        3, I2C_dis     | 2, SW_RES      | /* ---------- */ 0, Low_ODR     |
    }
    // ...
}
