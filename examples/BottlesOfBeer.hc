U0 Main () {
	I64 i = 99;
 
	for (; i >= 0; i--) {
		if (i == 1) {
			// Just a string on it's own will pass it to an inbuilt HolyC function that puts it to terminal
			"1 Bottle of Beer on the wall, 1 bottle of beer.\n";
			"Take one down and pass it around, no more bottles of beer on the wall.\n";
		} else if (i == 0) {
			"No more bottles of beer on the wall, no more bottles of beer.\n";
			"Go to the store and buy some more, 99 bottles of beer on the wall.\n";
		} else {
			"%d bottles of beer on the wall, %d bottles of beer.\n",i,i;
			"Take one down and pass it around, %d bottle",(i-1);
			// Only add the s if it's not 1
			if ((i-1) != 1) {
				"s";
			}
 
			" of beer on the wall.\n";
		}
	}
}