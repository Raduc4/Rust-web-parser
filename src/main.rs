use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
     foreign_links{
         ReqError(reqwest::Error);
    IoError(std::io::Error);
  }
}

#[tokio::main]
async fn main() -> Result<()> {
    let resp = reqwest::get("https://ro.wikipedia.org/wiki/Guy_de_Maupassant")
        .await?
        .text()
        .await?;

    Document::from(resp.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| {
            if x.contains("http") || x.contains("https") {
                println!("{}", x)
            }
        });

    Ok(())
}
