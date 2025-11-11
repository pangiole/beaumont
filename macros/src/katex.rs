use proc_macro2::TokenStream;
use quote::quote;


// This is a copy of https://katex.org/docs/autorender.html (accessed on 2025-11-16)
const KATEX_HEADER: &str = r#"
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/katex.min.css" integrity="sha384-WcoG4HRXMzYzfCgiyfrySxx90XSl2rxY5mnVY5TwtWE6KLrArNKn0T/mOgNL0Mmi" crossorigin="anonymous">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/katex.min.js" integrity="sha384-J+9dG2KMoiR9hqcFao0IBLwxt6zpcyN68IgwzsCSkbreXUjmNVRhPFTssqdSGjwQ" crossorigin="anonymous"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/contrib/auto-render.min.js" integrity="sha384-hCXGrW6PitJEwbkoStFjeJxv+fSOOQKOPbJxSfM6G5sWZjAyWhXiTIIAmQqnlLlh" crossorigin="anonymous"></script>
<script>
    document.addEventListener("DOMContentLoaded", function() {
        renderMathInElement(document.body, {
          // customised options
          // • auto-render specific keys, e.g.:
          delimiters: [
              {left: '$$', right: '$$', display: true},
              {left: '$', right: '$', display: false},
              {left: '\\(', right: '\\)', display: false},
              {left: '\\[', right: '\\]', display: true}
          ],
          // • rendering keys, e.g.:
          throwOnError : false
        });
    });
</script>
"#;

pub fn transform(item: TokenStream) -> TokenStream {
    if let Ok(mut item) = syn::parse2(item.clone()) {
        match item {
            syn::Item::Const(syn::ItemConst { ref mut attrs, .. })
            | syn::Item::Enum(syn::ItemEnum { ref mut attrs, .. })
            | syn::Item::ExternCrate(syn::ItemExternCrate { ref mut attrs, .. })
            | syn::Item::Fn(syn::ItemFn { ref mut attrs, .. })
            | syn::Item::ForeignMod(syn::ItemForeignMod { ref mut attrs, .. })
            | syn::Item::Impl(syn::ItemImpl { ref mut attrs, .. })
            | syn::Item::Macro(syn::ItemMacro { ref mut attrs, .. })
            | syn::Item::Mod(syn::ItemMod { ref mut attrs, .. })
            | syn::Item::Static(syn::ItemStatic { ref mut attrs, .. })
            | syn::Item::Struct(syn::ItemStruct { ref mut attrs, .. })
            | syn::Item::Trait(syn::ItemTrait { ref mut attrs, .. })
            | syn::Item::TraitAlias(syn::ItemTraitAlias { ref mut attrs, .. })
            | syn::Item::Type(syn::ItemType { ref mut attrs, .. })
            | syn::Item::Union(syn::ItemUnion { ref mut attrs, .. })
            | syn::Item::Use(syn::ItemUse { ref mut attrs, .. }) => {
                attrs.push(syn::parse_quote! { #[doc = #KATEX_HEADER] });
            }
            _ => {}
        }
        return quote! { #item };
    }

    if let Ok(mut file) = syn::parse2::<syn::File>(item) {
        file.attrs
            .push(syn::parse_quote! { #![doc = #KATEX_HEADER] });
        return quote! { #file };
    }

    todo!()
}