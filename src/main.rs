pub mod metadata;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test = metadata::Metadata::new("https://hral.xyz".into(), None, r#"
        metadata.title = "Wikipedia"
        metadata:push_tag("foo", "bar")
        metadata:push_tag("hello", "world")
        metadata:push_file(metadata.source.."/img/abe_nana.png")
        metadata:push_file(metadata.source.."/img/reimu_tree.png")
    "#.into())?;
    println!("{}", test);
    Ok(())
}