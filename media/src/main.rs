enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Song { title: String },
}

impl Media {
    fn description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: {} by {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Media: {} by {}", title, director)
        } else if let Media::Song { title } = self {
            format!("Song: {}", title)
        } else {
            format!("No media description!")
        }
    }
}

fn main() {
    println!("Hello, world!");

    let book = Media::Book {
        title: String::from("good book"),
        author: String::from("me"),
    };
    let movie = Media::Movie {
        title: String::from("bad movie"),
        director: String::from("me"),
    };
    let song = Media::Song {
        title: String::from("my song"),
    };

    println!("{}", book.description());
    println!("{}", movie.description());
    println!("{}", song.description());
}
