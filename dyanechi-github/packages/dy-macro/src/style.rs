use std::{fs::{self, File, remove_file, read, write}, path::Path};
use std::io::prelude::*;

use dy_utils::rand_string;
use regex::Regex;

const CSS_DIR: &'static str = "../../css/";

pub type ClassName = String;

pub trait Build {
    fn build(&self) -> ClassName;
}

pub struct CssNode {
    id: ClassName,
    inner: String,
}
impl CssNode {
    pub fn new(inner: impl Into<String>) -> Self {
        CssNode { id: format!("dy-{}", rand_string(8)), inner: inner.into() }
    }

    pub fn id(&self) -> ClassName { self.id.to_owned() }
    pub fn with_inner(mut self, inner: impl Into<String>) -> Self { self.inner = inner.into(); self }

    fn _clean(s: String, chars: &str) -> String {
        let mut str = s.to_owned();
        let reserved = "{}[]";
        for c in chars.chars() {
            let char = c.to_string();
            let reg_char = 
                if reserved.contains(&char) { format!(r"\{}", char) }
                else { char.to_string() };
            
            let re = Regex::new(&format!(r"\s{}\s", reg_char)).unwrap();
            str = re.replace_all(&str, &char).to_string();
        }
        str
    }
}
impl Clone for CssNode {
    fn clone(&self) -> Self {
        CssNode { id: rand_string(8), inner: self.inner.clone() }
    }
}
impl Build for CssNode {
    fn build(&self) -> ClassName {
        let css_dir = Path::new(CSS_DIR);
        let css_tmp_dir = css_dir.join(".tmp");
        let css_file = css_tmp_dir.join(format!("{}.css", self.id));

        let css_text = self.inner.to_ascii_lowercase().replace("_", "-");
        let css_text = Self::_clean(css_text, ":;{}");

        if !css_tmp_dir.exists() { fs::create_dir_all(&css_tmp_dir).unwrap(); }
        if css_file.exists() { remove_file(&css_text).unwrap(); }

        File::create(&css_file).unwrap();
        fs::write(&css_file, &css_text).unwrap();

        self.id.to_owned()
    }
}



#[macro_export]
macro_rules! css {
    (
        &$(
            $($(.)?$selector:ident)*
            {
                $($key:ident: $prop:expr;)*
            }
        )*
    ) => {
        stringify!(
            $(
                $($selector)*
                {
                    $($key: $prop;)*
                }
            )*
        )
    };
    
    (
        {
            $($key:ident: $prop:expr;)*
            $(
                &$($(.)?$selector:ident)*
                {
                    $($sub_key:ident: $sub_prop:expr;)*
                }
            )*
        }
    ) => {
        {
            let css_node = CssNode::new("");
            let inner = stringify!(
                $($key: $prop;)*
            );

            let mut expanded = String::new();
            $(expanded.push_str(&format!("\n.{} {}",
                css_node.id(),
                css!(
                    &$($selector)*
                    {
                        $($sub_key: $sub_prop;)*
                    }
                )
            ));)*

            let inner = format!(".{} {{ {} }}{}", css_node.id(), inner, expanded);
            css_node.with_inner(inner).build()
        }
    };
}


pub fn build_css() {
    let css_dir = Path::new("./css");
    let css_tmp_dir = css_dir.join(".tmp");
    let css_index = css_dir.join("index.css");
    let css_root = css_dir.join("root.css");
    
    // if !css_dir.exists() { fs::create_dir_all(&css_dir).unwrap(); }
    if !css_tmp_dir.exists() { eprintln!("WARN!: No css files exist. Tried to load from: {}", css_tmp_dir.display()); return; } 
    
    if css_index.exists() { remove_file(&css_index).unwrap(); }
    File::create(&css_index).unwrap();
    
    let mut index_content = String::new();
    
    // Put all `root.css` styles into `index.css` final build
    if css_root.exists() {
        for line in read(&css_root).unwrap().lines() {
            if let Ok(line) = line {
                if line.is_empty() { continue; }
                index_content.push_str(&line.replace(r"\s", " "));
            }
        }
    }
    
    // Add all temp built styles to `index.css` file
    let tmp_read_dir = css_tmp_dir.read_dir().unwrap();
    for dir_entry in tmp_read_dir {
        let entry = dir_entry.unwrap();
        let read_file = read(&entry.path()).unwrap();

        for line in read_file.lines() {
            index_content.push_str(line.unwrap().as_str());
        }
    }

    write(&css_index, &index_content).unwrap();

}

pub fn amazing_shit() -> String {
    "S Amazing Shit!!".into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let class = css!({
            color: rgb(4, 5, 2);
            background_color: blue;

            &div p {
                color: white;
                margin: 1px;
            }

            &h1 {
                margin: none;
            }
        });

        // assert_eq!(class.inner, ".class_name div p { color : yellow ; background_color : blue ; } div {}");
    }
}