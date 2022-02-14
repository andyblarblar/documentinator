use std::fmt::Write as FmtWrite;

use anyhow::{Context, Result};

use crate::doc_types::Node;
use crate::generators::Generator;
use crate::Doc;

/// Generates docs to markdown.
#[derive(Default)]
pub struct MarkdownGenerator {}

impl Generator for MarkdownGenerator {
    fn generate_string(&self, nodes: Doc) -> anyhow::Result<Vec<(String, String)>> {
        fn gen_single(node: Node) -> Result<String> {
            let mut buf = String::new();

            writeln!(&mut buf, "# {}", node.node_name)?;

            //Source files
            writeln!(&mut buf, "# File")?;
            for src in node.source_file {
                writeln!(&mut buf, "`{}`", src)?;
            }
            writeln!(&mut buf)?;

            //Summary
            writeln!(&mut buf, "## Summary \n {}", node.summary)?;

            //Topics
            if node.subscribes.is_some() || node.publishes.is_some() {
                writeln!(&mut buf, "## Topics\n")?;

                if let Some(pubs) = node.publishes {
                    writeln!(&mut buf, "### Publishes")?;

                    for pubz in pubs {
                        writeln!(&mut buf, "- `{}`: {}", pubz.name, pubz.description)?;
                    }
                    writeln!(&mut buf)?;
                }

                if let Some(subs) = node.subscribes {
                    writeln!(&mut buf, "### Subscribes")?;

                    for sub in subs {
                        writeln!(&mut buf, "- `{}`: {}", sub.name, sub.description)?;
                    }
                    writeln!(&mut buf)?;
                }
            }

            //Params
            if let Some(params) = node.params {
                writeln!(&mut buf, "## Params")?;

                for param in params {
                    writeln!(&mut buf, "- `{}`: {}", param.name, param.description)?;
                }
                writeln!(&mut buf)?;
            }

            //Improvements
            if let Some(improve) = node.potential_improvements {
                writeln!(&mut buf, "## Potential Improvements")?;
                writeln!(&mut buf, "{} \n", improve)?;
            }

            //Launchfile
            writeln!(
                &mut buf,
                "# Launch \n `{}` \n {} \n",
                node.launch.file_path, node.launch.usage
            )?;

            //Launch args
            if let Some(args) = node.launch.args {
                writeln!(&mut buf, "## Args")?;

                for arg in args {
                    writeln!(&mut buf, "- `{}`: {}", arg.name, arg.description)?;
                }
                writeln!(&mut buf)?;
            }

            //Misc
            if let Some(misc) = node.misc {
                writeln!(&mut buf, "# Misc \n {} ", misc)?;
            }

            Ok(buf)
        }

        let mut nodes_doc = vec![];

        //Actually gen docs
        for node in nodes.nodes {
            let node_name = node.node_name.clone();
            nodes_doc.push((
                node.node_name.clone(),
                gen_single(node).context(format!(
                    "Failed to generate markdown for node: {}",
                    node_name
                ))?,
            ));
        }

        Ok(nodes_doc)
    }

    fn add_file_extension(&self, filename: &str) -> String {
        format!("{filename}.md")
    }
}

#[cfg(test)]
mod test {
    use crate::generators::markdown::MarkdownGenerator;
    use crate::generators::Generator;
    use crate::{ConfigParser, TomlParser};

    #[test]
    fn test_md_gen() {
        let wld = include_str!("../../test_assets/white_line_detection.doctor.toml");
        let parser = TomlParser::default();
        let parsed = parser.parse_str(wld.to_string()).unwrap();

        let gen = MarkdownGenerator::default();

        let md = gen.generate_string(parsed).unwrap();

        //Just print as its very hard to ensure a common output across platforms due to line endings
        println!("{md:?}")
    }
}
