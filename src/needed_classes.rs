use {

};

// THESE CLASSES COME DIRECTLY FROM /Kernel/Kernel*.HH

pub struct CHashExport {
    val : i64,
}

pub struct CAsmUndefHash<'a> {
    next : Option<&'a mut CAsmUndefHash<'a>>,
    hash : CHashExport,
}

pub struct CAbsCntsI64 {
    abs_addres : u16, //Only odd/even matters. Cnt of absolute addres in an exp.
	c_addres : u16, //Only odd/even matters. Cnt of C addres in an exp.
    externs : u32, //Only nonzero matters. Some regions have externs banned.
}

pub struct CAsmNum<'a> {
    i : i64,
    machine_code : u8,
    local_asm_undef_hash : Option<CAsmUndefHash<'a>>,
    glbl_asm_undef_hash : Option<CAsmUndefHash<'a>>,
    abs_cnts : CAbsCntsI64,
}

pub struct CAsmArg<'a> {
    num : CAsmNum<'a>,
    seg : i64,
    size : i64,
    reg1 : i64,
    reg2 : i64,
	reg1_type : i64,
    reg2_type : i64,
	scale : i64,
    indirect : bool,
    imm_or_off_present : bool,
    just_seg : bool,
    pad : Vec<bool>,
}

pub struct CAOTAbsAddr<'a> {
    next : Option<&'a mut CAOTAbsAddr<'a>>,
    rip : i64,
    _type : u8,
    pad : Vec<u8>,
}

pub struct CAOTHeapGlblRef<'a> {
    next : Option<&'a mut CAOTHeapGlblRef<'a>>,
    rip : i64,
}

pub struct CAOTHeapGlbl<'a> {
    next : Option<&'a mut CAOTHeapGlbl<'a>>,
    _str : u8,
    size : i64,
    references : &'a mut CAOTHeapGlblRef<'a>,
}

pub struct I64_with_doc<'a> {
    val : &'a mut i64,
    doc : &'a mut CDoc<'a>,
    doc_e : &'a mut CDocEntry<'a>,
}

pub struct Put_Key<'a> {
    val : &'a mut bool,
    doc : &'a mut CDoc<'a>,
    data : &'a mut u8,
    ch : i64,
    sc : i64,
}

pub struct Put_S<'a> {
    val : &'a mut bool,
    doc : &'a mut CDoc<'a>,
    data : &'a mut u8,
    st : &'a mut u8,
}

pub struct CDocSettings {
    final_u32_attr : u32,
    left_margin : i16,
    right_margin : i16,
    indent : i16,
    page_len : u16,
    header : u16,
    footer : u16,
    shifted_x : i8,
    shifted_y : i8,
    state : u8,
    comment_depth : u8,
    paren_depth : u8,
    brace_depth : u8,
	cur_text_attr : u8,
    dft_text_attr : u8,
}

pub struct CDocEntryBase<'a> {
    next : Option<&'a mut CDocEntryBase<'a>>,
    last : Option<&'a mut CDocEntryBase<'a>>,
    tag : &'a mut u8,
    type_u8 : u8,
    _type : u32,
    page_line_num : i32,
    de_flags : i64,
    x : i32,
    y : i32,
    min_col : u32,
    max_col : u32,
    settings : CDocSettings,
    user_data : i64,
}

pub struct CDate {
    time : u32,
    date : i32,
}

pub struct CBGR48 {
    r : u16,
    g : u16,
    b : u16,
    pad : u16,
}

pub struct CDC<'a> {
    start : bool,
    cdt : CDate,
    x0 : i32,
    y0 : i32,
    width : i32,
    width_internal : i32,
	height : i32,
	flags : i32,
    end : bool,
    palette : CBGR48,
    body : &'a mut u8,
}

pub struct U0_with_CTask<'a> {
    val : &'a mut u16,
    task : &'a mut CTask<'a>,
    dc : &'a mut CDC<'a>,
}

pub struct CMemBlk<'a> {
    next : Option<&'a mut CMemBlk<'a>>,
    last : &'a mut CMemBlk<'a>,
    mb_signature : u32,
    pags : u32,
}

pub struct CBlkPool<'a> {
    locked_flags : i64,
    alloced_u8s : i64,
    used_u8s : i64,
    mem_free_lst : &'a mut CMemBlk<'a>,
	mem_free_2meg_lst : &'a mut CMemBlk<'a>, //This is for Sup1CodeScraps/Mem/Mem2Meg.HC.
    free_pag_hash : &'a mut Vec<CMemBlk<'a>>,
    free_pag_hash2 : &'a mut Vec<CMemBlk<'a>>,
}

pub struct CMemUnused<'a> {
    hc : bool,
    caller1 : bool,
    caller2 : bool,
    next : Option<&'a mut CMemUnused<'a>>,
    size : i64,
}

pub struct CMemUsed<'a> {
    hc : &'a mut CHeapCtrl<'a>,
    caller1 : bool,
    caller2 : bool,
    next : bool,
    last : bool,
    size : i64,
    start : bool,
}

pub struct CHeapCtrl<'a> {
    bp : &'a mut CBlkPool<'a>,
    hc_signature : u32,
    pad : u32,
    locked_flags : i64,
    alloced_u8s : i64,
    used_u8s : i64,
    mem_task : &'a mut CTask<'a>,
    next_mem_blk : &'a mut CMemBlk<'a>,
    last_mem_blk : &'a mut CMemBlk<'a>,
    last_mergable : &'a mut CMemBlk<'a>,
    malloc_free_lst : &'a mut CMemUnused<'a>,
    next_um : &'a mut CMemUsed<'a>,
    last_um : &'a mut CMemUsed<'a>,
    heap_hash : &'a mut Vec<CMemUnused<'a>>,
}

pub struct CDirEntry<'a> {
    next : &'a mut CDirEntry<'a>,
    parent : &'a mut CDirEntry<'a>,
    sub : &'a mut CDirEntry<'a>,
    full : &'a mut u8,
    user_data : i64,
    user_data2 : i64,
    start : bool,
    attr : u16,
    name : Vec<u8>,
    clus : i64,
    size : i64,
    datetime : CDate,
}

pub struct CFile<'a> {
    flags : i64,
   de : CDirEntry<'a>,
  dv : &'a mut CDrv<'a>,
  fblk_num : i64,
  clus : i64,
  file_clus_num : i64,
  max_blk : i64,
  clus_buf : &'a mut u8,
}

pub struct CBlkDev<'a> {
    lock_fwding : &'a mut CBlkDev<'a>, //If two blkdevs on same controller, use just one lock
  locked_flags : i64,
  bd_signature : u32,
	_type : u32,
    flags : u32,
  first_drv_let : u8,
  unit : u8,
  pad : Vec<u8>,
  base0 : u32,
  base1 : u32,
    blk_size : u32,
  drv_offset : i64,
  init_root_dir_blks : i64,
	max_blk : i64,
  dev_id_record : &'a mut u16,
  RAM_dsk : &'a mut u8,
    file_dsk_name : &'a mut u8,
  file_dsk : &'a mut CFile<'a>,
  owning_task : &'a mut CTask<'a>,
  last_time : f64,
  max_reads : u32,
  max_writes : u32,
}

pub struct CFAT32FileInfoSect {
    signature1 : u32,
    unknown : Vec<u8>,
    signature2 : u32,
    free_clus : u32,
    most_recently_alloced : u32,
    reserved : Vec<u8>,
    signature3 : u32,
}

pub struct CFreeLst<'a> {
    next : &'a mut CFreeLst<'a>,
    last : &'a mut CFreeLst<'a>,
    start : i64,
    size : i64,
}

pub struct CDrv<'a> {
    //Don't access ->drv_let directly in case a drive has been remapped.
    //Use $LK,"Drv2Let",A="MN:Drv2Let"$().
    locked_flags : i64,
    dv_signature : u32,
    drv_let : u8,
    pad : u8,
    fs_type : u16,
    drv_offset : i64,
    size : i64,
    prt_num : i64,
    file_system_info_sect : i64,
    fat1 : i64,
    fat2 : i64,
    root_clus : i64,
    data_area : i64,
    spc : i64, //sectors per clus
    fat32_local_time_offset : CDate,
    owning_task : &'a mut CTask<'a>,
    bd : &'a mut CBlkDev<'a>,
    fis : &'a mut CFAT32FileInfoSect,
    fat_blk_dirty : i64,
    cur_fat_blk_num : i64,
    cur_fat_blk : u32,
    next_free : &'a mut CFreeLst<'a>,
    last_free : &'a mut CFreeLst<'a>,
}

pub struct CTaskDying<'a> {
    next : &'a mut CTask<'a>,
    last : &'a mut CTask<'a>,
    wake_jiffy : i64,
}

pub struct CTSS {
    res1 : u32,
    rsp0 : i64,
    rsp1 : i64,
    rsp2 : i64,
    res2 : i64,
	ist1 : i64,
    ist2 : i64,
    ist3 : i64,
    ist4 : i64,
    ist5 : i64,
    ist6 : i64,
    ist7 : i64,
    res3 : i64,
    res4 : u16,
    io_map_offset : u16,
    io_map : Vec<u8>,
    st0 : i64,
    st1 : i64,
    st2 : i64,
    tr : u16,
    tr_ring3 : u16,
}

pub struct CCPU<'a> {
    adder : &'a mut CCPU<'a>,
    num : i64,
    cpu_flags : i64,
    startup_rip : i64,
	idle_pt_hits : i64,
    idle_factor : f64,
    total_jiffies : i64,
    seth_task : &'a mut CTask<'a>,
    idle_task : &'a mut CTask<'a>,
    tr : i64,   //task reg
	swap_cnter : i64,
    profiler_timer_irq : U0_with_CTask<'a>,
    next_dying : CTaskDying<'a>,
    last_dying : CTaskDying<'a>,
    kill_jiffy : i64,
    tss : &'a mut CTSS,
    start_stk : Vec<i64>,
}

pub struct CFPU {
    body : Vec<u8>,
}

pub struct CTaskStk<'a> {
    next_stk : &'a mut CTaskStk<'a>,
    stk_size : i64,
    stk_ptr : i64,
    stk_base : bool,
}

pub struct CExcept<'a> {
    next : &'a mut CExcept<'a>,
    last : &'a mut CExcept<'a>,
    hndlr_catch : i64,
    hndlr_untry : i64,
	rsp : i64,
    rbp : i64,
    rflags : i64,
    rsi : i64,
    rdi : i64,
    r10 : i64,
    r11 : i64,
    r12 : i64,
    r13 : i64,
    r14 : i64,
    r15 : i64,
}

pub struct CBpt<'a> {
    next : &'a mut CBpt<'a>,
    addr : &'a mut u8,
    val : u8,
    pad : Vec<u8>,
}

pub struct U0_With_CCtrl<'a> {
    val : &'a mut bool,
    c : &'a mut CCtrl<'a>,
}

pub struct U0_With_CDC<'a> {
    val : &'a mut bool,
    dc : &'a mut CDC<'a>,
    c : &'a mut CCtrl<'a>,
}

pub struct Bool_With_CCtrl<'a> {
    val : &'a mut bool,
    c : &'a mut CCtrl<'a>,
    x : i64,
    y : i64,
}

pub struct U0_With_CCtrl_And_Bool<'a> {
    val : &'a mut bool,
    c : &'a mut CCtrl<'a>,
    x : i64,
    y : i64,
    down : bool,
}

pub struct U0_With_CCtrl_And_I64<'a> {
    val : &'a mut bool,
    c : &'a mut CCtrl<'a>,
    delta : i64,
}

pub struct CCtrl<'a> {
    next : &'a mut CCtrl<'a>,
    last : &'a mut CCtrl<'a>,
    win_task : &'a mut CTask<'a>,
    _type : i64,
    flags : i64,
    left : i64,
    right : i64,
    top : i64,
    botoom : i64,
    scrn_left : i64,
    scrn_right : i64,
    scrn_top : i64,
    scrn_bottom : i64,
    state : &'a mut u8,
    update_derived_vals : U0_With_CCtrl<'a>,
    draw_it : U0_With_CDC<'a>,
    inside_ctrl : Bool_With_CCtrl<'a>,
    left_click : U0_With_CCtrl_And_Bool<'a>,
    right_click : U0_With_CCtrl_And_Bool<'a>,
    wheel_chg : U0_With_CCtrl_And_I64<'a>,
}

pub struct CMenuEntry<'a> {
    next : &'a mut CMenuEntry<'a>,
    sub : &'a mut CMenuEntry<'a>,
    name : Vec<u8>,
    msg_code : i64,
    arg1 : i64,
    arg2 : i64,
    checked : bool,
    dir : bool,
    pad : Vec<bool>,
}

pub struct CMenu<'a> {
    next : &'a mut CMenu<'a>,
    sub : &'a mut CMenuEntry<'a>,
    task : &'a mut CTask<'a>,
    flags : i64,
    attr : u8,
    pad : Vec<u8>,
}

pub struct U0_With_CTask_And_CDC<'a> {
    val : &'a mut bool,
    task : &'a mut CTask<'a>,
    dc : &'a mut CDC<'a>,
}

pub struct CTaskSettings<'a> {
    next : &'a mut CTaskSettings<'a>,
    cur_dir : &'a mut u8,
    left : i64,
    right : i64,
    top : i64,
    bottom : i64,
    draw_it : U0_With_CTask_And_CDC<'a>,
    task_end_cb : &'a mut bool,
    song_task : &'a mut CTask<'a>,
    animate_task : &'a mut CTask<'a>,
    scroll_x : i64,
    scroll_y : i64,
    scroll_z : i64,
    palette : &'a mut Vec<CBGR48>,
    win_inhibit : u32,
    text_attr : u8,
    title_src : u8,
	border_attr : u8,
    border_src : u8,
	task_title : Vec<u8>,
    border : bool,
    hide_cursor : bool,
    highlight_cursor : bool,
    scroll : bool,
    autocomplete : bool,
    pad : Vec<bool>,
}

pub struct U0_CMathODE_With_F64<'a> {
    val : &'a mut bool,
    o : &'a mut CMathODE<'a>,
    t : f64,
    state : &'a mut f64,
    DstateDt : &'a mut f64,
}

pub struct U0_CMathODE_With_I64<'a> {
    val : &'a mut bool,
    o : &'a mut CMathODE<'a>,
    t : f64,
    cpu_num : i64,
    state : &'a mut f64,
    DstateDt : &'a mut f64,
}

pub struct COrder2D3 {
    x : f64,
    y : f64,
    z : f64,
    DxDt : f64,
    DyDt : f64,
    DzDt : f64,
}

pub struct CMass<'a> {
    next : &'a mut CMass<'a>,
    last : &'a mut CMass<'a>,
    state : &'a mut COrder2D3, //Point to entries in $LK,"CMathODE",A="MN:CMathODE"$.state[]
    DstateDt : &'a mut COrder2D3, //Point to entries in $LK,"CMathODE",A="MN:CMathODE"$.DstateDt[]
    start : bool,
    flags : u32,
    num : u32,
    mass : f64,
    drag_profile_factor : f64,
    saved_state : bool,
    x : f64,
    y : f64,
    z : f64,
	DxDt : f64,
    DyDt : f64,
    DzDt : f64,
    end : bool,
}

pub struct CSpring<'a> {
    next : &'a mut CSpring<'a>,
    last : &'a mut CSpring<'a>,
    end1 : &'a mut CMass<'a>,
    end2 : &'a mut CMass<'a>,
    f : f64,
    displacement : f64,
    start : bool,
    flags : u32,
    num : u32,
	end1_num : u32,
    end2_num : u32,
    _const : f64,
    rest_len : f64,
    end : bool,
}

pub struct CMathODE<'a> {
    next : &'a mut CMathODE<'a>,
    last : &'a mut CMathODE<'a>,
    flags : i64,
    n : i64,
    n_internal : i64,
    next_mass : &'a mut CMass<'a>,
    last_mass : &'a mut CMass<'a>,
    next_spring : &'a mut CSpring<'a>,
    last_spring : &'a mut CSpring<'a>,
    drag_v : f64,  //drag proportional to velocity
	drag_v2 : f64, //drag proportional to velocity squared
	drag_v3 : f64, //drag proportional to velocity cubed
	acceleration_limit : f64, //This clips acceleration
	base_t : f64,
	t : f64,
    t_scale : f64,
	h : f64,
    h_min : f64,
    h_max : f64,
    min_tolerance : f64,
    max_tolerance : f64,
    tolerance_internal : &'a mut f64,
	array_base : &'a mut f64,
	state : &'a mut f64,
	state_internal : &'a mut f64,
	DstateDt : &'a mut f64,
	state_scale : &'a mut f64,
	initial_state : &'a mut f64,
	tmp0 : &'a mut f64,
    tmp1 : &'a mut f64,
    tmp2 : &'a mut f64,
    tmp3 : &'a mut f64,
	tmp4 : &'a mut f64,
    tmp5 : &'a mut f64,
    tmp6 : &'a mut f64,
    tmp7 : &'a mut f64,
    mem_task : &'a mut CTask<'a>,
    win_task : &'a mut CTask<'a>,
    derive : U0_CMathODE_With_F64<'a>,
    mp_derive : U0_CMathODE_With_I64<'a>,
    slave_tasks : &'a mut &'a mut CTask<'a>,
    mp_not_done_flags : i64,
    mp_t : f64,
    mp_state : &'a mut f64,
    mp_DstateDt : &'a mut f64,
    user_data : i64,
}

pub struct CHash<'a> {
    next : &'a mut CHash<'a>,
    str : &'a mut u8,
    _type : u32,
    use_cnt : u32,
}

pub const REGT_NONE : u8 = 0;
pub const REGT_R8 : u8 = 1;
pub const REGT_R16 : u8 = 2;
pub const REGT_R32 : u8 = 3;
pub const REGT_R64 : u8 = 4;
pub const REGT_SEG : u8 = 5;
pub const REGT_FSTK : u8 = 6;
pub const REGT_MM : u8 = 7;
pub const REGT_XMM : u8 = 8;

pub struct CHashReg<'a> {
    parent : CHash<'a>,
    reg_num : u8,
    reg_type : u8,
}

pub struct CHashTable<'a> {
    next : &'a mut CHashTable<'a>,
    mask : i64,
    locked_flags : i64,
    body : &'a mut &'a mut CHash<'a>,
}

pub struct I64_With_U8<'a> {
    val : &'a mut i64,
    fun_arg : &'a mut u8,
}

pub struct CJob<'a> {
    next : &'a mut CJob<'a>,
    last : &'a mut CJob<'a>,
    ctrl : &'a mut CJobCtrl<'a>,
    job_code : i64,
    flags : i64,
    msg_code : i64,
}

pub struct CJobCtrl<'a> {
    next_waiting : &'a mut CJob<'a>,
    last_waiting : &'a mut CJob<'a>,
    next_done : &'a mut CJob<'a>,
    last_done : &'a mut CJob<'a>,
    flags : i64,
    addr : I64_With_U8<'a>,
    fun_arg : &'a mut u8,
    aux_str : &'a mut u8,
    aux1 : i64,
    aux2 : i64, //Sometimes called arg1 and arg2. (Windows msg param1 param2)
	res : i64,
    spawned_task : &'a mut CTask<'a>,
	master_task : &'a mut CTask<'a>,
}

pub struct CMemberLstMeta<'a> {
    next : &'a mut CMemberLstMeta<'a>,
    str : &'a mut u8,
    flags : i64,
    user_data : i64,
}

pub struct CArrayDim<'a> {
    next : &'a mut CArrayDim<'a>,
    cnt : i64,
    total_cnt : i64,
}

pub struct CMemberLst<'a> {
    next : &'a mut CMemberLst<'a>,
    left : &'a mut CMemberLst<'a>,
    right : &'a mut CMemberLst<'a>,
	left_class_base : &'a mut CMemberLst<'a>,
    right_class_base : &'a mut CMemberLst<'a>,
    str : &'a mut u8,
    member_class : &'a mut CHashClass<'a>,
    member_class_base : &'a mut CHashClass<'a>,
    meta : &'a mut CMemberLstMeta<'a>,
    use_cnt : u32,
    flags : u16,
    reg : i8,
    pad : i8,
    offset : i64,
    size : i64,
    dim : CArrayDim<'a>,
    static_data : &'a mut u8,
    static_data_rip : i64,
    dft_val : i64,
    fun_ptr : &'a mut CHashFun<'a>,
}

pub struct CHashClass<'a> {
    size : i64,
    neg_offset : i64,
    member_cnt : u32,
    ptr_stars_cnt : u8,
    raw_type : u8,
    flags : u16,
    member_lst_and_root : &'a mut CMemberLst<'a>, //Head of linked list and head of tree.
	member_class_base_root : &'a mut CMemberLst<'a>, //For finding dup class local vars.
	last_in_member_lst : &'a mut CMemberLst<'a>,
    base_class : &'a mut CHashClass<'a>,
	fwd_class : &'a mut CHashClass<'a>,
}

pub struct CExternUsage<'a> {
    next : &'a mut CExternUsage<'a>,
    rip : i64,
}

pub struct CHashFun<'a> {
    child : CHashClass<'a>,
    return_class : &'a mut CHashClass<'a>,
    arg_cnt : u32,
    pad : u32,
	used_reg_mask : u32,
    clobbered_reg_mask : u32,
    exe_addr : &'a mut u8,
    ext_lst : CExternUsage<'a>,
}

pub struct CLexHashTableContext<'a> {
    next : &'a mut CLexHashTableContext<'a>,
    old_flags : i64,
    hash_mask : i64,
    local_var_lst : CHashFun<'a>,
    fun : CHashFun<'a>,
    hash_table_lst : CHashTable<'a>,
	define_hash_table : CHashTable<'a>,
	local_hash_table : CHashTable<'a>,
	glbl_hash_table : CHashTable<'a>
}

pub struct CHashGeneric<'a> {
    parent : CHash<'a>,
    user_data0 : i64,
    user_data1 : i64,
    user_data2 : i64,
}

pub struct CCodeMisc<'a> {
    next : &'a mut CCodeMisc<'a>,
    last : &'a mut CCodeMisc<'a>,
    fwd : &'a mut CCodeMisc<'a>,
    dft : &'a mut CCodeMisc<'a>,
    begin : &'a mut CCodeMisc<'a>,
    str : &'a mut u8,
    _type : u32,
    flags : u32,
    use_cnt : i64,
    addr : &'a mut u8,
    st_len : i64,		//STR_CONST
    num_consts : i64,	//FLOAT_CONSTS
    range : i64,
    rip : i64,		//ASM_LABEL
    jmp_table : &'a mut &'a mut CCodeMisc<'a>,
    float_consts : &'a mut f64,
    dim : &'a mut CArrayDim<'a>,
    h : &'a mut CHash<'a>,
}

pub struct CStreamBlk<'a> {
    next : &'a mut CStreamBlk<'a>,
    last : &'a mut CStreamBlk<'a>,
    body : &'a mut u8,
}

pub struct CAOTImportExport<'a> {
    next : &'a mut CAOTImportExport<'a>,
    last : &'a mut CAOTImportExport<'a>,
    rip : i64,
    flags : i64,
    aot : &'a mut CAOT<'a>,
    str : &'a mut u8,
    src_link : &'a mut u8,
    _type : &'a mut u8,
    pad : &'a mut Vec<u8>,
}

pub struct CAOT<'a> {
    next : &'a mut CAOT<'a>,
    last : &'a mut CAOT<'a>,
    buf : &'a mut u8,
    rip : i64,
    rip2 : i64,
    aot_U8s : i64,
    max_align_bits : i64,
    org : i64,
    parent_aot : &'a mut CAOT<'a>,
    next_ie : &'a mut CAOTImportExport<'a>,
    last_ie : &'a mut CAOTImportExport<'a>,
    abss : &'a mut CAOTAbsAddr<'a>,
    heap_glbls : &'a mut CAOTHeapGlbl<'a>,
}

pub struct CICType {
    val : u16,
    raw_type : u8,
    mode : u8,
}

pub struct CICArg {
    _type : CICType,
    reg : u16, //low is reg, high is index_reg+scale<<6
    disp : i64,
}

pub struct CICTreeLinks<'a> {
    arg1_class : &'a mut CHashClass<'a>,
    arg2_class : &'a mut CHashClass<'a>,
    arg1_tree : &'a mut CIntermediateCode<'a>,
    arg2_tree : &'a mut CIntermediateCode<'a>,
    class2 : &'a mut CHashClass<'a>,
}

pub struct CIntermediateCode<'a> {
    ic_flags : i64,
	ic_data : i64,
	ic_line : i64,
    ic_class : &'a mut CHashClass<'a>,
    ic_class2 : &'a mut CHashClass<'a>,
    arg1 : CICArg,
    arg2 : CICArg,
    res : CICArg,
    arg1_type_pointed_to : u8,
    ic_body : Vec<u8>,
    t : CICTreeLinks<'a>,
}

pub struct CIntermediateCodeBase<'a> {
    next : &'a mut CIntermediateCodeBase<'a>,
    last : &'a mut CIntermediateCodeBase<'a>,
    ic_code : u16,
    ic_precedence : u16,
    ic_cnt : i16,
    ic_last_start : i16,
}

pub struct CCodeCtrl<'a> {
    coc_next : &'a mut CCodeCtrl<'a>,
    coc_next_misc : &'a mut CCodeMisc<'a>,
    coc_last_misc : &'a mut CCodeMisc<'a>,
    coc_head : CIntermediateCodeBase<'a>,
}

pub struct CPrsStk {
    ptr : i64,
    stk : Vec<i64>,
    ptr2 : i64,
    stk2 : Vec<i64>,
}

pub struct CCmpCtrl<'a> {
    next : &'a mut CCmpCtrl<'a>,
    last : &'a mut CCmpCtrl<'a>,
    token : i64,
	flags : i64,
	cur_i64 : i64,
    cur_f64 : f64,
    cur_str : &'a mut u8,
    cur_str_len : i64,
	class_dol_offset : i64,
    dollar_buf : &'a mut u8,
    dollar_cnt : i64,
    cur_help_idx : &'a mut u8,
    last_U16 : i64,
	min_line : i64,
    max_line : i64,
    last_line_num : i64,
	lock_cnt : i64,
    char_bmp_alpha_numeric : &'a mut u32,
    htc : CLexHashTableContext<'a>,
    hash_entry : Option<&'a mut CHashGeneric>,
    abs_cnts : CAbsCntsI64,
    asm_undef_hash : Option<&'a mut CAsmUndefHash<'a>>,
    local_var_entry : &'a mut CMemberLst<'a>,
    lb_leave : &'a mut CCodeMisc<'a>,
    cur_buf_ptr : &'a mut u8,
    lex_include_stk : &'a mut CLexFile<'a>,
	lex_prs_stk : &'a mut CLexFile<'a>,
	fun_lex_file : &'a mut CLexFile<'a>,
    next_stream_blk : &'a mut CStreamBlk<'a>,
    last_stream_blk : &'a mut CStreamBlk<'a>,
    aot : &'a mut CAOT<'a>,
    pass : i64,
    opts : i64,
    pass_trace : i64,
    saved_pass_trace : i64,
	error_cnt : i64,
    warning_cnt : i64,
    cur_ic_float_op_num : i64,
    last_ic_float_op_num : i64,
    last_float_op_ic : &'a mut CIntermediateCode<'a>,
    last_dont_pushable : bool,
    last_dont_popable : bool,
    last_float_op_pos : bool,
	dont_push_float : bool,
    pad : Vec<bool>,
    coc : CCodeCtrl<'a>,
    ps : &'a mut CPrsStk,
    aotc : &'a mut CAOTCtrl<'a>,
    aot_depth : i64,
    pmt_line : i64,
}

pub struct CWinScroll {
    min : i64,
    pos : i64,
    max : i64,
    flags : u32,
    color : u8,
    pad : Vec<u8>,
}

pub struct CTask<'a> {
    addr : &'a mut CTask<'a>,
    task_signature : u32,
    win_inhibit : u32,
    wake_jiffy : i64,
    task_flags : u32,
    display_flags : u32,
    code_heap : &'a mut CHeapCtrl<'a>,
    data_heap : &'a mut CHeapCtrl<'a>,
    put_doc : &'a mut CDoc<'a>,
    display_doc : &'a mut CDoc<'a>,
    border_doc : &'a mut CDoc<'a>,
    win_left : i64,
    win_right : i64,
    win_top : i64,
    win_bottom : i64,
    cur_dv : &'a mut CDrv<'a>,
    cur_dir : &'a mut u8,
    parent_task : &'a mut CTask<'a>,
    next_task : &'a mut CTask<'a>,
    last_task : &'a mut CTask<'a>,
	next_input_filter_task : &'a mut CTask<'a>,
    last_input_filter_task : &'a mut CTask<'a>,
	next_sibling_task : &'a mut CTask<'a>,
    last_sibling_task : &'a mut CTask<'a>,
	next_child_task : &'a mut CTask<'a>,
    last_child_task : &'a mut CTask<'a>,
    win_width : i64,
    win_height : i64,
	pix_left : i64,
    pix_right : i64,
    pix_width : i64, //These are in pixs, not characters
	pix_top : i64,
    pix_bottom : i64,
    pix_height : i64,
	scroll_x : i64,
    scroll_y : i64,
    scroll_z : i64,

    //These must be in this order
    //for $LK,"TASK_CONTEXT_SAVE",A="FF:::/Kernel/Sched.HC,TASK_CONTEXT_SAVE"$ and $LK,"_TASK_CONTEXT_RESTORE",A="FF:::/Kernel/Sched.HC,_TASK_CONTEXT_RESTORE"$
    rip : i64,
    rflags : i64,
    rsp : i64,
    rsi : i64,
    rax : i64,
    rcx : i64,
    rdx : i64,
    rbx : i64,
    rbp : i64,
    rdi : i64,
	r8 : i64,
    r9 : i64,
    r10 : i64,
    r11 : i64,
    r12 : i64,
    r13 : i64,
    r14 : i64,
    r15 : i64,
    gs : &'a mut CCPU<'a>,
    fpu_mmx : &'a mut CFPU,
    swap_cnter : i64,
    draw_it : U0_with_CTask<'a>,
    task_title : Vec<u8>,
    task_name : Vec<u8>,
    wallpaper_data : Vec<u8>,
    title_src : Vec<u8>,
    border_src : Vec<u8>,
    border_attr : Vec<u8>,
    win_z_num : u16,
    u16_pad : u16,
    stk : &'a mut CTaskStk<'a>,
    next_except : &'a mut CExcept<'a>,
    last_except : &'a mut CExcept<'a>,
    except_rbp : i64,
    except_ch : i64,
    except_callers : &'a mut Vec<u8>,
    catch_except : bool,
    new_answer : bool,
    answer_type : u8,
    u8_pad : Vec<u8>,
    answer : i64,
    answer_time : f64,
    bpt_list : &'a mut CBpt<'a>,
    next_ctrl : &'a mut CCtrl<'a>,
    last_ctrl : &'a mut CCtrl<'a>,
    cur_menu : &'a mut CMenu<'a>,
    next_settings : &'a mut CTaskSettings<'a>,
    next_ode : &'a mut CMathODE<'a>,
    last_ode : &'a mut CMathODE<'a>,
    last_ode_time : f64,
    hash_table : &'a mut CHashTable<'a>,
    srv_ctrl : CJobCtrl<'a>,
    next_cc : &'a mut CCmpCtrl<'a>,
    last_cc : &'a mut CCmpCtrl<'a>,
    last_fun : &'a mut CHashFun<'a>,
    task_end_cb : &'a mut u16,
    song_task : &'a mut CTask<'a>,
    animate_task : &'a mut CTask<'a>,
    rand_seed : i64,
    task_num : i64,
    fault_num : i64,
    fault_err_code : i64,
    popup_task : &'a mut CTask<'a>,
    dbg_task : &'a mut CTask<'a>,
    horz_scroll : CWinScroll,
    vert_scroll : CWinScroll,
    start_time : CDate,
    user_data : i64,
}

pub struct U8_with_doc<'a> {
    doc : &'a mut CDoc<'a>,
    doc_e : &'a mut CDocEntry<'a>,
    mem_task : &'a mut CTask<'a>,
}

pub struct CDocBin<'a> {
    next : &'a mut CDocBin<'a>,
    last : &'a mut CDocBin<'a>,
    tmp_use_cnt : i32,
    renum_num : i32,
    tags : &'a mut u8,
    start : bool,
    num : u32,
    flags : u32,
    size : u32,
    use_cnt : u32,
    end : bool,
    data : &'a mut u8,
}

pub struct CDocEntry<'a> {
    child : CDocEntryBase<'a>,
    attr : i64,
    cursor_x_offset : i64,
    left_cb : I64_with_doc<'a>,
    left_exp : i64,
    left_macro : &'a mut u8,
    cursor_y_offset : i64,
    right_cb : I64_with_doc<'a>,
    right_exp : i64,
    right_macro : &'a mut u8,
    tag_cb : &'a mut U8_with_doc<'a>,
    define_str : &'a mut u8,
    aux_str : &'a mut u8,
	bin_ptr_link : &'a mut u8,
	html_link : &'a mut u8,
	my_fmt_data : &'a mut u8,
    hex_ed_width : i64,
    scroll_len : i32,
    len : i32, //$LK,"DOCE_LEN_DFT",A="MN:DOCE_LEN_DFT"$
    bin_num : i32,
    raw_type : u8,
    pad : Vec<u8>,
    bin_data : &'a mut CDocBin<'a>,
    data : &'a mut u8,
}

pub struct CEdFindText {
    // Lets hope to god this isn't needed...
}

pub struct CEdFileName {
    // Lets hope to god this isn't needed...
}

pub struct CDocUndo<'a> {
    next : &'a mut CDocUndo<'a>,
    last : &'a mut CDocUndo<'a>,
    size : i64,
    doc_flags : i64,
    time_stamp : i64,
    body : &'a mut u8,
}

pub struct CDoc<'a> { //Linked Text File header
    head : CDocEntryBase<'a>,
    flags : i64,
    locked_flags : i64,
    cur_entry : &'a mut CDocEntry<'a>,
    old_cur_entry : &'a mut CDocEntry<'a>,
    cur_col : i32,
    old_cur_col : i32,
    line_start_col : i32,
    top_line_num : i32,
    dollar_buf_size : i32,
    dollar_buf_ptr : i32,
    dollar_buf : &'a mut u8, //When entering $$ cmds, it buffers them until the end $$.
    win_task : &'a mut CTask<'a>,
    mem_task : &'a mut CTask<'a>,
    owning_task : &'a mut CTask<'a>,
    page_line_num : i32,
    undo_cnt : i32,
    x : i32,
    y : i32,
    min_x : i32,
    max_x : i32,
    min_y : i32,
    max_y : i32,
    line : i64,
    col : i64,
    best_d : i64,
    old_win_top : i64,
    old_win_bottom : i64,
    old_win_left : i64,
    old_win_right : i64,
    cmd_U8 : i64,
    doc_signature : u32,
    cur_bin_num : u32,
    max_entries : i64,
    updates_cnt : i64,
    find_replace : &'a mut CEdFindText,
    filename : CEdFileName,
    file_attr : i64,
    left_click_link : I64_with_doc<'a>,
    right_click_link : I64_with_doc<'a>,
  
    //See $LK,"::/Apps/Psalmody/JukeBox.HC"$
    user_put_data : &'a mut u8, //Passed to user_put_key() and user_put_s()
    user_put_key : &'a mut Put_Key<'a>,
    user_put_s : &'a mut Put_S<'a>,
  
    parent_doc : Option<&'a mut CDoc<'a>>, //(When browsing deeper, opening deeper docs.)
    desc : u64, //8 characters. See $LK,"DocBorderLstDraw",A="MN:DocBorderLstDraw"$().
  
    bin_head : CDocBin<'a>,
    settings_head : CDocSettings,
    undo_head : CDocUndo<'a>,
  
    user_data : i64,
}

pub struct CLexFile<'a> {
    next : Option<&'a mut CLexFile<'a>>,
    buf : &'a mut u8,
    buf_ptr : &'a mut u8,
    line_num : i64,
    flags : i64,
    full_name : &'a mut u8,
    line_start : &'a mut u8,
    doc : &'a mut CDoc<'a>,
    cur_entry : &'a mut CDocEntry<'a>,
    depth : i32,
    last_u16 : u8,
    pad : Vec<u8>
}

pub struct CAOTBinBlk<'a> {
    next : &'a mut CAOTBinBlk<'a>,
    body : Vec<u8>,
}

pub struct CAsmUnresolvedRef<'a> {
    next : &'a mut CAsmUnresolvedRef<'a>,
    _type : i64,
    line_num : i64,
    machine_code : &'a mut u8,
    rip : i64,
    rel_rip : i64,
    aot : &'a mut CAOT<'a>,
    str : &'a mut u8,
    asm_undef_hash : &'a mut CAsmUndefHash<'a>,
    U8_avail : bool,
    imm_flag : bool,
}

pub struct CAOTCtrl<'a> {
    rip : i64, //Inst ptr
    arg1 : CAsmArg,
    arg2 : CAsmArg,
    bin : &'a mut CAOTBinBlk<'a>,
    num_bin_U8s : i64,
    max_align_bits : i64,
    org : i64,
    local_unresolved : &'a mut CAsmUnresolvedRef<'a>,
    glbl_unresolved : &'a mut CAsmUnresolvedRef<'a>,
    abss : &'a mut CAOTAbsAddr<'a>,
    heap_glbls : &'a mut CAOTHeapGlbl<'a>,
    lst_col : i64,
    lst_last_rip : i64,
    last_label : &'a mut u8,
    lst_last_line : &'a mut u8,
    lst_last_lfn : &'a mut CLexFile<'a>,
    seg_size : i64,
    lst : bool,
}