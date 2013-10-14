/*
Braden Wright
btw3ta
CS4414 Midterm 1
Problem 8
*/

fn one() -> (bool) {
   if (two() == true) {
      return true;
   } else {
      return false;
   }
}

fn two () -> (bool) {
   if (one() == true) {
      return true;
   } else {
      return false;
   }
}



fn main() {
   one();
}

