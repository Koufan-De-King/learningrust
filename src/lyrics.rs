pub fn lyrics(mut i: u32){
    if i == 1 {
        println!("\nOn the {i}st day of Christmas,");
        println!("my true love gave to me");
        println!("A partridge in a pear tree!");
        i=i-1;
    }
    else if i == 2 {
        println!("\nOn the {i}nd day of Christmas,");
        println!("my true love gave to me");
    }
    else if i == 3 {
        println!("\nOn the {i}rd day of Christmas,");
        println!("my true love gave to me");
    }
    else {
        println!("\nOn the {i}th day of Christmas,");
        println!("my true love gave to me");
    }
   
    while i!=0{
        match i{
        12 => { println!("Twelve drummers drumming,");
                i=i-1},
        11 => { println!("Eleven pipers piping,");
                i=i-1},
        10 => { println!("Ten lords a-leaping,");
                i=i-1},      
        9 =>  { println!("Nine ladies dancing,");
                i=i-1},
        8 =>  { println!("Eight maids a-milking,");
                i=i-1},
        7 =>  { println!("Seven swans a-swimming");
                i=i-1},
        6 =>  { println!("Six geese a-laying,");
                i=i-1},    
        5 =>  { println!("Five golden rings,");
                i=i-1},
        4 =>  { println!("Four calling birds,");    
                i=i-1},     
        3 =>  { println!("Three French hens,");
                i=i-1},
        2 =>  { println!("Two turtle doves,");
                i=i-1},
        1 =>  { println!("And a partridge in a pear tree!");
                i=i-1},
        _ => continue,
        }
    }
}    
