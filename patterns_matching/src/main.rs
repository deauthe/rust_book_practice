use std::io::{self, Write};

fn main() {
    print!("what's your favourite color, leave empty if None \n");
    let is_tuesday = false;
    let selected_age: Result<u8,_> = "no age".parse();
   

    io::stdout().flush().unwrap(); //flushing the output before input runs 
    //to make sure that the prompt to enter the text happens
    //before the programme actually exprects an input 

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    input.pop(); //removing trailing newline in the input if present 


    //syntax below to be noted 
   let chosen_color: Option<String> =  if input.is_empty(){
    None
   }else{
    Some(input)
   };

   if let Some(color) = chosen_color{
        print!("you chose the color: {}", color);
   }else if is_tuesday{
        print!("tuesday is green baba");

   }else if let Ok(age) = selected_age{
    if age > 30{
        print!("using gray as the background, you're old")
    }else{
        print!("using orange bloomer");

    }
   }else{
    print!("aah such ageless humour ");
   }




}
