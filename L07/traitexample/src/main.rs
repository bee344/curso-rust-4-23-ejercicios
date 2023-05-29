use aggregator::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 author info tweet: {}", tweet.summarize_author());

    let breaking_news = NewsArticle {
        headline: String::from("Noticia de Ejemplo"),
        location: String::from("Polkadot World"),
        author : String::from("El Informante"),
        content : String::from("Contenido de Ejemplo"),
    };

    aggregator::notify(&breaking_news);

    let breaking_news2  = aggregator::returns_summarizable();
    let news_summary = breaking_news2.summarize();
    println!("NEWS Summary : {}",news_summary)
}