use std::collections::HashMap;
use serde_json::{Value};

pub struct Firestore {
    pub url: String,
}

impl Firestore {
    pub fn new(url: String) -> Firestore {
        Firestore {
            url
        }
    }
}

#[derive(Debug)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSnapshot {
    /// The resource name of the document.
    /// For example: `projects/{projectId}/databases/{databaseId}/documents/{document_path}`.
    name: String,
    /// The document's fields.
    /// 
    /// The map keys represent field names.
    /// 
    /// A simple field name contains only characters a to z, A to Z, 0 to 9, or _, and must not start with 0 to 9. For example, foo_bar_17.
    fields: HashMap<String, Value>,
    /// ISO timestamp of document creation
    create_time: String,
    /// ISO timestamp of last update to the document
    update_time: String,
}

impl DocumentSnapshot {
    /// Get a string value from the document with the specified `field_name`
    pub fn get_string(&self, field_name: &str) -> String {
        self.fields
            .get(field_name).expect(
                &format!("field {} does not exist", field_name)
            ).get("stringValue").expect(
                &format!("field {} is not a string", field_name)
            ).to_string()
    }

    /// Get an integer value from the document with the specified `field_name`
    pub fn get_u64(&self, field_name: &str) -> u64 {
        self.fields
            .get(field_name).expect(
                &format!("field {} does not exist", field_name)
            ).get("integerValue").expect(
                &format!("field {} is not an integer", field_name)
            ).as_u64().expect(
                &format!("failed to parse field {} as u64", field_name)
            )
    }

    /// Get a float value from the document with the specified `field_name`
    pub fn get_f64(&self, field_name: &str) -> f64 {
        self.fields
            .get(field_name).expect(
                &format!("field {} does not exist", field_name)
            ).get("doubleValue").expect(
                &format!("field {} is not a double", field_name)
            ).as_f64().expect(
                &format!("failed to parse field {} as f64", field_name)
            )
    }

    /// Get an integer value from the document with the specified `field_name`
    pub fn get_geopoint(&self, field_name: &str) -> GeoPoint {
        let json_map = self.fields
            .get(field_name).expect(
                &format!("field {} does not exist", field_name)
            ).get("geoPointValue").expect(
                &format!("field {} is not a geopoint", field_name)
            ).as_object().expect(
                &format!("failed to parse field {} as u64", field_name)
            );

        GeoPoint {
            latitude: json_map.get("latitude").unwrap().as_f64().unwrap(),
            longitude: json_map.get("longitude").unwrap().as_f64().unwrap(),
        }
    }
}

pub struct DocumentReference {
    pub path: String,
}

impl DocumentReference {
   
}

pub struct FieldPath {

}



pub struct FieldValue {

}

pub struct Query {

}

pub struct QuerySnapshot {

}

pub struct Timestamp {

}
