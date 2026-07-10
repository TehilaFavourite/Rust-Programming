#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}

#[derive(Debug)]

struct TVChannel {
    name: String,
    channel_type: ChannelType,
}

fn main() {
    let channels = [
        TVChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy,
        },
        TVChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TVChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TVChannel {
            name: String::from("RustTV"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];

    let good_channels: Vec<String> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .map(|channel| channel.name.clone())
        .collect();

    println!("{good_channels:?}");

    let all_are_rust = channels
        .iter()
        .all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    let any_are_rust = channels
        .iter()
        .any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("All are Rust: {all_are_rust}");
    println!("Any are Rust: {any_are_rust}");

    let good_channel = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);


    println!("{}", good_channel.is_some());
}
