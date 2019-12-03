fn main() {
    let input: [i32; 129] = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,10,19,23,1,6,23,27,1,5,27,31,1,10,31,35,2,10,35,39,1,39,5,43,2,43,6,47,2,9,47,51,1,51,5,55,1,5,55,59,2,10,59,63,1,5,63,67,1,67,10,71,2,6,71,75,2,6,75,79,1,5,79,83,2,6,83,87,2,13,87,91,1,91,6,95,2,13,95,99,1,99,5,103,2,103,10,107,1,9,107,111,1,111,6,115,1,115,2,119,1,119,10,0,99,2,14,0,0];
    for noun in 0..1000 {
        for verb in 0..1000 {
            let mut operated_input = input;
            operated_input[1] = noun;
            operated_input[2] = verb;
            if execute(&mut operated_input)[0] == 19690720{
                println!("Bingo! noun {}, verb {} => Result {}", noun, verb, 100*noun+verb);
            }
        }
    }
   
    // let mut operated_input = input;
    // operated_input[1] = 12;
    // operated_input[2] = 2;
    // println!("Result: {}", execute(&mut operated_input)[0])
    
}

fn execute(arr: &mut [i32]) -> &mut [i32]{
    for i in (0..arr.len()).step_by(4) {
        if i+3 > arr.len()-1 {
           break;
        } else if arr[i] == 99{
            return arr;
        }else if arr[i] == 1 {
            if arr[i+3] as usize > arr.len()-1 || arr[i+2] as usize > arr.len()-1 || arr[i+1] as usize > arr.len()-1 {
                break;
            } 
            arr[arr[i+3] as usize]=arr[arr[i+1] as usize] + arr[arr[i+2] as usize];
        } else if arr[i] == 2{
            if arr[i+3] as usize > arr.len()-1 || arr[i+2] as usize > arr.len()-1 || arr[i+1] as usize > arr.len()-1 {
                break;
            } 
            arr[arr[i+3] as usize]=arr[arr[i+1]as usize] * arr[arr[i+2] as usize];
        } else{
            arr[0] = 0;
            return arr;
        }
    }
    arr[0] = 0;
    return arr;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let mut input: [i32; 5] = [1,0,0,0,99];
        let output: [i32; 5] = [2,0,0,0,99];
        assert_eq!(execute(&mut input), &output);
    }

    #[test]
    fn test_execute_2() {
        let mut input: [i32; 12] = [1,9,10,3,2,3,11,0,99,30,40,50];
        let output: [i32; 12] = [3500,9,10,70,2,3,11,0,99,30,40,50];
        assert_eq!(execute(&mut input), &output);
    }

    #[test]
    fn test_execute_3() {
        let mut input: [i32; 5] = [2,3,0,3,99];
        let output: [i32; 5] = [2,3,0,6,99];
        assert_eq!(execute(&mut input), &output);
    }

    #[test]
    fn test_execute_4() {
        let mut input: [i32; 6] = [2,4,4,5,99,0];
        let output: [i32; 6] = [2,4,4,5,99,9801];
        assert_eq!(execute(&mut input), &output);
    }

    #[test]
    fn test_execute_5() {
        let mut input: [i32; 9] = [1,1,1,4,99,5,6,0,99];
        let output: [i32; 9] = [30,1,1,4,2,5,6,0,99];
        assert_eq!(execute(&mut input), &output);
    }

    
}
