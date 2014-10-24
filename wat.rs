use std::io::File;

fn main()
{
    match File::create(&Path::new("message.txt")) {
        Some(mut file) => {
            file.write(bytes!("line one\n"));
            file.write_str("line two\n");
        }
        None =>{
            println!("Opening message.txt failed!");
        }
    }
}