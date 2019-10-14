pub mod calculator_functions{
   pub mod basic_function{
        pub fn add(x:i32,y:i32)->i32{
            x+y
        }
        pub fn sub(x:i32,y:i32) ->i32{
            x-y
        }
        pub fn divide(x:i32 , y:i32)->i32{
            x/y
        }
        pub fn multiply(x:i32 , y:i32)->i32{
            x*y
        }   
    }
   pub mod power_functions{
        pub fn square(x:i32) -> i32{
            x*x
        }
        pub fn cube(x:i32) -> i32{
           x*x*x
        }
        pub fn power(x:i32,y:i32) -> i32{
            let mut ans = x;
           for i in 1..y{
             ans = ans*x;

           }
           ans
        }
    }
}