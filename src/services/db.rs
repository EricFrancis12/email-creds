use futures_util::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, DateTime, Document},
    options::UpdateModifications,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection, Database,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    env, error,
    io::{
        Error,
        ErrorKind::{self, InvalidInput, Other},
    },
};
use std::{ptr::null, str::FromStr};

use crate::{
    models::{email_list_model::EmailList, subscriber_model::Subscriber, user_model::User},
    utils::error::fmt_err,
};

pub struct Storage {
    client: Client,
    db: Database,
    email_list: Collection<EmailList>,
    subscriber: Collection<Subscriber>,
    user: Collection<User>,
}

enum CollName {
    EmailList,
    Subscriber,
    User,
}

pub trait CollectionTrait: Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl CollectionTrait for Collection<EmailList> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CollectionTrait for Collection<Subscriber> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CollectionTrait for Collection<User> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Storage {
    pub fn new(client: Client) -> Self {
        let db = client.database("email-creds");
        Storage {
            client,
            email_list: db.collection("email_list"),
            subscriber: db.collection("subscriber"),
            user: db.collection("user"),
            db,
        }
    }

    pub async fn init() -> Result<Self, Error> {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27018/?directConnection=true".to_string(),
        };
        match Client::with_uri_str(uri).await {
            Ok(client) => Ok(Storage::new(client)),
            Err(err) => Err(fmt_err(err, Other)),
        }
    }

    fn get_coll<D: Send + Sync>(&self, coll_name: CollName) -> &Collection<D> {
        match coll_name {
            CollName::EmailList => &self.email_list,
            CollName::Subscriber => &self.subscriber,
            CollName::User => &self.user,
        }
    }

    pub async fn get_all<D: Send + Sync + DeserializeOwned>(
        &self,
        coll_name: CollName,
    ) -> Result<Vec<D>, Error> {
        get_many_docs(self.get_coll(coll_name), None).await
    }

    pub async fn get_one_by_id<D: Send + Sync + DeserializeOwned>(
        &self,
        coll_name: CollName,
        id: &str,
    ) -> Result<D, Error> {
        get_doc_by_id(self.get_coll(coll_name), id).await
    }

    pub async fn insert_new<D: Send + Sync + Serialize>(
        &self,
        coll_name: CollName,
        doc: Document,
    ) -> Result<InsertOneResult, Error> {
        insert_new_doc(self.get_coll(coll_name), doc).await
    }

    pub async fn update_one_by_id<D: Send + Sync>(
        &self,
        coll_name: CollName,
        id: &str,
        update: impl Into<UpdateModifications>,
    ) -> Result<UpdateResult, Error> {
        update_doc_by_id(self.get_coll(coll_name), id, update).await
    }

    pub async fn delete_one_by_id<D: Send + Sync>(
        &self,
        coll_name: CollName,
        id: &str,
    ) -> Result<DeleteResult, Error> {
        delete_doc_by_id(self.get_coll(coll_name), id).await
    }
}

pub async fn get_many_docs<D: Send + Sync + DeserializeOwned>(
    coll: &Collection<D>,
    filter: Option<Document>,
) -> Result<Vec<D>, Error> {
    // if no filter is passed in, fallback to the default filter that will return all documents
    let _filter = filter.unwrap_or_else(|| doc! {});

    let mut cursor = match coll.find(_filter).await {
        Ok(csr) => csr,
        Err(err) => return Err(fmt_err(err, Other)),
    };

    let mut docs: Vec<D> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                docs.push(doc);
            }
            Err(err) => return Err(fmt_err(err, Other)),
        }
    }

    Ok(docs)
}

pub async fn get_doc_by_id<D: Send + Sync + DeserializeOwned>(
    coll: &Collection<D>,
    id: &str,
) -> Result<D, Error> {
    let object_id = match ObjectId::from_str(id) {
        Ok(id) => id,
        Err(err) => {
            return Err(fmt_err(err, InvalidInput));
        }
    };

    let filter = doc! { "_id": object_id };

    match coll.find_one(filter).await {
        Ok(Some(doc)) => Ok(doc),
        Ok(None) => Err(Error::new(ErrorKind::NotFound, "document not found")),
        Err(err) => Err(fmt_err(err, Other)),
    }
}

pub async fn insert_new_doc<D: Send + Sync + Serialize>(
    coll: &Collection<D>,
    doc: D,
) -> Result<InsertOneResult, Error> {
    match coll.insert_one(doc).await {
        Ok(result) => Ok(result),
        Err(err) => Err(fmt_err(err, Other)),
    }
}

pub async fn update_doc_by_id<D: Send + Sync>(
    coll: &Collection<D>,
    id: &str,
    update: impl Into<UpdateModifications>,
) -> Result<UpdateResult, Error> {
    let object_id = match ObjectId::from_str(id) {
        Ok(id) => id,
        Err(err) => {
            return Err(fmt_err(err, InvalidInput));
        }
    };

    match coll.update_one(doc! {"_id": object_id}, update).await {
        Ok(result) => Ok(result),
        Err(err) => {
            return Err(fmt_err(err, Other));
        }
    }
}

pub async fn delete_doc_by_id<D: Send + Sync>(
    coll: &Collection<D>,
    id: &str,
) -> Result<DeleteResult, Error> {
    let object_id = match ObjectId::from_str(id) {
        Ok(id) => id,
        Err(err) => {
            return Err(fmt_err(err, InvalidInput));
        }
    };

    match coll.delete_one(doc! {"_id": object_id}).await {
        Ok(result) => Ok(result),
        Err(err) => {
            return Err(fmt_err(err, Other));
        }
    }
}
