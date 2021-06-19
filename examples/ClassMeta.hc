
U0 RankOut(I64 i)
{
  " %z",i,"Cadet\0Ensign\0Captain\0Admiral\0President\0";
}

class Test1Struct
{
  I64 age       print_str "%2d" dft_val 38;
  I64 color     dft_val RED; 
  I64 rank      print_str "%1d" dft_val 6/2 output_fun &RankOut;
};

class Test2Struct
{
  I64 age        print_str "%2d" dft_val 38 percentile 54.20;
  I64 rank       print_str "%1d" dft_val 5;
  I64 serial_num print_str "%6d" dft_val 123456;
};

U0 DumpStruct(U8 *_d,U8 *class_name=lastclass)
{
  CHashClass *tmpc=HashFind(class_name,Fs->hash_table,HTT_CLASS);
  U8 *print_str;
  I64 *q,dft_val;
  F64 percentile;
  if (!tmpc) return;
  CMemberLst *ml;
  ml=tmpc->member_lst_and_root;
  while (ml) {
    "%s:",ml->str;

    
    
    q=_d+ml->offset;

    if (print_str=MemberMetaData("print_str",ml))
      "", print_str,*q;

    if (dft_val=MemberMetaData("dft_val",ml))
      " default:%d",dft_val;

      
    if (MemberMetaFind("percentile",ml)) {

      
      percentile=MemberMetaData("percentile",ml)(F64);
      " percentile: %5.2f",percentile;
    }

    if (fp_output_fun=MemberMetaData("output_fun",ml))
      (*fp_output_fun)(*q);
    '\n';
    ml=ml->next;
  }
}

Test1Struct t1;
t1.age=44;
t1.rank=3;

DumpStruct(&t1);

Test2Struct t2;
t2.age=22;
t2.rank=2;
t2.serial_num=55555;

DumpStruct(&t2);