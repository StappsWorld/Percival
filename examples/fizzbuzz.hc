class TimeEntry
{
  TimeEntry *next,*last;
  U8 type;
  TimeEntry datetime;
  U8 *desc;
};

class CTask;

/* For each number that divides evenly with 3, print "Fizz", 5, print "Buzz", 15, print "FizzBuzz", otherwise print the number. */
CTask *Main () {
  U8 *j,k;
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
  
  TimeEntry *tmpt=header->next,*tmpt1;
}