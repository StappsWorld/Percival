mod needed_classes;
mod Asm;

fn main() {
    CmpLoadDefines();
    CmpLoadDefines();
    CmpFillTables();
    QueInit(&cmp.ic_nop);
    cmp.ic_nop.ic_class=cmp.internal_types[RT_I64];
    cmp.ic_nop.ic_code=IC_NOP1;
    AsmHashLoad();
    UAsmHashLoad();
}
