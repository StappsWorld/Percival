use std::{borrow::Borrow, ptr::null};

use crate::needed_classes::*;

pub fn PrsAsmImm(cc: &mut CCmpCtrl, arg: &mut CAsmArg) -> bool {
    if arg.imm_or_off_present {
        LexExcept(cc, "Already one immediate at ".to_owned());
    }
    arg.imm_or_off_present = true;
    arg.num.local_asm_undef_hash = None;
    arg.num.glbl_asm_undef_hash = None;
    cc.asm_undef_hash = None;
    cc.asb_cnts = 0;
    cc.flags &= !(CCF_UNRESOLVED + CCF_LOCAL);
    if !IsLexExpression2Bin(cc, &arg.num.machine_code) {
        LexSkipEol(cc);
    } else {
        if cc.abs_cnts.externs {
            LexExcept(cc, "Extern Not Allowed at ");
        }
        if cc.flags & CCF_UNRESOLVED {
            if cc.flags & CCF_LOCAL {
                arg.num.local_asm_undef_hash = cc.asm_undef_hash;
                cc.asm_undef_hash = None;
            } else {
                arg.num.glbl_asm_undef_hash = cc.asm_undef_hash;
                cc.asm_undef_hash = None;
            }
        } else {
            arg.num.i = Call(arg.num.machine_code);
            arg.num.glbl_asm_undef_hash = cc.asm_undef_hash;
            cc.asm_undef_hash = None;
            arg.num.machine_code = None;
        }
    }
    true
}

pub fn PrsAsmArg(cc: &mut CCmpCtrl, arg: &mut CAsmArg, rel: bool) {
    let mut tmph: CHashGeneric;
    let mut tmph1: CHashGeneric;
    let mut tmpr: CHashReg;
    arg.seg = REG_NONE;
    arg.reg1 = REG_NONE;
    arg.reg2 = REG_NONE;
    arg.scale = 1;
    while true {
        if cc.token == TK_IDENT {
            let mut goto_none = false;
            match cc.hash_entry {
                Some(h_e) => {
                    tmph = h_e;
                    if tmph.parent._type & HTG_TYPE_MASK == HTT_REG {
                        tmpr.parent = tmph.parent;
                        arg.reg1_type = tmpr.reg_type;
                        match tmpr.reg_type {
                            REGT_R8 => {
                                arg.size = 1;
                                arg.reg1 = tmpr.reg_num as i64;
                                Lex(cc);
                                return;
                            }
                            REGT_R16 => {
                                arg.size = 2;
                                arg.reg1 = tmpr.reg_num as i64;
                                Lex(cc);
                                return;
                            }
                            REGT_R32 => {
                                arg.size = 4;
                                arg.reg1 = tmpr.reg_num as i64;
                                Lex(cc);
                                return;
                            }
                            REGT_R64 => {
                                arg.size = 8;
                                arg.reg1 = tmpr.reg_num as i64;
                                Lex(cc);
                                return;
                            }
                            REGT_SEG => {
                                arg.seg = tmpr.reg_num as i64;
                                if Lex(cc) != ':' {
                                    arg.just_seg = true;
                                    return;
                                } else {
                                    Lex(cc);
                                }
                                return;
                            }
                            REGT_FSTK | REGT_MM | REGT_XMM => {
                                arg.size = 8;
                                arg.reg1 = tmpr.reg_sum as i64;
                                Lex(cc);
                                return;
                            }
                        }
                    } else {
                        if tmph.parent._type & HTG_TYPE_MASK == HTT_CLASS
                            || tmph.parent._type & HTG_TYPE_MASK == HTT_INTERNAL_TYPE
                        {
                            match HashFind(cc.cur_str, cmp.asm_hash, HTT_ASM_KEYWORD) {
                                Some(t) => tmph = t,
                                None => (),
                            }
                        }
                        if tmph.parent._type & HTG_TYPE_MASK == HTT_ASM_KEYWORD {
                            match tmph.user_data0 {
                                AKW_I8 | AKW_U8 => arg.size = 1,
                                AKW_I16 | AKW_U16 => arg.size = 2,
                                AKW_I32 | AKW_U32 => arg.size = 4,
                                AKW_I64 | AKW_U64 => arg.size = 8,
                                _ => LexExcept(cc, "syntax error at ".to_owned()),
                            }
                            Lex(cc);
                        } else {
                            goto_none = true;
                        }
                    }
                }
                None => goto_none = true,
            }
            if goto_none {
                PrsAsmImm(cc, arg);
                arg.num.abs_cnts = cc.abs_cnts;
                if arg.size <= 1 && !rel && arg.num.abs_cnts & 1 {
                    if cc.aotc.seg_size == 16 {
                        arg.size = 2;
                    } else {
                        arg.size = 4;
                    }
                }
                if cc.token != '[' {
                    return;
                }
            }
        } else if cc.token == '['.into() {
            arg.indirect = true;
            Lex(cc); // skip '['
            while cc.token != 0 && cc.token != ']'.into() {
                let pa_asm_indirect_imm = true;
                if cc.token == TK_IDENT {
                    pa_asm_indirect_imm = false;
                    match cc.hash_entry {
                        Some(t) => {
                            tmph = t;
                            if tmph.parent._type & HTG_TYPE_MASK == HTT_REG
                            // TODO : && REGT_R16<=tmph(CHashReg *)->reg_type<=REGT_R64
                            {
                                tmpr = tmph;
                                arg.reg2_type = tmpr.reg_type;
                                if arg.reg1 == REG_NONE {
                                    if tmpr.reg_num & 7 == REG_RSP {
                                        arg.reg1 = 4;
                                        arg.reg2 = tmpr.reg_num as i64;
                                    } else {
                                        arg.reg1 = tmpr.reg_num as i64;
                                    }
                                } else {
                                    arg.reg2 = tmpr.reg_num as i64;
                                    Lex(cc);
                                }
                            } else {
                                pa_asm_indirect_imm = true;
                            }
                        }
                        None => pa_asm_indirect_imm = true,
                    }
                } else if cc.token == '*'.into() {
                    pa_asm_indirect_imm = false;
                    Lex(cc);
                    if cc.token != TK_I64 {
                        LexExcept(cc, "Expecting scale factor at ".to_owned());
                        arg.scale = cc.cur_i64;
                        Lex(cc); // skip scale
                        if arg.reg2 != REG_NONE {
                            SwapI64(&mut arg.reg1, &mut arg.reg2);
                            SwapI64(&mut arg.reg1_type, &mut arg.reg2_type);
                        }
                    }
                } else if cc.token == '+' {
                    Lex(cc); //skip '+'
                }
                if pa_asm_indirect_imm {
                    PrsAsmImm(cc, arg);
                    arg.num.abs_cnts = cc.abs_cnts;
                }
            }
            if cc.token != ']'.into() {
                LexExcept(cc, "Missing ']' at ".to_owned());
                Lex(cc); //skip ]
                return;
            } else {
                while true {
                    PrsAsmImm(cc, arg);
                    arg.num.abs_cnts = cc.abs_cnts;
                    if cc.token != ']'.into() {
                        LexExcept(cc, "Missing ']' at ".to_owned());
                        Lex(cc); //skip ]
                        return;
                    }
                }
            }
        }
    }
}

pub fn AsmMakeArgMask(cc: &mut CCmpCtrl, arg: &mut CAsmArg) -> i64 {
    let mut aotc: &mut CAOTCtrl = cc.aotc;
    let res: i64;
    if arg.just_seg {
        match arg.seg {
            0 => res = 1 << ARGT_ES | 1 << ARGT_SREG,
            1 => res = 1 << ARGT_CS | 1 << ARGT_SREG,
            2 => res = 1 << ARGT_SS | 1 << ARGT_SREG,
            3 => res = 1 << ARGT_DS | 1 << ARGT_SREG,
            4 => res = 1 << ARGT_FS | 1 << ARGT_SREG,
            5 => res = 1 << ARGT_GS | 1 << ARGT_SREG,
        }
        return res;
    }
    if arg.reg1_type == REGT_FSTK {
        if arg.reg1 {
            res = 1 << ARGT_STI;
        } else {
            res = 1 << ARGT_ST0 | 1 << ARGT_STI;
        }
        return res;
    }

    res = cmp.size_arg_mask[arg.size];

    if aotc.seg_size == 64 {
        res &= 0xFF0FFFFFFF;
    }

    if arg.reg1 != REG_NONE
        && arg.imm_or_off_present
        && !arg.num.i > 0
        && !arg.num.glbl_asm_undef_hash.is_some()
        && !arg.num.local_asm_undef_hash.is_some()
    {
        arg.imm_or_off_present = false;
    }

    if arg.reg2 != REG_NONE || arg.scale != 1 {
        res &= 0x0000FF0000;
        return res;
    }

    if arg.indirect {
        if arg.imm_or_off_present {
            res &= 0x00FFFF0000;
        } else {
            res &= 0x000FFF0000;
        }
    } else {
        if arg.imm_or_off_present {
            res &= 0x000F000FFE;
        } else {
            res &= 0x3F0FFFF000;
        }
    }
    if arg.seg != REG_NONE {
        res &= 0x00FFFF0000;
    }
    if arg.reg1 != REG_NONE {
        if arg.indirect {
            res &= 0x00FFFF0000;
        } else if arg.num.i < 0 {
            if arg.num.i >= I8_MIN {
                res &= 0x8FE;
            } else if arg.num.i >= I16_MIN {
                res &= 0x8EE;
            } else if arg.num.i >= I32_MIN {
                res &= 0x8CE;
            } else {
                res &= 0x88E
            }
        } else {
            if arg.num.i <= I8_MAX {
                res &= 0xFFE;
            } else if arg.num.i <= U8_MAX {
                res &= 0xFEE;
            } else if arg.num.i <= I16_MAX {
                res &= 0xEEE;
            } else if arg.num.i <= I32_MAX {
                res &= 0xECE;
            } else if arg.num.i <= U32_MAX {
                res &= 0xC8E;
            } else {
                res &= 0x88E;
            }
        }
    } else {
        res &= 0x3F00FFF000;
        if !arg.indirect {
            res &= 0xFFFF0FFFFF;
        }
    }
    match arg.reg1 {
        REG_RAX => res &= !0x3000000000,
        REG_RCX => res &= !0x2F00000000,
        REG_RDX => res &= !0x1F00000000,
        _ => res &= !0x3F00000000,
    }
    res
}

pub fn AsmStoreNum(cc: &mut CCmpCtrl, num2: &mut CAsmNum2, cnt: i64, U8_avail: bool) -> bool {
    let mut aotc: &mut CAOTCtrl = cc.aotc;
    let mut tmpa: &mut CAOTAbsAddr;

    if !num2.imm_flag {
        num2.num.i -= num2.rel;
    }
    for i in 0..cnt {
        if num2.U8_cnt == 1 {
            if num2.num.local_asm_undef_hash.is_some() || num2.num.glbl_asm_undef_hash.is_some() {
                AsmUnresolvedAdd(
                    cc,
                    num2.num.machine_code,
                    IET_REL_I8 + num2.imm_flag,
                    aotc.rip,
                    num2.rel,
                    num2.num.local_asm_undef_hash.unwrap(),
                    num2.num.glbl_asm_undef_hash.unwrap(),
                    cc.lex_include_stk.line_num,
                    U8_avail,
                );
            } else if !num2.imm_flag && !(I8 <= num2.num.i && num2.num.i <= I8_MAX) {
                LexExcept(cc, "Branch out of range at ".to_owned());
            }
            if num2.imm_flag {
                if num2.num.abs_cnts.abs_addres & 1 {
                    tmpa = CAOTAbsAddr {
                        next: aotc.abss,
                        rip: aotc.rip,
                        _type: AAT_ADD_U8,
                        pad: vec![],
                    };
                }
            } else {
                if num2.num.abs_cnts.c_addres & 1 {
                    tmpa = CAOTAbsAddr {
                        next: aotc.abss,
                        rip: aotc.rip,
                        _type: AAT_SUB_U8,
                        pad: vec![],
                    };
                }
            }
            AOTStoreCodeU8(cc, num2.num.i);
        } else {
            if num2.U8_cnt == 2 {
                if num2.num.local_asm_undef_hash.is_some() || num2.num.glbl_asm_undef_hash.is_some()
                {
                    AsmUnresolvedAdd(
                        cc,
                        num2.num.machine_code,
                        IET_REL_I16 + num2.imm_flag,
                        aotc.rip,
                        num2.rel,
                        num2.num.local_asm_undef_hash.unwrap(),
                        num2.num.glbl_asm_undef_hash.unwrap(),
                        cc.lex_include_stk.line_num,
                        U8_avail,
                    );
                } else if !num2.imm_flag && !(I16 <= num2.num.i && num2.num.i <= I16_MAX) {
                    LexExcept(cc, "Branch out of range at ".to_owned());
                }
                if num2.imm_flag {
                    if num2.num.abs_cnts.abs_addres & 1 {
                        tmpa = CAOTAbsAddr {
                            next: aotc.abss,
                            rip: aotc.rip,
                            _type: AAT_ADD_U16,
                            pad: vec![],
                        };
                    }
                } else {
                    if num2.num.abs_cnts.c_addres & 1 {
                        tmpa = CAOTAbsAddr {
                            next: aotc.abss,
                            rip: aotc.rip,
                            _type: AAT_SUB_U16,
                            pad: vec![],
                        };
                    }
                }
                let hex_num = hex_to_bytes(num2.num.i.to_string().as_str()).unwrap_or(vec![0, 0]);
                AOTStoreCodeU8(cc, hex_num[0]);
                AOTStoreCodeU8(cc, hex_num[1]);
            } else if num2.U8_cnt == 4 {
                if num2.num.local_asm_undef_hash.is_some() || num2.num.glbl_asm_undef_hash.is_some()
                {
                    AsmUnresolvedAdd(
                        cc,
                        num2.num.machine_code,
                        IET_REL_I32 + if num2.imm_flag { 1 } else { 0 },
                        aotc.rip,
                        num2.rel,
                        num2.num.local_asm_undef_hash.unwrap_or_default(),
                        num2.num.glbl_asm_undef_hash.unwrap_or_default(),
                        cc.lex_include_stk.line_num,
                        U8_avail,
                    );
                }
            }
        }
    }
    return false;
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
    } else {
        None
    }
}
