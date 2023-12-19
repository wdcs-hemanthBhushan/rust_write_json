use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}
fn conver_to_json(structt: Article) -> String {
    let json = serde_json::to_string(&structt).unwrap();
    json
}
fn main() {
    let article: Article = Article {
        article: String::from("How to convert from struct to json"),
        author: String::from("Hemant"),
        paragraph:vec![
            Paragraph {
                name: String::from("chekkkk"),
            },
            Paragraph {
                name: String::from("chekkkk11111"),
            },
            Paragraph {
                name: String::from("chekkkk222222222"),
            },
        ],
    };

    let json:String = conver_to_json(article);
    println!("the json is {}", json);
}
