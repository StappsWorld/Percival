class CTask;

CTask *Main () {
  U8 *j;
  U8 *k;
  U8 i;
  for (i = 1; i <= 100; i++) {
    if (!(i % 15))
      Print("FizzBuzz");
    else if (!(i % 3))
      Print("Fizz");
    else if (!(i % 5))
      Print("Buzz");
    else
      Print("%d", i);
    Print("\n");
  }
  
  U0 *tmpt=header->next;
}