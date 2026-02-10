#[derive(Debug)]
struct File {
    name: String,
}
#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: Vec::new(),
        }
    
    }
    fn create_file(&mut self, name: String) {
        let file = File { name};
        self.contents.push(file);               
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut folder = Folder::new(String::from("MyFolder"));

    folder.create_file(String::from("file1.txt"));
    folder.create_file(String::from("file2.txt"));
    println!("{:?}", folder);

    let deleted_file = folder.delete_file(0);
    println!("Deleted file: {:?}", deleted_file);
    println!("{:?}", folder);

    match folder.get_file(0) {
        Some(file) => println!("Retrieved file: {:?}", file),
        None => println!("There was no file"),
    }
}

/*
Let's model a file system on a computer.
 
Define a File struct with a `name` field set to a
String. Derive a Debug implementation.
 
Define a Folder struct with a `name` field set to
a String and a `contents` field set to a vector of
File structs. Derive a Debug implementation.
 
On the Folder struct...
 
Define a `new` constructor function that accepts a
`name` String. The method should create and return
a new Folder with that name. For the `contents` field,
provide a hardcoded empty vector.
 
Define a `create_file` method that accepts a `name`
String. The method should create a new File with that
name and add it to the end of the `contents` vector.
 
Define a `delete_file` method that accepts an `index`
parameter of type `usize`. The method should remove the
File at the specified index position from the `contents`
vector. It should also return the File.
 
Define a `get_file` method that accepts an `index`
parameter of type `usize`. The method should return
an Option containing a reference to the File at
that index position.
 
In the `main` function, use the `new` function to
create a Folder instance with a `name` of your choosing.
 
Call the `create_file` method two times. Print out
the Folder in Debug format.
 
Delete one of the two files using the `delete_file`
method. Print out the Folder in Debug format.
 
Call the `get_file` method. Use a match statement
to react to both Option variants. For the Some variant,
print out the File in Debug format. For the None variant,
print out the text "There was no file".
*/