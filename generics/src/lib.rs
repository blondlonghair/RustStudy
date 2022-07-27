// pub trait Summarizable {
//     fn summery(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summarizable for NewsArticle {
//     fn summery(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summarizable for Tweet {
//     fn summery(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// extern crate aggregator;

// use aggregator::Summarizable;

// struct WeatherForecast {
//     high_temp: f64,
//     low_temp: f64,
//     chance_of_precipitation: 64,
// }

// impl Summarizable for WeatherForecast {
//     fn summary(&self) -> String {
//         format!("The high will be {}, and the low will be {}. The chance of
//         precipitation is {}%.", self.high_temp, self.low_temp,
//         self.chance_of_precipitation)
//     }
// }