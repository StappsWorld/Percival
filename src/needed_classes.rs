// THESE CLASSES COME DIRECTLY FROM /Kernel/Kernel*.HH

#[derive(Default)]
pub struct CHashExport {
    pub val : i64,
}

#[derive(Default)]
pub struct CAsmUndefHash<'a> {
    pub next : Option<&'a mut CAsmUndefHash<'a>>,
    pub hash : CHashExport,
}

pub struct CAbsCntsI64 {
    pub abs_addres : u16, //Only odd/even matters. Cnt of absolute addres in an exp.
	pub c_addres : u16, //Only odd/even matters. Cnt of C addres in an exp.
    pub externs : u32, //Only nonzero matters. Some regions have externs banned.
}

pub struct CAsmNum<'a> {
    pub i : i64,
    pub machine_code : u8,
    pub local_asm_undef_hash : Option<CAsmUndefHash<'a>>,
    pub glbl_asm_undef_hash : Option<CAsmUndefHash<'a>>,
    pub abs_cnts : CAbsCntsI64,
}

pub struct CAsmNum2<'a> {
    pub num : CAsmNum<'a>,
    pub U8_cnt : i64,
    pub rel : i64,
    pub imm_flag : bool,
}

pub struct CAsmArg<'a> {
    pub num : CAsmNum<'a>,
    pub seg : i64,
    pub size : i64,
    pub reg1 : i64,
    pub reg2 : i64,
	pub reg1_type : i64,
    pub reg2_type : i64,
	pub scale : i64,
    pub indirect : bool,
    pub imm_or_off_present : bool,
    pub just_seg : bool,
    pub pad : Vec<bool>,
}

pub struct CAOTAbsAddr<'a> {
    pub next : Option<&'a mut CAOTAbsAddr<'a>>,
    pub rip : i64,
    pub _type : u8,
    pub pad : Vec<u8>,
}

pub struct CAOTHeapGlblRef<'a> {
    pub next : Option<&'a mut CAOTHeapGlblRef<'a>>,
    pub rip : i64,
}

pub struct CAOTHeapGlbl<'a> {
    pub next : Option<&'a mut CAOTHeapGlbl<'a>>,
    pub _str : u8,
    pub size : i64,
    pub references : &'a mut CAOTHeapGlblRef<'a>,
}

pub struct I64_with_doc<'a> {
    pub val : &'a mut i64,
    pub doc : &'a mut CDoc<'a>,
    pub doc_e : &'a mut CDocEntry<'a>,
}

pub struct Put_Key<'a> {
    pub val : &'a mut bool,
    pub doc : &'a mut CDoc<'a>,
    pub data : &'a mut u8,
    pub ch : i64,
    pub sc : i64,
}

pub struct Put_S<'a> {
    pub val : &'a mut bool,
    pub doc : &'a mut CDoc<'a>,
    pub data : &'a mut u8,
    pub st : &'a mut u8,
}

pub struct CDocSettings {
    pub final_u32_attr : u32,
    pub left_margin : i16,
    pub right_margin : i16,
    pub indent : i16,
    pub page_len : u16,
    pub header : u16,
    pub footer : u16,
    pub shifted_x : i8,
    pub shifted_y : i8,
    pub state : u8,
    pub comment_depth : u8,
    pub paren_depth : u8,
    pub brace_depth : u8,
	pub cur_text_attr : u8,
    pub dft_text_attr : u8,
}

pub struct CDocEntryBase<'a> {
    pub next : Option<&'a mut CDocEntryBase<'a>>,
    pub last : Option<&'a mut CDocEntryBase<'a>>,
    pub tag : &'a mut u8,
    pub type_u8 : u8,
    pub _type : u32,
    pub page_line_num : i32,
    pub de_flags : i64,
    pub x : i32,
    pub y : i32,
    pub min_col : u32,
    pub max_col : u32,
    pub settings : CDocSettings,
    pub user_data : i64,
}

pub struct CDate {
    pub time : u32,
    pub date : i32,
}

pub struct CBGR48 {
    pub r : u16,
    pub g : u16,
    pub b : u16,
    pub pad : u16,
}

pub struct CDC<'a> {
    pub start : bool,
    pub cdt : CDate,
    pub x0 : i32,
    pub y0 : i32,
    pub width : i32,
    pub width_internal : i32,
	pub height : i32,
	pub flags : i32,
    pub end : bool,
    pub palette : CBGR48,
    pub body : &'a mut u8,
}

pub struct U0_with_CTask<'a> {
    pub val : &'a mut u16,
    pub task : &'a mut CTask<'a>,
    pub dc : &'a mut CDC<'a>,
}

pub struct CMemBlk<'a> {
    pub next : Option<&'a mut CMemBlk<'a>>,
    pub last : &'a mut CMemBlk<'a>,
    pub mb_signature : u32,
    pub pags : u32,
}

pub struct CBlkPool<'a> {
    pub locked_flags : i64,
    pub alloced_u8s : i64,
    pub used_u8s : i64,
    pub mem_free_lst : &'a mut CMemBlk<'a>,
	pub mem_free_2meg_lst : &'a mut CMemBlk<'a>, //This is for Sup1CodeScraps/Mem/Mem2Meg.HC.
    pub free_pag_hash : &'a mut Vec<CMemBlk<'a>>,
    pub free_pag_hash2 : &'a mut Vec<CMemBlk<'a>>,
}

pub struct CMemUnused<'a> {
    pub hc : bool,
    pub caller1 : bool,
    pub caller2 : bool,
    pub next : Option<&'a mut CMemUnused<'a>>,
    pub size : i64,
}

pub struct CMemUsed<'a> {
    pub hc : &'a mut CHeapCtrl<'a>,
    pub caller1 : bool,
    pub caller2 : bool,
    pub next : bool,
    pub last : bool,
    pub size : i64,
    pub start : bool,
}

pub struct CHeapCtrl<'a> {
    pub bp : &'a mut CBlkPool<'a>,
    pub hc_signature : u32,
    pub pad : u32,
    pub locked_flags : i64,
    pub alloced_u8s : i64,
    pub used_u8s : i64,
    pub mem_task : &'a mut CTask<'a>,
    pub next_mem_blk : &'a mut CMemBlk<'a>,
    pub last_mem_blk : &'a mut CMemBlk<'a>,
    pub last_mergable : &'a mut CMemBlk<'a>,
    pub malloc_free_lst : &'a mut CMemUnused<'a>,
    pub next_um : &'a mut CMemUsed<'a>,
    pub last_um : &'a mut CMemUsed<'a>,
    pub heap_hash : &'a mut Vec<CMemUnused<'a>>,
}

pub struct CDirEntry<'a> {
    pub next : &'a mut CDirEntry<'a>,
    pub parent : &'a mut CDirEntry<'a>,
    pub sub : &'a mut CDirEntry<'a>,
    pub full : &'a mut u8,
    pub user_data : i64,
    pub user_data2 : i64,
    pub start : bool,
    pub attr : u16,
    pub name : Vec<u8>,
    pub clus : i64,
    pub size : i64,
    pub datetime : CDate,
}

pub struct CFile<'a> {
  pub   flags : i64,
  pub  de : CDirEntry<'a>,
  pub dv : &'a mut CDrv<'a>,
  pub fblk_num : i64,
  pub clus : i64,
  pub file_clus_num : i64,
  pub max_blk : i64,
  pub clus_buf : &'a mut u8,
}

pub struct CBlkDev<'a> {
  pub   lock_fwding : &'a mut CBlkDev<'a>, //If two blkdevs on same controller, use just one lock
  pub locked_flags : i64,
  pub bd_signature : u32,
pub 	_type : u32,
  pub   flags : u32,
  pub first_drv_let : u8,
  pub unit : u8,
  pub pad : Vec<u8>,
  pub base0 : u32,
  pub base1 : u32,
  pub   blk_size : u32,
  pub drv_offset : i64,
  pub init_root_dir_blks : i64,
pub 	max_blk : i64,
  pub dev_id_record : &'a mut u16,
  pub RAM_dsk : &'a mut u8,
  pub   file_dsk_name : &'a mut u8,
  pub file_dsk : &'a mut CFile<'a>,
  pub owning_task : &'a mut CTask<'a>,
  pub last_time : f64,
  pub max_reads : u32,
  pub max_writes : u32,
}

pub struct CFAT32FileInfoSect {
    pub signature1 : u32,
    pub unknown : Vec<u8>,
    pub signature2 : u32,
    pub free_clus : u32,
    pub most_recently_alloced : u32,
    pub reserved : Vec<u8>,
    pub signature3 : u32,
}

pub struct CFreeLst<'a> {
    pub next : &'a mut CFreeLst<'a>,
    pub last : &'a mut CFreeLst<'a>,
    pub start : i64,
    pub size : i64,
}

pub struct CDrv<'a> {
    //Don't access ->drv_let directly in case a drive has been remapped.
    //Use $LK,"Drv2Let",A="MN:Drv2Let"$().
    pub locked_flags : i64,
    pub dv_signature : u32,
    pub drv_let : u8,
    pub pad : u8,
    pub fs_type : u16,
    pub drv_offset : i64,
    pub size : i64,
    pub prt_num : i64,
    pub file_system_info_sect : i64,
    pub fat1 : i64,
    pub fat2 : i64,
    pub root_clus : i64,
    pub data_area : i64,
    pub spc : i64, //sectors per clus
    pub fat32_local_time_offset : CDate,
    pub owning_task : &'a mut CTask<'a>,
    pub bd : &'a mut CBlkDev<'a>,
    pub fis : &'a mut CFAT32FileInfoSect,
    pub fat_blk_dirty : i64,
    pub cur_fat_blk_num : i64,
    pub cur_fat_blk : u32,
    pub next_free : &'a mut CFreeLst<'a>,
    pub last_free : &'a mut CFreeLst<'a>,
}

pub struct CTaskDying<'a> {
    pub next : &'a mut CTask<'a>,
    pub last : &'a mut CTask<'a>,
    pub wake_jiffy : i64,
}

pub struct CTSS {
    pub res1 : u32,
    pub rsp0 : i64,
    pub rsp1 : i64,
    pub rsp2 : i64,
    pub res2 : i64,
	pub ist1 : i64,
    pub ist2 : i64,
    pub ist3 : i64,
    pub ist4 : i64,
    pub ist5 : i64,
    pub ist6 : i64,
    pub ist7 : i64,
    pub res3 : i64,
    pub res4 : u16,
    pub io_map_offset : u16,
    pub io_map : Vec<u8>,
    pub st0 : i64,
    pub st1 : i64,
    pub st2 : i64,
    pub tr : u16,
    pub tr_ring3 : u16,
}

pub struct CCPU<'a> {
    pub adder : &'a mut CCPU<'a>,
    pub num : i64,
    pub cpu_flags : i64,
    pub startup_rip : i64,
	pub idle_pt_hits : i64,
    pub idle_factor : f64,
    pub total_jiffies : i64,
    pub seth_task : &'a mut CTask<'a>,
    pub idle_task : &'a mut CTask<'a>,
    pub tr : i64,   //task reg
	pub swap_cnter : i64,
    pub profiler_timer_irq : U0_with_CTask<'a>,
    pub next_dying : CTaskDying<'a>,
    pub last_dying : CTaskDying<'a>,
    pub kill_jiffy : i64,
    pub tss : &'a mut CTSS,
    pub start_stk : Vec<i64>,
}

pub struct CFPU {
    pub body : Vec<u8>,
}

pub struct CTaskStk<'a> {
    pub next_stk : &'a mut CTaskStk<'a>,
    pub stk_size : i64,
    pub stk_ptr : i64,
    pub stk_base : bool,
}

pub struct CExcept<'a> {
    pub next : &'a mut CExcept<'a>,
    pub last : &'a mut CExcept<'a>,
    pub hndlr_catch : i64,
    pub hndlr_untry : i64,
	pub rsp : i64,
    pub rbp : i64,
    pub rflags : i64,
    pub rsi : i64,
    pub rdi : i64,
    pub r10 : i64,
    pub r11 : i64,
    pub r12 : i64,
    pub r13 : i64,
    pub r14 : i64,
    pub r15 : i64,
}

pub struct CBpt<'a> {
    pub next : &'a mut CBpt<'a>,
    pub addr : &'a mut u8,
    pub val : u8,
    pub pad : Vec<u8>,
}

pub struct U0_With_CCtrl<'a> {
    pub val : &'a mut bool,
    pub c : &'a mut CCtrl<'a>,
}

pub struct U0_With_CDC<'a> {
    pub val : &'a mut bool,
    pub dc : &'a mut CDC<'a>,
    pub c : &'a mut CCtrl<'a>,
}

pub struct Bool_With_CCtrl<'a> {
    pub val : &'a mut bool,
    pub c : &'a mut CCtrl<'a>,
    pub x : i64,
    pub y : i64,
}

pub struct U0_With_CCtrl_And_Bool<'a> {
    pub val : &'a mut bool,
    pub c : &'a mut CCtrl<'a>,
    pub x : i64,
    pub y : i64,
    pub down : bool,
}

pub struct U0_With_CCtrl_And_I64<'a> {
    pub val : &'a mut bool,
    pub c : &'a mut CCtrl<'a>,
    pub delta : i64,
}

pub struct CCtrl<'a> {
    pub next : &'a mut CCtrl<'a>,
    pub last : &'a mut CCtrl<'a>,
    pub win_task : &'a mut CTask<'a>,
    pub _type : i64,
    pub flags : i64,
    pub left : i64,
    pub right : i64,
    pub top : i64,
    pub botoom : i64,
    pub scrn_left : i64,
    pub scrn_right : i64,
    pub scrn_top : i64,
    pub scrn_bottom : i64,
    pub state : &'a mut u8,
    pub update_derived_vals : U0_With_CCtrl<'a>,
    pub draw_it : U0_With_CDC<'a>,
    pub inside_ctrl : Bool_With_CCtrl<'a>,
    pub left_click : U0_With_CCtrl_And_Bool<'a>,
    pub right_click : U0_With_CCtrl_And_Bool<'a>,
    pub wheel_chg : U0_With_CCtrl_And_I64<'a>,
}

pub struct CMenuEntry<'a> {
    pub next : &'a mut CMenuEntry<'a>,
    pub sub : &'a mut CMenuEntry<'a>,
    pub name : Vec<u8>,
    pub msg_code : i64,
    pub arg1 : i64,
    pub arg2 : i64,
    pub checked : bool,
    pub dir : bool,
    pub pad : Vec<bool>,
}

pub struct CMenu<'a> {
    pub next : &'a mut CMenu<'a>,
    pub sub : &'a mut CMenuEntry<'a>,
    pub task : &'a mut CTask<'a>,
    pub flags : i64,
    pub attr : u8,
    pub pad : Vec<u8>,
}

pub struct U0_With_CTask_And_CDC<'a> {
    pub val : &'a mut bool,
    pub task : &'a mut CTask<'a>,
    pub dc : &'a mut CDC<'a>,
}

pub struct CTaskSettings<'a> {
    pub next : &'a mut CTaskSettings<'a>,
    pub cur_dir : &'a mut u8,
    pub left : i64,
    pub right : i64,
    pub top : i64,
    pub bottom : i64,
    pub draw_it : U0_With_CTask_And_CDC<'a>,
    pub task_end_cb : &'a mut bool,
    pub song_task : &'a mut CTask<'a>,
    pub animate_task : &'a mut CTask<'a>,
    pub scroll_x : i64,
    pub scroll_y : i64,
    pub scroll_z : i64,
    pub palette : &'a mut Vec<CBGR48>,
    pub win_inhibit : u32,
    pub text_attr : u8,
    pub title_src : u8,
	pub border_attr : u8,
    pub border_src : u8,
	pub task_title : Vec<u8>,
    pub border : bool,
    pub hide_cursor : bool,
    pub highlight_cursor : bool,
    pub scroll : bool,
    pub autocomplete : bool,
    pub pad : Vec<bool>,
}

pub struct U0_CMathODE_With_F64<'a> {
    pub val : &'a mut bool,
    pub o : &'a mut CMathODE<'a>,
    pub t : f64,
    pub state : &'a mut f64,
    pub DstateDt : &'a mut f64,
}

pub struct U0_CMathODE_With_I64<'a> {
    pub val : &'a mut bool,
    pub o : &'a mut CMathODE<'a>,
    pub t : f64,
    pub cpu_num : i64,
    pub state : &'a mut f64,
    pub DstateDt : &'a mut f64,
}

pub struct COrder2D3 {
    pub x : f64,
    pub y : f64,
    pub z : f64,
    pub DxDt : f64,
    pub DyDt : f64,
    pub DzDt : f64,
}

pub struct CMass<'a> {
    pub next : &'a mut CMass<'a>,
    pub last : &'a mut CMass<'a>,
    pub state : &'a mut COrder2D3, //Point to entries in $LK,"CMathODE",A="MN:CMathODE"$.state[]
    pub DstateDt : &'a mut COrder2D3, //Point to entries in $LK,"CMathODE",A="MN:CMathODE"$.DstateDt[]
    pub start : bool,
    pub flags : u32,
    pub num : u32,
    pub mass : f64,
    pub drag_profile_factor : f64,
    pub saved_state : bool,
    pub x : f64,
    pub y : f64,
    pub z : f64,
	pub DxDt : f64,
    pub DyDt : f64,
    pub DzDt : f64,
    pub end : bool,
}

pub struct CSpring<'a> {
    pub next : &'a mut CSpring<'a>,
    pub last : &'a mut CSpring<'a>,
    pub end1 : &'a mut CMass<'a>,
    pub end2 : &'a mut CMass<'a>,
    pub f : f64,
    pub displacement : f64,
    pub start : bool,
    pub flags : u32,
    pub num : u32,
	pub end1_num : u32,
    pub end2_num : u32,
    pub _const : f64,
    pub rest_len : f64,
    pub end : bool,
}

pub struct CMathODE<'a> {
    pub next : &'a mut CMathODE<'a>,
    pub last : &'a mut CMathODE<'a>,
    pub flags : i64,
    pub n : i64,
    pub n_internal : i64,
    pub next_mass : &'a mut CMass<'a>,
    pub last_mass : &'a mut CMass<'a>,
    pub next_spring : &'a mut CSpring<'a>,
    pub last_spring : &'a mut CSpring<'a>,
    pub drag_v : f64,  //drag proportional to velocity
	pub drag_v2 : f64, //drag proportional to velocity squared
	pub drag_v3 : f64, //drag proportional to velocity cubed
	pub acceleration_limit : f64, //This clips acceleration
	pub base_t : f64,
	pub t : f64,
    pub t_scale : f64,
	pub h : f64,
    pub h_min : f64,
    pub h_max : f64,
    pub min_tolerance : f64,
    pub max_tolerance : f64,
    pub tolerance_internal : &'a mut f64,
	pub array_base : &'a mut f64,
	pub state : &'a mut f64,
	pub state_internal : &'a mut f64,
	pub DstateDt : &'a mut f64,
	pub state_scale : &'a mut f64,
	pub initial_state : &'a mut f64,
	pub tmp0 : &'a mut f64,
    pub tmp1 : &'a mut f64,
    pub tmp2 : &'a mut f64,
    pub tmp3 : &'a mut f64,
	pub tmp4 : &'a mut f64,
    pub tmp5 : &'a mut f64,
    pub tmp6 : &'a mut f64,
    pub tmp7 : &'a mut f64,
    pub mem_task : &'a mut CTask<'a>,
    pub win_task : &'a mut CTask<'a>,
    pub derive : U0_CMathODE_With_F64<'a>,
    pub mp_derive : U0_CMathODE_With_I64<'a>,
    pub slave_tasks : &'a mut &'a mut CTask<'a>,
    pub mp_not_done_flags : i64,
    pub mp_t : f64,
    pub mp_state : &'a mut f64,
    pub mp_DstateDt : &'a mut f64,
    pub user_data : i64,
}

pub struct CHash<'a> {
    pub next : &'a mut CHash<'a>,
    pub str : &'a mut u8,
    pub _type : u32,
    pub use_cnt : u32,
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
    pub parent : CHash<'a>,
    pub reg_num : u8,
    pub reg_type : u8,
}

pub struct CHashTable<'a> {
    pub next : &'a mut CHashTable<'a>,
    pub mask : i64,
    pub locked_flags : i64,
    pub body : &'a mut &'a mut CHash<'a>,
}

pub struct I64_With_U8<'a> {
    pub val : &'a mut i64,
    pub fun_arg : &'a mut u8,
}

pub struct CJob<'a> {
    pub next : &'a mut CJob<'a>,
    pub last : &'a mut CJob<'a>,
    pub ctrl : &'a mut CJobCtrl<'a>,
    pub job_code : i64,
    pub flags : i64,
    pub msg_code : i64,
}

pub struct CJobCtrl<'a> {
    pub next_waiting : &'a mut CJob<'a>,
    pub last_waiting : &'a mut CJob<'a>,
    pub next_done : &'a mut CJob<'a>,
    pub last_done : &'a mut CJob<'a>,
    pub flags : i64,
    pub addr : I64_With_U8<'a>,
    pub fun_arg : &'a mut u8,
    pub aux_str : &'a mut u8,
    pub aux1 : i64,
    pub aux2 : i64, //Sometimes called arg1 and arg2. (Windows msg param1 param2)
	pub res : i64,
    pub spawned_task : &'a mut CTask<'a>,
	pub master_task : &'a mut CTask<'a>,
}

pub struct CMemberLstMeta<'a> {
    pub next : &'a mut CMemberLstMeta<'a>,
    pub str : &'a mut u8,
    pub flags : i64,
    pub user_data : i64,
}

pub struct CArrayDim<'a> {
    pub next : &'a mut CArrayDim<'a>,
    pub cnt : i64,
    pub total_cnt : i64,
}

pub struct CMemberLst<'a> {
    pub next : &'a mut CMemberLst<'a>,
    pub left : &'a mut CMemberLst<'a>,
    pub right : &'a mut CMemberLst<'a>,
	pub left_class_base : &'a mut CMemberLst<'a>,
    pub right_class_base : &'a mut CMemberLst<'a>,
    pub str : &'a mut u8,
    pub member_class : &'a mut CHashClass<'a>,
    pub member_class_base : &'a mut CHashClass<'a>,
    pub meta : &'a mut CMemberLstMeta<'a>,
    pub use_cnt : u32,
    pub flags : u16,
    pub reg : i8,
    pub pad : i8,
    pub offset : i64,
    pub size : i64,
    pub dim : CArrayDim<'a>,
    pub static_data : &'a mut u8,
    pub static_data_rip : i64,
    pub dft_val : i64,
    pub fun_ptr : &'a mut CHashFun<'a>,
}

pub struct CHashClass<'a> {
    pub size : i64,
    pub neg_offset : i64,
    pub member_cnt : u32,
    pub ptr_stars_cnt : u8,
    pub raw_type : u8,
    pub flags : u16,
    pub member_lst_and_root : &'a mut CMemberLst<'a>, //Head of linked list and head of tree.
	pub member_class_base_root : &'a mut CMemberLst<'a>, //For finding dup class local vars.
	pub last_in_member_lst : &'a mut CMemberLst<'a>,
    pub base_class : &'a mut CHashClass<'a>,
	pub fwd_class : &'a mut CHashClass<'a>,
}

pub struct CExternUsage<'a> {
    pub next : &'a mut CExternUsage<'a>,
    pub rip : i64,
}

pub struct CHashFun<'a> {
    pub child : CHashClass<'a>,
    pub return_class : &'a mut CHashClass<'a>,
    pub arg_cnt : u32,
    pub pad : u32,
	pub used_reg_mask : u32,
    pub clobbered_reg_mask : u32,
    pub exe_addr : &'a mut u8,
    pub ext_lst : CExternUsage<'a>,
}

pub struct CLexHashTableContext<'a> {
    pub next : &'a mut CLexHashTableContext<'a>,
    pub old_flags : i64,
    pub hash_mask : i64,
    pub local_var_lst : CHashFun<'a>,
    pub fun : CHashFun<'a>,
    pub hash_table_lst : CHashTable<'a>,
	pub define_hash_table : CHashTable<'a>,
	pub local_hash_table : CHashTable<'a>,
	pub glbl_hash_table : CHashTable<'a>
}

pub struct CHashGeneric<'a> {
    pub parent : CHash<'a>,
    pub user_data0 : i64,
    pub user_data1 : i64,
    pub user_data2 : i64,
}

pub struct CCodeMisc<'a> {
    pub next : &'a mut CCodeMisc<'a>,
    pub last : &'a mut CCodeMisc<'a>,
    pub fwd : &'a mut CCodeMisc<'a>,
    pub dft : &'a mut CCodeMisc<'a>,
    pub begin : &'a mut CCodeMisc<'a>,
    pub str : &'a mut u8,
    pub _type : u32,
    pub flags : u32,
    pub use_cnt : i64,
    pub addr : &'a mut u8,
    pub st_len : i64,		//STR_CONST
    pub num_consts : i64,	//FLOAT_CONSTS
    pub range : i64,
    pub rip : i64,		//ASM_LABEL
    pub jmp_table : &'a mut &'a mut CCodeMisc<'a>,
    pub float_consts : &'a mut f64,
    pub dim : &'a mut CArrayDim<'a>,
    pub h : &'a mut CHash<'a>,
}

pub struct CStreamBlk<'a> {
    pub next : &'a mut CStreamBlk<'a>,
    pub last : &'a mut CStreamBlk<'a>,
    pub body : &'a mut u8,
}

pub struct CAOTImportExport<'a> {
    pub next : &'a mut CAOTImportExport<'a>,
    pub last : &'a mut CAOTImportExport<'a>,
    pub rip : i64,
    pub flags : i64,
    pub aot : &'a mut CAOT<'a>,
    pub str : &'a mut u8,
    pub src_link : &'a mut u8,
    pub _type : &'a mut u8,
    pub pad : &'a mut Vec<u8>,
}

pub struct CAOT<'a> {
    pub next : &'a mut CAOT<'a>,
    pub last : &'a mut CAOT<'a>,
    pub buf : &'a mut u8,
    pub rip : i64,
    pub rip2 : i64,
    pub aot_U8s : i64,
    pub max_align_bits : i64,
    pub org : i64,
    pub parent_aot : &'a mut CAOT<'a>,
    pub next_ie : &'a mut CAOTImportExport<'a>,
    pub last_ie : &'a mut CAOTImportExport<'a>,
    pub abss : &'a mut CAOTAbsAddr<'a>,
    pub heap_glbls : &'a mut CAOTHeapGlbl<'a>,
}

pub struct CICType {
    pub val : u16,
    pub raw_type : u8,
    pub mode : u8,
}

pub struct CICArg {
    pub _type : CICType,
    pub reg : u16, //low is reg, high is index_reg+scale<<6
    pub disp : i64,
}

pub struct CICTreeLinks<'a> {
    pub arg1_class : &'a mut CHashClass<'a>,
    pub arg2_class : &'a mut CHashClass<'a>,
    pub arg1_tree : &'a mut CIntermediateCode<'a>,
    pub arg2_tree : &'a mut CIntermediateCode<'a>,
    pub class2 : &'a mut CHashClass<'a>,
}

pub struct CIntermediateCode<'a> {
    pub ic_flags : i64,
	pub ic_data : i64,
	pub ic_line : i64,
    pub ic_class : &'a mut CHashClass<'a>,
    pub ic_class2 : &'a mut CHashClass<'a>,
    pub arg1 : CICArg,
    pub arg2 : CICArg,
    pub res : CICArg,
    pub arg1_type_pointed_to : u8,
    pub ic_body : Vec<u8>,
    pub t : CICTreeLinks<'a>,
}

pub struct CIntermediateCodeBase<'a> {
    pub next : &'a mut CIntermediateCodeBase<'a>,
    pub last : &'a mut CIntermediateCodeBase<'a>,
    pub ic_code : u16,
    pub ic_precedence : u16,
    pub ic_cnt : i16,
    pub ic_last_start : i16,
}

pub struct CCodeCtrl<'a> {
    pub coc_next : &'a mut CCodeCtrl<'a>,
    pub coc_next_misc : &'a mut CCodeMisc<'a>,
    pub coc_last_misc : &'a mut CCodeMisc<'a>,
    pub coc_head : CIntermediateCodeBase<'a>,
}

pub struct CPrsStk {
    pub ptr : i64,
    pub stk : Vec<i64>,
    pub ptr2 : i64,
    pub stk2 : Vec<i64>,
}

pub struct CCmpCtrl<'a> {
    pub next : &'a mut CCmpCtrl<'a>,
    pub last : &'a mut CCmpCtrl<'a>,
    pub token : i64,
	pub flags : i64,
	pub cur_i64 : i64,
    pub cur_f64 : f64,
    pub cur_str : &'a mut u8,
    pub cur_str_len : i64,
	pub class_dol_offset : i64,
    pub dollar_buf : &'a mut u8,
    pub dollar_cnt : i64,
    pub cur_help_idx : &'a mut u8,
    pub last_U16 : i64,
	pub min_line : i64,
    pub max_line : i64,
    pub last_line_num : i64,
	pub lock_cnt : i64,
    pub char_bmp_alpha_numeric : &'a mut u32,
    pub htc : CLexHashTableContext<'a>,
    pub hash_entry : Option<&'a mut CHashGeneric<'a>>,
    pub abs_cnts : CAbsCntsI64,
    pub asm_undef_hash : Option<&'a mut CAsmUndefHash<'a>>,
    pub local_var_entry : &'a mut CMemberLst<'a>,
    pub lb_leave : &'a mut CCodeMisc<'a>,
    pub cur_buf_ptr : &'a mut u8,
    pub lex_include_stk : &'a mut CLexFile<'a>,
	pub lex_prs_stk : &'a mut CLexFile<'a>,
	pub fun_lex_file : &'a mut CLexFile<'a>,
    pub next_stream_blk : &'a mut CStreamBlk<'a>,
    pub last_stream_blk : &'a mut CStreamBlk<'a>,
    pub aot : &'a mut CAOT<'a>,
    pub pass : i64,
    pub opts : i64,
    pub pass_trace : i64,
    pub saved_pass_trace : i64,
	pub error_cnt : i64,
    pub warning_cnt : i64,
    pub cur_ic_float_op_num : i64,
    pub last_ic_float_op_num : i64,
    pub last_float_op_ic : &'a mut CIntermediateCode<'a>,
    pub last_dont_pushable : bool,
    pub last_dont_popable : bool,
    pub last_float_op_pos : bool,
	pub dont_push_float : bool,
    pub pad : Vec<bool>,
    pub coc : CCodeCtrl<'a>,
    pub ps : &'a mut CPrsStk,
    pub aotc : &'a mut CAOTCtrl<'a>,
    pub aot_depth : i64,
    pub pmt_line : i64,
}

pub struct CWinScroll {
    pub min : i64,
    pub pos : i64,
    pub max : i64,
    pub flags : u32,
    pub color : u8,
    pub pad : Vec<u8>,
}

pub struct CTask<'a> {
    pub addr : &'a mut CTask<'a>,
    pub task_signature : u32,
    pub win_inhibit : u32,
    pub wake_jiffy : i64,
    pub task_flags : u32,
    pub display_flags : u32,
    pub code_heap : &'a mut CHeapCtrl<'a>,
    pub data_heap : &'a mut CHeapCtrl<'a>,
    pub put_doc : &'a mut CDoc<'a>,
    pub display_doc : &'a mut CDoc<'a>,
    pub border_doc : &'a mut CDoc<'a>,
    pub win_left : i64,
    pub win_right : i64,
    pub win_top : i64,
    pub win_bottom : i64,
    pub cur_dv : &'a mut CDrv<'a>,
    pub cur_dir : &'a mut u8,
    pub parent_task : &'a mut CTask<'a>,
    pub next_task : &'a mut CTask<'a>,
    pub last_task : &'a mut CTask<'a>,
	pub next_input_filter_task : &'a mut CTask<'a>,
    pub last_input_filter_task : &'a mut CTask<'a>,
	pub next_sibling_task : &'a mut CTask<'a>,
    pub last_sibling_task : &'a mut CTask<'a>,
	pub next_child_task : &'a mut CTask<'a>,
    pub last_child_task : &'a mut CTask<'a>,
    pub win_width : i64,
    pub win_height : i64,
	pub pix_left : i64,
    pub pix_right : i64,
    pub pix_width : i64, //These are in pixs, not characters
	pub pix_top : i64,
    pub pix_bottom : i64,
    pub pix_height : i64,
	pub scroll_x : i64,
    pub scroll_y : i64,
    pub scroll_z : i64,

    //These must be in this order
    //for $LK,"TASK_CONTEXT_SAVE",A="FF:::/Kernel/Sched.HC,TASK_CONTEXT_SAVE"$ and $LK,"_TASK_CONTEXT_RESTORE",A="FF:::/Kernel/Sched.HC,_TASK_CONTEXT_RESTORE"$
    pub rip : i64,
    pub rflags : i64,
    pub rsp : i64,
    pub rsi : i64,
    pub rax : i64,
    pub rcx : i64,
    pub rdx : i64,
    pub rbx : i64,
    pub rbp : i64,
    pub rdi : i64,
	pub r8 : i64,
    pub r9 : i64,
    pub r10 : i64,
    pub r11 : i64,
    pub r12 : i64,
    pub r13 : i64,
    pub r14 : i64,
    pub r15 : i64,
    pub gs : &'a mut CCPU<'a>,
    pub fpu_mmx : &'a mut CFPU,
    pub swap_cnter : i64,
    pub draw_it : U0_with_CTask<'a>,
    pub task_title : Vec<u8>,
    pub task_name : Vec<u8>,
    pub wallpaper_data : Vec<u8>,
    pub title_src : Vec<u8>,
    pub border_src : Vec<u8>,
    pub border_attr : Vec<u8>,
    pub win_z_num : u16,
    pub u16_pad : u16,
    pub stk : &'a mut CTaskStk<'a>,
    pub next_except : &'a mut CExcept<'a>,
    pub last_except : &'a mut CExcept<'a>,
    pub except_rbp : i64,
    pub except_ch : i64,
    pub except_callers : &'a mut Vec<u8>,
    pub catch_except : bool,
    pub new_answer : bool,
    pub answer_type : u8,
    pub u8_pad : Vec<u8>,
    pub answer : i64,
    pub answer_time : f64,
    pub bpt_list : &'a mut CBpt<'a>,
    pub next_ctrl : &'a mut CCtrl<'a>,
    pub last_ctrl : &'a mut CCtrl<'a>,
    pub cur_menu : &'a mut CMenu<'a>,
    pub next_settings : &'a mut CTaskSettings<'a>,
    pub next_ode : &'a mut CMathODE<'a>,
    pub last_ode : &'a mut CMathODE<'a>,
    pub last_ode_time : f64,
    pub hash_table : &'a mut CHashTable<'a>,
    pub srv_ctrl : CJobCtrl<'a>,
    pub next_cc : &'a mut CCmpCtrl<'a>,
    pub last_cc : &'a mut CCmpCtrl<'a>,
    pub last_fun : &'a mut CHashFun<'a>,
    pub task_end_cb : &'a mut u16,
    pub song_task : &'a mut CTask<'a>,
    pub animate_task : &'a mut CTask<'a>,
    pub rand_seed : i64,
    pub task_num : i64,
    pub fault_num : i64,
    pub fault_err_code : i64,
    pub popup_task : &'a mut CTask<'a>,
    pub dbg_task : &'a mut CTask<'a>,
    pub horz_scroll : CWinScroll,
    pub vert_scroll : CWinScroll,
    pub start_time : CDate,
    pub user_data : i64,
}

pub struct U8_with_doc<'a> {
    pub doc : &'a mut CDoc<'a>,
    pub doc_e : &'a mut CDocEntry<'a>,
    pub mem_task : &'a mut CTask<'a>,
}

pub struct CDocBin<'a> {
    pub next : &'a mut CDocBin<'a>,
    pub last : &'a mut CDocBin<'a>,
    pub tmp_use_cnt : i32,
    pub renum_num : i32,
    pub tags : &'a mut u8,
    pub start : bool,
    pub num : u32,
    pub flags : u32,
    pub size : u32,
    pub use_cnt : u32,
    pub end : bool,
    pub data : &'a mut u8,
}

pub struct CDocEntry<'a> {
    pub child : CDocEntryBase<'a>,
    pub attr : i64,
    pub cursor_x_offset : i64,
    pub left_cb : I64_with_doc<'a>,
    pub left_exp : i64,
    pub left_macro : &'a mut u8,
    pub cursor_y_offset : i64,
    pub right_cb : I64_with_doc<'a>,
    pub right_exp : i64,
    pub right_macro : &'a mut u8,
    pub tag_cb : &'a mut U8_with_doc<'a>,
    pub define_str : &'a mut u8,
    pub aux_str : &'a mut u8,
	pub bin_ptr_link : &'a mut u8,
	pub html_link : &'a mut u8,
	pub my_fmt_data : &'a mut u8,
    pub hex_ed_width : i64,
    pub scroll_len : i32,
    pub len : i32, //$LK,"DOCE_LEN_DFT",A="MN:DOCE_LEN_DFT"$
    pub bin_num : i32,
    pub raw_type : u8,
    pub pad : Vec<u8>,
    pub bin_data : &'a mut CDocBin<'a>,
    pub data : &'a mut u8,
}

pub struct CEdFindText {
    // Lets hope to god this isn't needed...
}

pub struct CEdFileName {
    // Lets hope to god this isn't needed...
}

pub struct CDocUndo<'a> {
    pub next : &'a mut CDocUndo<'a>,
    pub last : &'a mut CDocUndo<'a>,
    pub size : i64,
    pub doc_flags : i64,
    pub time_stamp : i64,
    pub body : &'a mut u8,
}

pub struct CDoc<'a> { //Linked Text File header
    pub head : CDocEntryBase<'a>,
    pub flags : i64,
    pub locked_flags : i64,
    pub cur_entry : &'a mut CDocEntry<'a>,
    pub old_cur_entry : &'a mut CDocEntry<'a>,
    pub cur_col : i32,
    pub old_cur_col : i32,
    pub line_start_col : i32,
    pub top_line_num : i32,
    pub dollar_buf_size : i32,
    pub dollar_buf_ptr : i32,
    pub dollar_buf : &'a mut u8, //When entering $$ cmds, it buffers them until the end $$.
    pub win_task : &'a mut CTask<'a>,
    pub mem_task : &'a mut CTask<'a>,
    pub owning_task : &'a mut CTask<'a>,
    pub page_line_num : i32,
    pub undo_cnt : i32,
    pub x : i32,
    pub y : i32,
    pub min_x : i32,
    pub max_x : i32,
    pub min_y : i32,
    pub max_y : i32,
    pub line : i64,
    pub col : i64,
    pub best_d : i64,
    pub old_win_top : i64,
    pub old_win_bottom : i64,
    pub old_win_left : i64,
    pub old_win_right : i64,
    pub cmd_U8 : i64,
    pub doc_signature : u32,
    pub cur_bin_num : u32,
    pub max_entries : i64,
    pub updates_cnt : i64,
    pub find_replace : &'a mut CEdFindText,
    pub filename : CEdFileName,
    pub file_attr : i64,
    pub left_click_link : I64_with_doc<'a>,
    pub right_click_link : I64_with_doc<'a>,
  
    //See $LK,"::/Apps/Psalmody/JukeBox.HC"$
    pub user_put_data : &'a mut u8, //Passed to user_put_key() and user_put_s()
    pub user_put_key : &'a mut Put_Key<'a>,
    pub user_put_s : &'a mut Put_S<'a>,
  
    pub parent_doc : Option<&'a mut CDoc<'a>>, //(When browsing deeper, opening deeper docs.)
    pub desc : u64, //8 characters. See $LK,"DocBorderLstDraw",A="MN:DocBorderLstDraw"$().
  
    pub bin_head : CDocBin<'a>,
    pub settings_head : CDocSettings,
    pub undo_head : CDocUndo<'a>,
  
    pub user_data : i64,
}

pub struct CLexFile<'a> {
    pub next : Option<&'a mut CLexFile<'a>>,
    pub buf : &'a mut u8,
    pub buf_ptr : &'a mut u8,
    pub line_num : i64,
    pub flags : i64,
    pub full_name : &'a mut u8,
    pub line_start : &'a mut u8,
    pub doc : &'a mut CDoc<'a>,
    pub cur_entry : &'a mut CDocEntry<'a>,
    pub depth : i32,
    pub last_u16 : u8,
    pub pad : Vec<u8>
}

pub struct CAOTBinBlk<'a> {
    pub next : &'a mut CAOTBinBlk<'a>,
    pub body : Vec<u8>,
}

pub struct CAsmUnresolvedRef<'a> {
    pub next : &'a mut CAsmUnresolvedRef<'a>,
    pub _type : i64,
    pub line_num : i64,
    pub machine_code : &'a mut u8,
    pub rip : i64,
    pub rel_rip : i64,
    pub aot : &'a mut CAOT<'a>,
    pub str : &'a mut u8,
    pub asm_undef_hash : &'a mut CAsmUndefHash<'a>,
    pub U8_avail : bool,
    pub imm_flag : bool,
}

pub struct CAOTCtrl<'a> {
    pub rip : i64, //Inst ptr
    pub arg1 : CAsmArg<'a>,
    pub arg2 : CAsmArg<'a>,
    pub bin : &'a mut CAOTBinBlk<'a>,
    pub num_bin_U8s : i64,
    pub max_align_bits : i64,
    pub org : i64,
    pub local_unresolved : &'a mut CAsmUnresolvedRef<'a>,
    pub glbl_unresolved : &'a mut CAsmUnresolvedRef<'a>,
    pub abss : &'a mut CAOTAbsAddr<'a>,
    pub heap_glbls : &'a mut CAOTHeapGlbl<'a>,
    pub lst_col : i64,
    pub lst_last_rip : i64,
    pub last_label : &'a mut u8,
    pub lst_last_line : &'a mut u8,
    pub lst_last_lfn : &'a mut CLexFile<'a>,
    pub seg_size : i64,
    pub lst : bool,
}