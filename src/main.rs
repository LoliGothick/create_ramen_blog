use anyhow::{bail};
use clap::Parser;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

fn command(slug: &str, title: &str, cover: impl AsRef<Path>) -> anyhow::Result<()> {
    let created_at = chrono::Utc::now();
    let template = format!(
        r#"
        ---
        title: '{title}'
        excerpt: '#'
        coverImage: '/assets/blog/{slug}/cover.jpg'
        date: '${created_at}'
        author:
          name: Mitama
          picture: '/assets/blog/authors/mitama.jpg'
        ogImage:
          url: '/assets/blog/{slug}/cover.jpg'
        ---

        ## お店の詳細

        :::
        ここにお店の詳細を書く
        :::

        ## 注文の詳細

        :::
        ここにラーメンの写真を貼る
        :::

        ## 感想

        :::
        ここにラーメンの写真を貼る
        :::

        ## 補足情報

        :::
        補足情報があれば書く
        :::

    "#
    );

    fn touch(path: impl AsRef<Path> + Display) -> anyhow::Result<()> {
        match OpenOptions::new().create(true).write(true).open(&path) {
            Ok(_) => Ok(()),
            Err(_e) => bail!("{path} is already exists"),
        }
    }

    let path = format!("./_posts/{slug}.md");
    touch(&path)?;
    let mut post = File::create(&path)?;
    post.write_all(template.as_bytes())?;

    let path = format!(
        "./public/assets/blog/{slug}/{}",
        cover
            .as_ref()
            .file_name()
            .unwrap()
            .to_string_lossy()
    );
    touch(path)?;

    let mut asset = File::create(&path);
    let mut cover_image = File::open(cover)?;
    let mut buffer = String::new();
    cover_image.read_to_string(&mut buffer)?;
    asset.write_all(buffer.as_bytes())?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    slug: String,

    #[arg(short, long)]
    title: String,

    #[arg(short, long)]
    cover: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    command(&args.slug, &args.title, &args.cover)?;

    Ok(())
}
