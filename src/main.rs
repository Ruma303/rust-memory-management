mod ownership;
mod coercion;
mod borrowing;

fn main() {
  
  //INFO Ownership and move
  ownership::ownership();
  
  //INFO Borrowing and references
  borrowing::borrowing();
  borrowing::deref();
  
  //INFO Coercion
  coercion::coercion();
}
