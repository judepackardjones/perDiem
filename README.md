# perDiem

0.1.61 to 0.1.65 Patch notes:  
-------------

Message me on Discord @ dtimer for any feedback or suggestions  
------------

Adds:  
-----------

>OrdinalDate struct  
>increase and decrease ordinally functions  
>to_OrdinalDate  
>Finally adding decrease function for Date and DateTime(decrease_ordinally_as_new and decrease_ordinally)  
>Improves documentation a ton  

Fixes/Changes:  
-----------

> Changed the start year for .new() for Date, DateTime, and OrdinalDate to 0, (Was 1)  
> Changed some doc descriptions  
> Removed unnecessary checks due to type restrictions  
> Made 24 an invalid hour(Should be hour 0)  
> Fixed 29/2 being change to a non leap year returning 28/2 rather than 1/3

Normal README:  
-----------------

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

