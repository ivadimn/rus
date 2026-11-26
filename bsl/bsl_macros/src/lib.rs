use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn say_hello(input: TokenStream) -> TokenStream {
    let msg = parse_macro_input!(input as syn::LitStr);

    let expanded = quote! {
        println!("{}", #msg);
    };

    expanded.into()
}

#[proc_macro_derive(Describable)]
pub fn derive_describable(input: TokenStream) -> TokenStream {
    // Шаг 1: разбираем входной TokenStream в удобную структуру
    let input = parse_macro_input!(input as DeriveInput);

    // Шаг 2 берём имя структцры
    let name = input.ident;
    let field_names: Vec<_> = if let syn::Data::Struct(data) = &input.data {
        if let syn::Fields::Named(fields) = &data.fields {
            fields.named.iter()
                .map(|f| f.ident.as_ref().unwrap().clone())
                .collect()
        } else { vec![] }
    } else { vec![] };

    //Шаг 3 генерируем реализацию
    let expanded = quote! {
        impl Describable for #name {
            fn describe(&self) -> String {
                let fields = vec![
                    #( format!("{}: {:?}", stringify!(#field_names), self.#field_names) ),*
                ];
                format!("{} {{ {} }}", stringify!(#name), fields.join(", "))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Transaction,  attributes(transaction))]
pub fn derive_transaction(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut kind = "deposit";
    for attr in &input.attrs {
        if attr.path().is_ident("transaction") {
            if let Ok(meta) = attr.parse_args::<syn::LitStr>() {
            let val = meta.value();
            if val == "withdraw" {
                kind = "withdraw";
            } else if val == "transfer"  {
                kind = "transfer";
            }
        }    
        }
        
    } 

    let body = match kind {
        "deposit" => quote! {
            match  storage.get_accounts().get(&self.account) {
                    Some(_) => storage.deposit(&self.account, self.amount)?, 
                    None => return Err(StorageError::UserNotFound(self.account.clone())),
            };
            
        },
        "withdraw" => quote! {
            match  storage.get_accounts().get(&self.account) {
                    Some(_) => storage.withdraw(&self.account, self.amount)?, 
                    None => return Err(StorageError::UserNotFound(self.account.clone())),
            };
            
        },
        "transfer" => quote! {
             if !storage.get_accounts().contains_key(&self.to) {
                return Err(StorageError::UserNotFound(self.to.clone()));
            }

            storage.withdraw(&self.from, self.amount)?;
            storage.deposit(&self.to, self.amount)?;
        
            
        },
        _ => panic!("Неизвестный тип транзакции {}", kind),
    };


    let expanded = quote! {
        impl Transaction for #name {
            fn apply(&self, storage: &mut Storage) -> Result<(), StorageError> {

                #body
                Ok(())    
            }
        }
    };
    TokenStream::from(expanded)
}