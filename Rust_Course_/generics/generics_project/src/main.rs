#[derive(Debug)]
enum DigitalContent {
    AudioFile(String),
    VideoFile(String),
}

#[derive(Debug)]
struct ChatMessage <T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message1 = ChatMessage {
        content: "Hello, how are you doing?",
        time: "10:00 AM".to_string(),
    };
    
    let message2 = ChatMessage {
        content: String::from("I am doing fine, Thank you"),
        time: "10:01 AM".to_string(),
    };
    
    let message3 = ChatMessage {
        content: DigitalContent::AudioFile("song.mp3".to_string()),
        time: "10:10 AM".to_string(),
    };

    message3.consume_entertainment();

    println!("Message 1 time: {}", message1.retrieve_time());
    println!("Message 2 time: {}", message2.retrieve_time());
    println!("Message 3 time: {}", message3.retrieve_time());

    
}
/*
The `time` field should always be a String.
Derive a Debug implementation.
 
Add an impl block for ChatMessage structs whose T type
is a DigitalContent enum. Define a `consume_entertainment`
method that prints out the value of the `content` field in
Debug format. For example, "Watching the AudioFile".
 
Add an impl block for ChatMessage structs with any type T.
Define a `retrieve_time` method that returns a String.
It should return a clone of the `time` field from
the struct.
 
In `main`, create a ChatMessage with `content` set to a
string slice.
 
Create another ChatMessage with `content` set to a String.
 
Create another ChatMessage with `content' set to a
DigitalContent variant.
 
Invoke the `consume_entertainment` method on the
ChatMessage storing a DigitalContent enum.
 
Invoke the `retrieve_time` method on all 3 ChatMessage
instances and print out each String's content.
*/


