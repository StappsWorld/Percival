U0 Main () {
	I64 i,j,k;
	i = (j = (k = 99));
 
	for (; i >= 0; i--) {
		if (i == 1) {
			"1 Bottle of Beer on the wall, 1 bottle of beer.\n";
			"Take one down and pass it around, no more bottles of beer on the wall.\n";
		} else if (i == 0) {
			"No more bottles of beer on the wall, no more bottles of beer.\n";
			"Go to the store and buy some more, 99 bottles of beer on the wall.\n";
		} else {
			"%d bottles of beer on the wall, %d bottles of beer.\n",i,i;
			"Take one down and pass it around, %d bottle",(i-1);
			if ((i-1) != 1) 
				"s";
			
 
			" of beer on the wall.\n";
		}
	}
}