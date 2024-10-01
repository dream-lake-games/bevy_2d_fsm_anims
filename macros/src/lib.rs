use quote::quote;

macro_rules! find_required_attr {
    ($variant:expr, $attr:literal, $error:literal) => {{
        $variant
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == $attr)
            .expect($error)
    }};
}
macro_rules! find_optional_attr {
    ($variant:expr, $attr:literal) => {{
        $variant
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == $attr)
    }};
}

#[proc_macro_derive(
    AnimStateMachine,
    attributes(
        file,
        size,
        length,
        fps,
        offset_x_negative,
        offset_y_negative,
        offset,
        zix,
        render_layers,
        next,
        next_despawn,
    )
)]
pub fn anim_state_machine_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let enum_ident = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
        _ => panic!("Expected a struct with named fields."),
    };
    let mut variant_idents = vec![];
    let mut data_matches = vec![];
    let mut next_matches = vec![];

    for variant in variants {
        let ident = &variant.ident;
        variant_idents.push(quote! { Self::#ident, });
        data_matches.push(make_data_match(variant));
        next_matches.push(make_next_match(variant));
    }

    let gen = quote! {
        impl bevy_2delight_anims::prelude::AnimStateMachine for #enum_ident {
            fn all() -> Vec<Self> {
                vec![#(#variant_idents)*]
            }

            fn get_body(&self) -> AnimBody {
                match self {
                    #(#data_matches)*
                }
            }

            fn get_next(&self) -> AnimNextState<Self> {
                match self {
                    #(#next_matches)*
                }
            }
        }
    };
    gen.into()
}

fn make_data_match(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

    let file = find_required_attr!(variant, "file", "file is required");
    let size = find_required_attr!(variant, "size", "size is required");
    let length = find_optional_attr!(variant, "length");
    let fps = find_optional_attr!(variant, "fps");
    let offset_x_negative = find_optional_attr!(variant, "offset_x_negative");
    let offset_y_negative = find_optional_attr!(variant, "offset_y_negative");
    let offset = find_optional_attr!(variant, "offset");
    let zix = find_optional_attr!(variant, "zix");
    let render_layers = find_optional_attr!(variant, "render_layers");

    let file = match file
        .parse_meta()
        .expect("Cannot parse #[path...] attribute")
    {
        syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 1 => {
            match nested.first().unwrap() {
                syn::NestedMeta::Lit(syn::Lit::Str(file)) => file.value(),
                _ => panic!("Expected file as the only argument of #[file(...)]"),
            }
        }
        _ => {
            panic!(r#"#[file...] attribute should take the form #[file("path/inside/assets.png")]"#)
        }
    };
    let (size_x, size_y) = match size
        .parse_meta()
        .expect("Cannot parse #[size...] attribute")
    {
        syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 2 => {
            let mut nested_iter = nested.iter();
            let (Some(thing1), Some(thing2)) = (nested_iter.next(), nested_iter.next()) else {
                panic!("Expected #[size(thing1, thing2)], but unable to get two things");
            };
            match (thing1, thing2) {
                (
                    syn::NestedMeta::Lit(syn::Lit::Int(x)),
                    syn::NestedMeta::Lit(syn::Lit::Int(y)),
                ) => (
                    x.base10_parse::<u32>().expect("can't parse u32"),
                    y.base10_parse::<u32>().expect("can't parse u32"),
                ),
                _ => panic!("Expected #[size(x, y)] as ints but not syn::Lit::Int's"),
            }
        }
        _ => panic!(r#"#[size...] attribute should take the form #[size(x, y)]"#),
    };
    let length = length
        .map(|length| {
            match length
                .parse_meta()
                .expect("Cannot parse #[length...] attribute")
            {
                syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 1 => {
                    match nested.first().unwrap() {
                        syn::NestedMeta::Lit(syn::Lit::Int(length)) => {
                            length.base10_parse::<u32>().expect("can't parse u32")
                        }
                        _ => panic!("Expected #[length(l)] as int"),
                    }
                }
                _ => panic!(r#"#[length...] attribute should take the form #[length(l)]"#),
            }
        })
        .unwrap_or(1);
    let fps = fps.map(
        |fps| match fps.parse_meta().expect("Cannot parse #[fps...] attribute") {
            syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 1 => {
                match nested.first().unwrap() {
                    syn::NestedMeta::Lit(syn::Lit::Float(fps)) => {
                        fps.base10_parse::<f32>().expect("can't parse f32")
                    }
                    _ => panic!("Expected #[fps(l)] as float"),
                }
            }
            _ => panic!(r#"#[fps...] attribute should take the form #[fps(l)]"#),
        },
    );
    let fps_token = fps.map_or_else(
        || quote! { None },
        |fps_value| quote! { Some(#fps_value as f32) },
    );
    let (offset_x, offset_y) = offset
        .map(|offset| {
            match offset
                .parse_meta()
                .expect("Cannot parse #[offset...] attribute")
            {
                syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 2 => {
                    let mut nested_iter = nested.iter();
                    let (Some(thing1), Some(thing2)) = (nested_iter.next(), nested_iter.next())
                    else {
                        panic!("Expected #[offset(thing1, thing2)], but unable to get two things");
                    };
                    match (thing1, thing2) {
                        (
                            syn::NestedMeta::Lit(syn::Lit::Float(x)),
                            syn::NestedMeta::Lit(syn::Lit::Float(y)),
                        ) => (
                            x.base10_parse::<f32>().expect("can't parse f32")
                                * if offset_x_negative.is_some() {
                                    -1.0
                                } else {
                                    1.0
                                },
                            y.base10_parse::<f32>().expect("can't parse f32")
                                * if offset_y_negative.is_some() {
                                    -1.0
                                } else {
                                    1.0
                                },
                        ),
                        _ => panic!("Expected #[offset(x, y)] as f32s or ints"),
                    }
                }
                _ => panic!(r#"#[offset...] attribute should take the form #[offset(x, y)]"#),
            }
        })
        .unwrap_or((0.0, 0.0));
    let zix = zix
        .map(
            |zix| match zix.parse_meta().expect("Cannot parse #[zix...] attribute") {
                syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 1 => {
                    match nested.first().unwrap() {
                        syn::NestedMeta::Lit(syn::Lit::Float(zix)) => {
                            zix.base10_parse::<f32>().expect("can't parse f32")
                        }
                        _ => panic!("Expected #[zix(l)] as f32"),
                    }
                }
                _ => panic!(r#"#[zix...] attribute should take the form #[zix(l)]"#),
            },
        )
        .unwrap_or(0.0);
    let mut add_render_layers = vec![];
    if let Some(render_layers) = render_layers {
        match render_layers.parse_meta().expect("Cannot parse #[render_layers...] attribute") {
            syn::Meta::List(syn::MetaList {nested, ..}) => {
                for thing in nested.iter() {
                    match thing {
                        syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
                            let ident = p.get_ident().expect("p should have ident").clone();
                            add_render_layers.push(quote! {
                                .add_render_layers(#ident.into())
                            });
                        }
                        _ => panic!("#[render_layers(...)] should contain idents of things implementing Into<RenderLayers>"),
                    }
                }
            }
            _ => panic!("#[render_layers...] attribute should take the form #[render_layers(ident, ident...)"),
        }
    }

    quote! {
        Self::#variant_ident => bevy_2delight_anims::prelude::AnimBody::new(#file, #size_x, #size_y)
            .with_length(#length)
            .with_fps(#fps_token)
            .with_offset(#offset_x, #offset_y)
            .with_zix(#zix)
            #(#add_render_layers)*,
    }
}

fn make_next_match(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

    let next = find_optional_attr!(variant, "next");
    let Some(attr) = next else {
        return quote! {
            Self::#variant_ident => bevy_2delight_anims::prelude::AnimNextState::Stay,
        };
    };

    let ident = match attr.parse_meta().unwrap() {
        syn::Meta::List(syn::MetaList { nested, .. }) => match nested.first().unwrap() {
            syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
                p.get_ident().expect("next should have ident").clone()
            }
            _ => panic!(r#"#[next...] attribute should take the form #[next(ident)]"#),
        },
        _ => panic!(r#"#[next...] attribute should take the form #[next(ident)]2"#),
    };

    if ident.to_string().as_str() == "Despawn" {
        quote! {
            Self::#variant_ident => bevy_2delight_anims::prelude::AnimNextState::Despawn,
        }
    } else if ident.to_string().as_str() == "Remove" {
        quote! {
            Self::#variant_ident => bevy_2delight_anims::prelude::AnimNextState::Remove,
        }
    } else {
        quote! {
            Self::#variant_ident => bevy_2delight_anims::prelude::AnimNextState::Some(Self::#ident),
        }
    }
}
