
use std::fmt::Debug;
use super::descriptive_data;
use super::json_object::JSonObject;
use super::json_object::JSonSerializer;
use super::w3c_list::W3CList;





#[derive(enumset::EnumSetType, Debug)]
///1
pub enum DataSchemaId {
    ///1
    DSIObject,
    ///1
    DSIArray, 
    ///1
    DSIString, 
    ///1
    DSINumber, 
    ///1
    DSIInteger, 
    ///1
    DSIBoolean, 
    ///1
    DSINull
}



///1
pub trait DataSchema : Debug + JSonObject  {
    ///1
    fn get_description(&self) -> Option<String>;
    ///1
    fn set_description(&mut self, v : Option<String>);
    ///1
    fn get_title(&self) -> Option<String>;
    ///1
    fn set_title(&mut self, v : Option<String>);
    ///1
    fn get_i18n_title(&self, k: String) -> Option<&String>;
    ///1
    fn set_i18n_title(&mut self, k : String , v: Option<String>);
    ///1
    fn get_i18n_description(&self, k: String) -> Option<&String>;
    ///1
    fn set_i18n_description(&mut self, k : String , v: Option<String>);
    ///1 
    fn get_type(&self) -> &W3CList<String>;
    ///1
    fn set_type(&mut self, v : &W3CList<String>);
    ///1
    fn get_schema_type(&self) -> Option<DataSchemaId>;
    ///1
    fn get_unit(&self) -> Option<String>;
    ///1
    fn set_unit(&mut self , v : Option<String>);
    ///1
    fn add_oneof(&mut self, v: Box<dyn DataSchema >);
    ///1
    fn get_oneof_list(&self) -> Vec<Box<dyn DataSchema>>;
    ///1
    fn remove_oneof(&self, i : i32);
    ///1
    fn get_format(&self) -> Option<String>;
    ///1
    fn set_format(&mut self , v : Option<String>);
    ///1
    fn get_readonly(&self) -> Option<bool>;
    ///1
    fn set_readonly(&mut self, v : Option<bool>);
    ///1
    fn get_writeonly(&self) ->Option< bool>;
    ///1
    fn set_writeonly(&mut self, v : Option<bool>);

}


/*
///1
pub trait ArrayDataSchema : DataSchema {
    ///1
    fn get_items(&self) -> W3CList<Box<dyn DataSchema>>;
    ///1
    fn set_items(&mut self, v : W3CList<Box<dyn DataSchema>>);

    ///1
    fn get_min_items(&self) -> Option<i32>;
    ///2
    fn set_min_items(&mut self, v : Option<i32>);
    ///1
    fn get_max_items(&self) -> Option<i32>;
    ///2
    fn set_max_items(&mut self, v : Option<i32>);

}

///1
pub trait NumberDataSchema : DataSchema {

    ///1
    fn get_min(&self) -> Option<f64>;
    ///2
    fn set_min(&mut self, v : Option<f64>);
    ///1
    fn get_max(&self) -> Option<f64>;
    ///2
    fn set_max(&mut self, v : Option<f64>);

}

///1
pub trait IntegerDataSchema : DataSchema {

    ///1
    fn get_min(&self) -> Option<i32>;
    ///2
    fn set_min(&mut self, v : Option<i32>);
    ///1
    fn get_max(&self) -> Option<i32>;
    ///2
    fn set_max(&mut self, v : Option<i32>);

}
///1
pub trait ObjectDataSchema : DataSchema {
    ///1
    fn get_properties(&self) -> Option<BTreeMap<String, Box<dyn DataSchema>>>;
    ///1
    fn set_properties(&mut self, v : Option<BTreeMap<String, Box<dyn DataSchema>>>);
    ///1
    fn clear_properties(&mut self);
    ///1
    fn get_required_list(&self) -> Vec<String>;
    ///1
    fn add_required(&mut self, v: String);
    ///1
    fn remove_required(&mut self, i : i32);
    ///1
    fn clear_required(&mut self);
    
}
*/
impl JSonObject for BaseDataSchema {
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut ret  = serde_json::Map::new();

        self.desc_data.copy("".to_string(), &mut ret);
        self.write_only.copy("writeOnly".to_string(), &mut ret);
        self.read_only.copy("readOnly".to_string(), &mut ret);
        self.jstype.copy("type".to_string(),&mut ret);
        self.one_of.copy("oneOf".to_string(),&mut ret);
        self.format.copy("format".to_string(), &mut ret);
        self.unit.copy("unit".to_string(), &mut ret);

        ret
    }

}
impl DataSchema for BaseDataSchema {
    ///1
    fn get_description(&self) -> Option<String> {
        self.desc_data.description
    }
    ///1
    fn set_description(&mut self, v : Option<String>) {
        self.desc_data.description = v.clone();
    }

    ///1
    fn get_title(&self) -> Option<String> {
        self.desc_data.title
    }
    ///1
    fn set_title(&mut self, v : Option<String>) {
        self.desc_data.title = v.clone();
    }
    ///1
    fn get_i18n_title(&self, k: String) -> Option<&String> {
        self.desc_data.titles.get(&k)
    }
    ///1
    fn set_i18n_title(&mut self, k : String , v: Option<String>) {
        match v {
            None => self.desc_data.titles.remove(&k),
            Some(x) => self.desc_data.titles.insert(k,x)
        };
        
    }
    ///1
    fn get_i18n_description(&self, k: String) -> Option<&String> {
        self.desc_data.descriptions.get(&k)
    }
    ///1
    fn set_i18n_description(&mut self, k : String , v: Option<String>) {
        match v {
            None => self.desc_data.descriptions.remove(&k),
            Some(x) => self.desc_data.descriptions.insert(k,x)
        };

    }
    ///1 
    fn get_type(&self) -> &W3CList<String> {
        &self.desc_data.stype
    }
    ///1
    fn set_type(&mut self, v : &W3CList<String>) {
        self.desc_data.stype = v.clone();
    }
    ///1
    fn get_schema_type(&self) -> Option<DataSchemaId> {
        self.jstype
    }
    ///1
    fn get_unit(&self) -> Option<String> {
        self.unit
    }
    ///1
    fn set_unit(&mut self , v : Option<String>) {
        self.unit = v.clone();
    }
    ///1
    fn add_oneof(&mut self, v: Box<dyn DataSchema>) {
        self.one_of.push(v);
    }
    ///1
    fn get_oneof_list(&self) -> Vec<Box<dyn DataSchema>> {
        self.one_of
    }
    ///1
    fn remove_oneof(&self, i : i32) {

    }
    ///1
    fn get_format(&self) -> Option<String> {
        self.format
    }
    ///1
    fn set_format(&mut self , v : Option<String>) {
        self.format = v.clone();
    }
    ///1
    fn get_readonly(&self) -> Option<bool> {
        self.read_only
    }
    ///1
    fn set_readonly(&mut self, v : Option<bool>) {
        self.read_only = v.clone();
    }
    ///1
    fn get_writeonly(&self) ->Option< bool> {
        self.write_only
    }
    ///1
    fn set_writeonly(&mut self, v : Option<bool>) {
        self.write_only = v.clone();
    }

}




//Start of base data schema impl
#[derive(Debug)]
pub (crate) struct BaseDataSchema {
    desc_data : descriptive_data::DescriptiveData,
    write_only : Option<bool>,
    read_only : Option<bool>,
    jstype : Option<DataSchemaId>,
    one_of :  Vec<Box<dyn DataSchema>>,
    format : Option<String>,
    unit : Option<String>,
}

impl BaseDataSchema {
    pub fn new(id : Option<DataSchemaId>) -> Self {
        Self {
            write_only : None,
            read_only : None,
            jstype : id,
            one_of : Vec::new(),
            format : None,
            unit : None,
            desc_data : descriptive_data::DescriptiveData::new()
        } 
    }
}




pub struct DataSchemaFactory {
}

impl DataSchemaFactory {
    pub fn new(id : Option<DataSchemaId> )  -> Box<dyn DataSchema> {
        if id.is_some() == false  {
            return Box::new(BaseDataSchema::new(None));
        } else {
            let idid = id.unwrap();
            match idid {
                DataSchemaId::DSIBoolean  => Box::new(BaseDataSchema::new(Some(idid))),
                DataSchemaId::DSIString => Box::new(BaseDataSchema::new(Some(idid))),
                DataSchemaId::DSINull  => Box::new(BaseDataSchema::new(Some(idid))),


            }
        }
    }
}