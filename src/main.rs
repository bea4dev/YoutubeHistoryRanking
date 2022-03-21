use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use easy_scraper::Pattern;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Movie {
    title: String,
    url: String
}


fn main() {
    println!("Loading...");

    let html = fs::read_to_string("watch-history.html").expect("watch-history.html is not found.");

    let pattern = Pattern::new(r#"
    <div class="content-cell mdl-cell mdl-cell--6-col mdl-typography--body-1">
        <a href="{{url}}">{{title}}</a>
    </div>
    "#).unwrap();

    let elements = pattern.matches(html.as_str());


    let mut map: HashMap<Movie, i32> = HashMap::new();
    let mut text: Vec<String> = Vec::new();

    //Movie history
    for element in elements {
        if !element["url"].contains("https://www.youtube.com/watch?v=") {
            continue;
        }

        let movie = Movie {
            title: element["title"].clone(),
            url: element["url"].clone()
        };

        if map.contains_key(&movie) {
            let views = map.get(&movie).unwrap() + 1;
            map.insert(movie, views);
        } else {
            map.insert(movie, 1);
        }
    }

    //Sort
    let mut vec: Vec<(&Movie, &i32)> = map.iter().collect();
    vec.sort_by(|a, b| (-a.1).cmp(&(-b.1)));

    println!("< Movie history ranking >");
    println!(" ");
    text.push("< Movie history ranking >".to_string());
    text.push(" ".to_string());

    let mut ranking = 1;
    for rank in vec {
        let movie = rank.0;
        let views = rank.1;

        println!("  {:?}, views = {:?}, title = {:?}, url = {:?}", ranking, views, movie.title, movie.url);
        text.push("  ".to_string() + ranking.to_string().as_str() + ", views = " + views.to_string().as_str() + ", title = " + movie.title.as_str() + ", url = " + movie.url.as_str());

        if ranking >= 20 {
            break;
        }

        ranking += 1;
    }

    //Channel history
    let elements = pattern.matches(html.as_str());
    let mut map: HashMap<Movie, i32> = HashMap::new();
    for element in elements {
        if !element["url"].contains("https://www.youtube.com/channel/") {
            continue;
        }

        let movie = Movie {
            title: element["title"].clone(),
            url: element["url"].clone()
        };

        if map.contains_key(&movie) {
            let views = map.get(&movie).unwrap() + 1;
            map.insert(movie, views);
        } else {
            map.insert(movie, 1);
        }
    }

    //Sort
    let mut vec: Vec<(&Movie, &i32)> = map.iter().collect();
    vec.sort_by(|a, b| (-a.1).cmp(&(-b.1)));

    println!(" ");
    println!("< Channel history ranking >");
    println!(" ");
    text.push(" ".to_string());
    text.push("< Channel history ranking >".to_string());
    text.push(" ".to_string());

    let mut ranking = 1;
    for rank in vec {
        let movie = rank.0;
        let views = rank.1;

        println!("  {:?}, views = {:?}, channel = {:?}, url = {:?}", ranking, views, movie.title, movie.url);
        text.push("  ".to_string() + ranking.to_string().as_str() + ", views = " + views.to_string().as_str() + ", channel = " + movie.title.as_str() + ", url = " + movie.url.as_str());

        if ranking >= 20 {
            break;
        }

        ranking += 1;
    }


    let mut file = File::create("ranking.txt").unwrap();
    for t in text {
        file.write_all(t.as_bytes());
        file.write_all("\n".as_bytes());
    }
    file.flush();

}