use super::*;


pub fn Navbar<G: Html>(cx: Scope) -> View<G> {
    let links = vec!["Start", "Develop", "Contact", "Projects", "Skills"];
    let view_links = View::new_fragment(
        links
            .iter()
            .map(|&x| view! { cx, a(href="https://") { (x) } }).collect()
    );

    let onclick = || {
        ""
    };

    view! { cx,
        header(class="Nav-Header") {
            nav {

                (view_links)
                // Keyed(
                //     iterable=links,
                //     view=|cx, x| view! { cx, 
                //         a (href="https://", onclick=onclick) {
                //             (x)
                //         }
                //     },
                //     key=|x| *x
                // )
                
            }
        }

    }
}
