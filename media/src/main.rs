#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Song { title: String },
    Podcast(u32),
    PlaceHolder,
}

impl Media {
    fn description(&self) -> String {
        //     if let Media::Book { title, author } = self {
        //         format!("Book: {} by {}", title, author)
        //     } else if let Media::Movie { title, director } = self {
        //         format!("Media: {} by {}", title, director)
        //     } else if let Media::Song { title } = self {
        //         format!("Song: {}", title)
        //     } else {
        //         format!("No media description!")
        //     }

        match self {
            Media::Book { title, author } => format!("Book: {} by {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} by {}", title, director),
            Media::Song { title } => format!("Song: {}", title),
            Media::Podcast(episode_num) => format!("Podcast: {}", episode_num),
            Media::PlaceHolder => String::from("PlaceHolder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Catalog {
        Catalog { items: vec![] }
    }

    fn add(&mut self, item: Media) {
        self.items.push(item);
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
    let podcast = Media::Podcast(12);
    let placeholder = Media::PlaceHolder;

    println!("{}", book.description());
    println!("{}", movie.description());
    println!("{}", song.description());
    println!("{}", podcast.description());
    println!("{}", placeholder.description());

    let mut catalog = Catalog::new();
    catalog.add(book);
    catalog.add(movie);
    catalog.add(song);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("Catalog: {:#?}", catalog);
}
