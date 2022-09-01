#![allow(non_snake_case)]

use proc_macro::{TokenStream};
use syn::{*, parse::Parse, punctuated::Punctuated};
use std::time::UNIX_EPOCH;
use quote::quote;
use uuid;

extern crate proc_macro;

struct Parent {
      parent: Type,
}

impl Parse for Parent {
      fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let vars = Punctuated::<Type, Token![,]>::parse_terminated(input)?;
            let parent = match vars.first() {
                  Some(p) => p,
                  None => {
                        panic!("Error: #[object] takes a parent");
                  },
            };

            Ok(Self {
                  parent: parent.clone(),
            })
      }
}

struct ParentFieldName {
      name: Option<Ident>
}

impl Parse for ParentFieldName {
      fn parse(input: parse::ParseStream) -> Result<Self> {
            let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
            let vars: Vec<_> = vars.iter().collect();

            let name = if vars.len() > 1 {
                  Some(vars[1].clone())
            } else {
                  None
            };
            Ok(Self {
                  name
            })
      }
}

fn generateID(name: &Ident) -> String {
      let now = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
      let id = uuid::Uuid::new_v4();
      format!("{:?}{}{}", now, id, name.to_string())
}

fn offsetof(parent: &Type, parentField: ParentFieldName) -> __private::TokenStream2 {
      match parentField.name {
            Some(name) => {
                  return quote! {
                        fn offset() -> isize {
                              let this: *const Self = 0 as *const Self;
                              let parent = unsafe {
                                    &((*this).#name) as *const _ as *const u8
                              };
                              parent as isize
                        }
                  }
            },
            None => {}
      }
      match *parent {
            Type::Path(ref path) => {
                  match path.path.segments.iter().last() {
                        Some(parent) => {
                              // TODO: resolve fully qualified name of DynamicObjectBase
                              // Allow base class to omit parent field
                              if parent.ident.to_string() == "DynamicObjectBase" {
                                    return quote! {
                                          fn offset() -> isize {
                                                0
                                          }
                                    }
                              }
                        },
                        None => {}
                  }
            },
            _ => panic!("Expected super class")
      }
      panic!("Expect parent field name in second argument of attribute");
}

#[proc_macro_attribute]
pub fn subclass(args: TokenStream, tokens: TokenStream) -> TokenStream {
      let argsCpy = args.clone();
      let parse = parse_macro_input!(tokens as ItemStruct);
      let parent = parse_macro_input!(args as Parent).parent;
      let name = &parse.ident;

      let parentField = parse_macro_input!(argsCpy as ParentFieldName);
      let id = generateID(name);
      let offsetof = offsetof(&parent, parentField);

      let result = quote! {
            #parse

            impl dynamic::Class for #name {
                  type Parent = #parent;
                  const NAME:&'static str = #id;

                  fn isa(id: usize) -> bool {
                        println!("In {}", core::any::type_name::<Self>());
                        println!("In id: {}, Self::id: {}", id, Self::id());
                        id == Self::id() || <Self as dynamic::Class>::Parent::isa(id)
                  }

                  // fn offsetof() -> usize;
                  #offsetof

                  fn id() -> usize {
                        Self::id as *const u8 as usize
                  }
            }
      };
      TokenStream::from(result)
}

#[proc_macro_attribute]
pub fn module_name(_: TokenStream, stream: TokenStream) -> TokenStream {
      let parse = parse_macro_input!(stream as ItemStruct);

      let name = &parse.ident;

      let result = quote! {
            #parse

            impl #name {

            }
      };
      TokenStream::from(result)
}
