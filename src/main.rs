use std::fmt;
#[derive(PartialEq)] // to use == in rust

enum Genre{
    Action,
    Comedy,
    Drama,
    Horror,
    Romance,
}
struct Movie {
    title: String,
    genre: Genre,
    is_watch: bool 
}
struct MovieCollection{
    movies : Vec<Movie>
}
impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let genre_str = match self {
            Genre::Action => "Action",
            Genre::Comedy => "Comedy",
            Genre::Drama => "Drama",
            Genre::Horror => "Horror",
            Genre::Romance => "Romance",
        };
        write!(f, "{}", genre_str)
    }
}


impl  MovieCollection  {
    fn add_movie(&mut self, title: String, genre: Genre, watch: bool){
        let movie = Movie {
            title,
            genre,
            is_watch: watch,
        };
        self.movies.push(movie);
        println!("Successfully added Movie");
    }

    fn list_movie(&self){
        println!("---Your list of movies---");
        for movie in &self.movies{
            println!("- {} ({}) {}",movie.title,movie.genre, if movie.is_watch{"-Watch"} else{"-Not watch"} )
        }
    }
    fn filter_movie(&self,genre: Genre){
        println!("--- This is your list of movies by genre: {} ---",genre );
        for movie in &self.movies{
            if movie.genre == genre{
                println!("Title: {} Genre: {}", movie.title ,movie.genre)
            }
        }
    }
}

fn main() {
    let mut movies = MovieCollection{ movies: Vec::new()};
    
    movies.add_movie("Harry Potter".to_string(), Genre::Action, true);
    movies.add_movie("Alien".to_string(), Genre::Horror, false);
    movies.add_movie("Alien vs predator 2023".to_string(), Genre::Horror, true);
    movies.add_movie("Cinderella".to_string(), Genre::Romance, false);
    movies.add_movie("Chaplin".to_string(), Genre::Comedy, true);
    movies.add_movie("Game OF Thrones".to_string(), Genre::Drama, true);
    
    
    movies.list_movie();

    
    movies.filter_movie(Genre::Horror);
}
