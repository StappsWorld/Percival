pub mod parser;

fn main() {
    let s = r#"
U8 *JukeReward(U8 *msg)
{
  U8 *buf;
  U8 *doc;
  U8 *res=Spawn(&SrvCmdLine,NULL,"Reward",,Fs);
  StrCpy(res->task_title,"Reward");
  res->title_src=TTS_LOCKED_CONST;

  doc=DocNew(,res);
  DocPrint(doc,"$$WW+H,1$$$$RED$$%s",msg);

  buf=MStrPrint("DocEd(0x%X);",doc);
  TaskExe(res,NULL,buf,1<<JOBf_EXIT_ON_COMPLETE|1<<JOBf_FREE_ON_COMPLETE);
  Free(buf);
  TaskWait(res);

  res->border_src =BDS_CONST;
  res->border_attr=LTGRAY<<4+DrvTextAttrGet(':')&15;
  res->text_attr  =LTGRAY<<4+BLUE;
  res->win_inhibit=WIG_NO_FOCUS_TASK_DFT;
  WinHorz(Fs->win_right+2,TEXT_COLS-2,res);
  WinVert(2,TEXT_ROWS-2,res);

  WinFocus(Fs->parent_task);
  return res;
}
"#;
    println!(
        "{:#?}",
        parser::Parser::new().parse(s)
    );
}
