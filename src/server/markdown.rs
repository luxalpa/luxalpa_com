use crate::pages::blog_page::Article;
use serde::de::DeserializeOwned;

#[derive(serde::Deserialize)]
struct Metadata {
    date: String,
    title: String,
    #[serde(rename = "abstract")]
    description: String,
    slug: String,
}

pub fn load_articles() -> anyhow::Result<Vec<Article>> {
    let dir = "content/articles";
    let files = std::fs::read_dir(dir)?;
    let articles = files
        .map(|file| {
            let content = std::fs::read_to_string(file?.path())?;
            let document: Document<Metadata> = parse_frontmatter::<Metadata>(&content)?;
            Ok(Article {
                date: document.metadata.date,
                title: document.metadata.title,
                description: document.metadata.description,
                slug: document.metadata.slug,
                content: document.content,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(articles)
}

/// Parses the front matter header of a Markdown file and returns
/// the body of the Markdown.
/// From: https://github.com/LeoBorai/yaml-front-matter
pub fn parse_frontmatter<T: DeserializeOwned>(markdown: &str) -> anyhow::Result<Document<T>> {
    let yaml = extract(markdown);
    let metadata = serde_yaml::from_str::<T>(yaml.0.as_str())?;

    Ok(Document {
        metadata,
        content: yaml.1,
    })
}

fn extract(markdown: &str) -> (String, String) {
    let mut front_matter = String::default();
    let mut sentinel = false;
    let mut front_matter_lines = 0;
    let lines = markdown.lines();

    for line in lines.clone() {
        front_matter_lines += 1;

        if line.trim() == "---" {
            if sentinel {
                break;
            }

            sentinel = true;
            continue;
        }

        if sentinel {
            front_matter.push_str(line);
            front_matter.push('\n');
        }
    }

    (
        front_matter,
        lines
            .skip(front_matter_lines)
            .collect::<Vec<&str>>()
            .join("\n"),
    )
}

pub struct Document<T: DeserializeOwned> {
    /// A generic type with the structure of the Markdown's
    /// front matter header.
    pub metadata: T,
    /// The body of the Markdown without the front matter header
    pub content: String,
}
