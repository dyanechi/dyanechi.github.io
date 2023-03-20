use sycamore::prelude::*;
use anyhow::Result;

use crate::components::navbar::Navbar;
use dy_macro::{style::*, css};



#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    // let css = "some";
    // let app_css = css!({
    //     display: "flex";
    //     padding: "3em";
    //     background_color: "green";
    // });

    view! { cx,
        div(class="") {  }
        h1 { "hey man" } 
        Navbar {
            
        }
        
    }
}



#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test() {
        let css_class = css!({
            display: "flex";
            padding: "3em";
            background_color: "green";
        });

        eprintln!("{:?}", css_class);
        // panic!();
    }
}