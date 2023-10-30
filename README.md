# perDiem

The `Date` struct:  
----------------
Fields:  
> `day`: i8  
> `month`: i8  
> `year`: i16  
[Implementations](https://docs.rs/perDiem/0.1.4/perDiem/types/struct.Date.html#method.allShareEL)  

The `DateTime` struct:  
-------------------
Fields:  
>`second`: i8  
>`minute`: i8  
>`hour`: i8  
>`day`: i8  
>`month`: i8  
>`year`: i16  
[Implementations](https://docs.rs/perDiem/0.1.4/perDiem/types/struct.DateTime.html)  

The `TimeDifference` struct:  
----------------------------
Used to represent the difference between Dates and DateTimes
Fields:
>`second`: i32   
>`minute`: i32  
>`hour`: i32  
>`day`: i32  
>`month`: i32  
>`year`: i32  
[Implementations](https://docs.rs/perDiem/0.1.4/perDiem/types/struct.TimeDifference.html)  

The `TimeSpan` enum:  
--------------------
Used for increase and decrease methods(WIP)  
Variants:
>`second`: i32   
>`minute`: i32  
>`hour`: i32  
>`day`: i32  
>`month`: i32  
>`year`: i32  

Helper functions and methods:  
-----------------------------
as_Date -> Convert a forming a date to a Date - [docs](https://docs.rs/perDiem/0.1.4/perDiem/types/trait.x.html)  
with_separators -> Takes a date format such as ddmmyyy and inserts a separator in between fields - [docs](https://docs.rs/perDiem/0.1.4perDiem/types/trait.y.html)  