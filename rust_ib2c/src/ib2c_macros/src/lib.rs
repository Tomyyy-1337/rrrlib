use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemFn, ItemStruct, Pat, Stmt, Type};

/// Automatically ports (activity, target_rating, stimulation, inhibition) to a struct
#[proc_macro_attribute]
pub fn module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a struct
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident.clone();
    let vis = input.vis; 

    let attrs = input.attrs.clone();

    let generics = input.generics.clone();
    let (_, _, where_clause) = generics.split_for_impl();

    let fields = if let Fields::Named(fields_named) = input.fields.clone() {
        fields_named.named
    } else {
        return TokenStream::from(quote! {
            compile_error!("The #[module] macro can only be applied to structs with named fields.");
        });
    };

    let expanded = quote! {
        #(#attrs)*
        #[ports]
        #vis struct #struct_name #generics
        #where_clause
        {
            #fields
        }
    };

    TokenStream::from(expanded)
}

/// Adds standard ports (activity, target_rating, stimulation, inhibition) and implements MetaSignals and UpdateReceivePorts
#[proc_macro_attribute]
pub fn group(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a struct
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident.clone();
    let vis = input.vis; 

    let attrs = input.attrs.clone();

    let generics = input.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = if let Fields::Named(fields_named) = input.fields.clone() {
        fields_named.named
    } else {
        return TokenStream::from(quote! {
            compile_error!("The #[module] macro can only be applied to structs with named fields.");
        });
    };

    let expanded = quote! {
        #(#attrs)*
        #[ports]
        #vis struct #struct_name #generics
        #where_clause
        {
            #fields
        }

        impl #impl_generics #struct_name #ty_generics
        #where_clause
        {
            fn set_characteristic_module<M>(&mut self, module: &mut M) 
            where
                M: MetaSignals + UpdateReceivePorts + 'static,
            {
                self.activity.connect_to_source(module.get_activity_port());
                self.target_rating.connect_to_source(module.get_target_rating_port());

                self.stimulation.connect_as_source(module.get_stimulation_port());
                self.inhibition.connect_as_source(module.get_inhibition_port());
            }
        }
    };

    TokenStream::from(expanded)
}

/// Automatically adds activity, target_rating, stimulation, and inhibition ports to a struct
#[proc_macro_attribute]
pub fn ports(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a struct
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident.clone();
    let vis = input.vis; 

    let attrs = input.attrs.clone();

    let generics = input.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = if let Fields::Named(fields_named) = input.fields.clone() {
        fields_named.named
    } else {
        return TokenStream::from(quote! {
            compile_error!("The #[module] macro can only be applied to structs with named fields.");
        });
    };

    let mut receive_port_updates = Vec::new();
    for field in &fields {
        let field_name = field.ident.clone().unwrap();

        if let Type::Path(type_path) = &field.ty {
            if let Some(ident) = type_path.path.segments.last().map(|s| &s.ident) {
                if ident == "ReceivePort" || ident == "ParameterPort" {
                    receive_port_updates.push(quote! {
                        self.#field_name.update();
                    });
                }
            }
        }
    }   

    let expanded = quote! {
        #(#attrs)*
        #[derive(Default)]
        #vis struct #struct_name #generics
        #where_clause
        {
            #fields
            pub activity: SendPort<MetaSignal>,
            pub target_rating: SendPort<MetaSignal>,
            pub stimulation: ReceivePort<MetaSignal>,
            pub inhibition: ReceivePort<MetaSignal>,
            delta_time: std::time::Duration,
        }

        impl #impl_generics MetaSignals for #struct_name #ty_generics
        #where_clause
        {
            fn set_activity(&mut self, activity: MetaSignal) {
                self.activity.send(activity);
            }

            fn get_activity(&self) -> Option<MetaSignal> {
                self.activity.get()
            }

            fn get_activity_port(&self) -> &SendPort<MetaSignal> {
                &self.activity
            }

            fn set_target_rating(&mut self, target_rating: MetaSignal) {
                self.target_rating.send(target_rating);
            }
            
            fn get_target_rating(&self) -> Option<MetaSignal> {
                self.target_rating.get()
            }
            
            fn get_target_rating_port(&self) -> &SendPort<MetaSignal> {
                &self.target_rating
            }

            fn get_stimulation(&self) -> Option<&MetaSignal> {
                self.stimulation.get()
            }

            fn get_inhibition(&self) -> Option<&MetaSignal> {
                self.inhibition.get()
            }

            fn get_stimulation_port(&mut self) -> &ReceivePort<MetaSignal> {
                &mut self.stimulation
            }

            fn get_inhibition_port(&mut self) -> &ReceivePort<MetaSignal> {
                &mut self.inhibition
            }

            fn set_delta_time(&mut self, delta_time: std::time::Duration) {
                self.delta_time = delta_time;
            }
        }

        impl #impl_generics UpdateReceivePorts for #struct_name #ty_generics
        #where_clause
        {
            fn update_all_ports(&mut self) {
                #(#receive_port_updates)*
                self.stimulation.update();
                self.inhibition.update();
            }
        }

    };

    TokenStream::from(expanded)
}

/// Automatically spawns all BehaviorModules, MaximumFusions, and BehaviorGroups defined in a function
/// 
/// # Example
/// ```rust	ignore
/// use ib2c::prelude::*;
/// 
/// struct MyGroup { 
///     /// ... your fields ...
/// }
/// 
/// impl Group for MyGroup {
///     #[spawn]
///     fn init(&mut self, cycle_time: std::time::Duration) {
///         let module1 = BehaviorModule::<Module1>::with_name("Sender 1", cycle_time);
///         let module2 = BehaviorModule::<Module2>::with_name("Sender 2", cycle_time);
///         let mut fusion_module = MaximumFusion::with_name("Fusion", cycle_time);
///         // ... connect ports ......      
///     }
/// }
/// ```
/// This will automatically call `module1.spawn()`, `module2.spawn()`, and `fusion_module.spawn()` at the end of the `init` function.
#[proc_macro_attribute]
pub fn spawn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let mut spawn_stmts = Vec::new();
    for stmt in &input.block.stmts {
        if let Stmt::Local(local) = stmt {
            let var = if let Pat::Ident(pat_ident) = &local.pat {
                &pat_ident.ident
            } else {
                continue;
            };

            if let Some(local_init) = &local.init {
                if let syn::Expr::Call(call) = &*local_init.expr {
                    if let syn::Expr::Path(path) = &*call.func {
                        for seg in &path.path.segments {
                            let type_ident = &seg.ident;
                            if type_ident == "BehaviorModule" || type_ident == "MaximumFusion" {
                                // Parse the quoted statement into a Stmt
                                let stmt: Stmt = syn::parse_quote! { #var.spawn(); };
                                spawn_stmts.push(stmt);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    // Insert spawn calls at the end of the function
    input.block.stmts.extend(spawn_stmts);

    TokenStream::from(quote! { #input })
}