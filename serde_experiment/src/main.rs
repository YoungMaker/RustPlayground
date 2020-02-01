#[derive(Debug)]
struct Point {
  x: u32,
  y: u32,
}

fn generate_square(center: &Point, size_px: u32) -> [Point; 4]
{
  //TODO: generate square using upper left corner point
  let half_size = size_px / 2;
  let square: [Point; 4] = [
    Point{ x: ( center.x - half_size ), y: (center.y - half_size), },
    Point{ x: ( center.x + half_size ), y: (center.y - half_size), },
    Point{ x: ( center.x + half_size ), y: (center.y + half_size), },
    Point{ x: ( center.x - half_size ), y: (center.y + half_size), },
  ];
  return square;
}

//generates squares in a vector
fn generate_squares(center: Point, num_squares: u32,  size_px: u32) -> Vec<[Point; 4]> 
{
  let mut squares: Vec<[Point; 4]> = Vec::new();
  for _i in 0..num_squares 
  {
    squares.push( generate_square( &center, size_px) );
  }
  return squares;
}

fn main() {
  println!("Generating squares");
  for square in generate_squares( Point{x: 10, y: 10}, 10, 10 ) 
  {
    println!("Square:");
    
    for i in 0..3
    {
      println!("\t{}, {}", square[i].x, square[i].y);
    }

  }
}
