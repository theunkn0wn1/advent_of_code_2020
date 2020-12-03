#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
use anyhow;


pub fn solve_p1(maze: Vec<String>) -> anyhow::Result<usize>{
    let mut x = 0;
    let mut collisions: usize = 0;

    for row in maze.iter(){
        if row.chars().nth(x % row.len()).unwrap() == '#' {
            collisions +=1;
        }
        x+=3;
    }

    Ok(collisions)
}