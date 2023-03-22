use proc_macro;


#[macro_export]
macro_rules! view_component {
    (
        $fname:ident
        $(($arg:ident: $val:ty $(,)*))*
        // $( -> <$($ret:ty)*>)?
        $tt:block

    ) => {
        pub fn $fname<G: Html>(cx: Scope, $($arg: $val)*) -> View<G> {
            view! { cx,
                $tt
                
            }
        }
    };
}
