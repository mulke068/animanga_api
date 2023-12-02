#[macro_export]
macro_rules! search {
    ($struct_name: ident { $($field: ident : $ty: ty),* $(,)? }) => {

        trait ContentField {
            fn base(&self) -> $struct_name;
        }


        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct $struct_name {
            $(pub $field: $ty,)*
        }

        impl ContentField for $struct_name {
            fn base(&self) -> $struct_name {
                self.clone()
            }
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ContentCreate {
            id: usize,
            // uid: String,

            #[serde(flatten)]
            base: $struct_name,
        }

        #[derive(Debug, Deserialize, Serialize)]
        struct ContentRecord {
            id: usize,
            // uid: String,

            #[serde(flatten)]
            base: $struct_name,
        }

        impl $struct_name {
            pub async fn latest_id(&self, index: Index, increase: usize) -> usize {
                match index.get_stats().await {
                    Ok(data) => data.number_of_documents + increase,
                    Err(_) => 0,
                }
            }

            pub async fn create(&self, index: Index) -> TaskInfo {
                index
                    .add_documents(
                        &[ContentCreate {
                            id: self.latest_id(index.clone(), 1).await,
                            // uid: crate::splitted_data_at(self.uid.clone(), ":"),
                            base: $struct_name {
                                $($field: self.$field.clone(),)*
                            },
                        }],
                        Some("id"),
                    )
                    .await
                    .unwrap()
            }

            pub async fn update(&self,mut index: Index) -> TaskInfo {
                index.primary_key = Some(self.db_id.to_string());
                index.update().await.unwrap()
            }

        }
    };


}
