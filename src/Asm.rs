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
                                arg.reg1 = tmpr.reg_num;
                                Lex(cc);
                                return;
                            }
                            REGT_R16 => {
                                arg.size = 2;
                                arg.reg1 = tmpr.reg_num;
                                Lex(cc);
                                return;
                            }
                            REGT_R32 => {
                                arg.size = 4;
                                arg.reg1 = tmpr.reg_num;
                                Lex(cc);
                                return;
                            }
                            REGT_R64 => {
                                arg.size = 8;
                                arg.reg1 = tmpr.reg_num;
                                Lex(cc);
                                return;
                            }
                            REGT_SEG => {
                                arg.seg = tmpr.reg_num;
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
                                arg.reg1 = tmpr.reg_sum;
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
                },
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
                match cc.hash_entry {
                    Some(t) => {
                        tmph = t;
                        if tmph._type & HTG_TYPE_MASK == HTT_REG
                         && REGT_R16 <= tmph
                    },
                    None => {},
                }
            }
        }
    }
}
