use anyhow::bail;
use clap::Parser;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

fn command(slug: &str, title: &str, cover: impl AsRef<Path>) -> anyhow::Result<()> {
    let created_at = chrono::Utc::now();
    let template = format!(
        indoc::indoc! {r#"
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
    "#},
        title = title,
        created_at = created_at,
        slug = slug,
    );

    fn touch(path: impl AsRef<Path>) -> anyhow::Result<()> {
        match OpenOptions::new().create(true).write(true).open(&path) {
            Ok(_) => Ok(()),
            Err(e) => bail!("{e:?}"),
        }
    }

    let path = format!("./_posts/{slug}.md");
    touch(&path)?;
    let mut post = File::create(&path)?;
    post.write_all(template.as_bytes())?;

    let cover_path = format!(
        "./public/assets/blog/{slug}/cover.{}",
        cover.as_ref().extension().unwrap().to_string_lossy()
    );
    let path = Path::new(cover_path.as_str());
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let cover_image = image::open(cover).unwrap();
    cover_image.save(cover_path).unwrap();

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
