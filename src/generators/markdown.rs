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
        fn gen_single(node: &Node, nodes: &Doc) -> Result<String> {
            let mut buf = String::new();

            writeln!(&mut buf, "# {}", node.node_name)?;

            //Add repo link
            if let Some(ref lnk) = nodes.repo {
                writeln!(&mut buf, "From package '[{}]({})'", nodes.package_name, lnk)?;
            } else {
                writeln!(&mut buf, "From package '{}'", nodes.package_name)?;
            }

            //Source files
            writeln!(&mut buf, "# File")?;
            for src in node.source_file.clone() {
                writeln!(&mut buf, "`{}`", src)?;
            }
            writeln!(&mut buf)?;

            //Summary
            writeln!(&mut buf, "## Summary \n {}", node.summary)?;

            //Topics
            if node.subscribes.is_some() || node.publishes.is_some() {
                writeln!(&mut buf, "## Topics\n")?;

                if let Some(ref pubs) = node.publishes {
                    writeln!(&mut buf, "### Publishes")?;

                    for pubz in pubs {
                        writeln!(&mut buf, "- `{}`: {}", pubz.name, pubz.description)?;
                    }
                    writeln!(&mut buf)?;
                }

                if let Some(ref subs) = node.subscribes {
                    writeln!(&mut buf, "### Subscribes")?;

                    for sub in subs {
                        writeln!(&mut buf, "- `{}`: {}", sub.name, sub.description)?;
                    }
                    writeln!(&mut buf)?;
                }
            }

            //Params
            if let Some(ref params) = node.params {
                writeln!(&mut buf, "## Params")?;

                for param in params {
                    writeln!(&mut buf, "- `{}`: {}", param.name, param.description)?;
                }
                writeln!(&mut buf)?;
            }

            //Improvements
            if let Some(ref improve) = node.potential_improvements {
                writeln!(&mut buf, "## Potential Improvements")?;
                writeln!(&mut buf, "{} \n", improve)?;
            }

            //Launchfile
            if let Some(ref launch) = node.launch {
                writeln!(&mut buf, "# Launch")?;

                for launch in launch {
                    writeln!(&mut buf, "## File \n {} \n \n {} \n", launch.file_path, launch.usage)?;

                    //Remappings
                    if let Some(ref remappings) = launch.remap {
                        writeln!(&mut buf, "### Remappings")?;

                        for map in remappings {
                            writeln!(&mut buf, "- from `{}` to `{}`", map.from, map.to)?;
                        }
                        writeln!(&mut buf)?;
                    }

                    //Launch args
                    if let Some(ref args) = launch.args {
                        writeln!(&mut buf, "### Args")?;

                        for arg in args {
                            writeln!(&mut buf, "- `{}`: {}", arg.name, arg.description)?;
                        }
                        writeln!(&mut buf)?;
                    }
                }
            }

            //Misc
            if let Some(ref misc) = node.misc {
                writeln!(&mut buf, "# Misc \n {} ", misc)?;
            }
            Ok(buf)
        }

        let mut nodes_doc = vec![];

        //Actually gen docs
        for node in nodes.nodes.iter() {
            let node_name = node.node_name.clone();
            nodes_doc.push((
                node.node_name.clone(),
                gen_single(node, &nodes).context(format!(
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
