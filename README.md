# jikan-rs

Jikan-rs is an asynchronous client for [Jikan](https://jikan.moe) API in Rust. Perhaps most importantly, jikan-rs does 
not make any attempts to rate limit itself, so use it as responsibly and remember that Jikan API has limitations, check 
out [this section](https://jikan.docs.apiary.io/#introduction/information/rate-limiting) of documentation in order to see 
to what extent the API is limited or throttled.

## Installation
Jikan-rs uses reqwest that uses [tokio](https://github.com/tokio-rs/tokio) for asynchronous tasks, so you will need the 
tokio runtime as well. 

Add the following lines to your cargo.toml:
```toml
[dependencies]
tokio = { version = "0.2", features = ["full"] }
jikan-rs = { git = "https://github.com/bruno319/jikan-rs" }
```

## Basic usage
```rust,no_run
use jikan_rs::client::Jikan;

#[tokio::main]
async fn main() {
    let jikan = Jikan::new();
    let cowboy_bebop = jikan.find_anime(1)
        .await
        .unwrap();
    println!("{:#?}", cowboy_bebop);
}
```

## Documentation
_todo_

## List of features
- Anime
    - Basic information
    - Characters & Staff
    - Episode
    - News
    - Videos/PV/Episodes
    - Pictures
    - Stats
    - Forum Topics
    - More Info
    - Reviews
    - Recommendations
    - User Updates
- Manga
    - Basic information
    - Characters 
    - News
    - Pictures
    - Stats
    - Forum Topics
    - More Info
    - Reviews
    - Recommendations
    - User Updates
- People
    - Basic information
    - Pictures
- Characters
    - Basic information
    - Pictures
- Search (Anime/Manga/Character/Person)
    - Basic query
    - Filters (Advanced Search)
    - Pagination Support
    - No.# of pages
- Seasonal Anime 
    - Season + Year
    - Undefined airing date
- Season Archive
- Anime Scheduling (for the current season)
    - Filtering by day of the week.
- Top
    - Anime
    - Manga
    - People
    - Characters
    - Sub Types & Pagination Support
- Genre
    - Anime genres
    - Manga genres
- Producer
- Magazine
- User
    - Profile
    - Friends
    - History
        - Filter by Anime/Manga.
    - Anime list
        - Filter by status (watching, completed, etc.)
        - Advanced filters
        - Pagination support
    - Manga list
        - Filter by status (reading, completed, etc.)
        - Advanced filters
        - Pagination support
- Clubs
    - Profile
    - Member list
        - Pagination support
- Meta
    - API status
    
## To-do
- Publish on crates.io
- Unit tests
- Improve integration tests coverage