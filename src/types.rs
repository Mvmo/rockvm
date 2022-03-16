use std::any::Any;

pub type TypeName = &'static str;

pub trait RockType {
    fn new(name: String) -> Self;
    fn name(&self) -> &str;
}
    
pub struct PrimitiveType {
    pub name: String
}
    
impl RockType for PrimitiveType {
    fn new(name: String) -> Self {
        PrimitiveType { name }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}
    
pub struct ComplexType {
    pub name: String,
    pub fields: Vec<Field>
}
    
impl RockType for ComplexType {
    fn new(name: String) -> Self {
        ComplexType { name: name, fields: Vec::new() }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}
    
pub struct Field {
    pub name: String,
    pub type_name: TypeName
}
    
impl Field {
    pub fn new(name: String, type_name: TypeName) -> Self {
        Field { name, type_name }
    }
}
    
pub trait RockInstance<T: RockType> {
    fn _type(&self) -> &T;
}
    
pub struct PrimitiveInstance {
    pub _type: &'static PrimitiveType,
    pub value: dyn Any
}
    
    /*const INT_PRIMITIVE: PrimitiveType = PrimitiveType::new("int".to_string());
    const BOOL_PRIMITIVE: PrimitiveType = PrimitiveType::new("bool".to_string());
    const STRING_PRIMITIVE: PrimitiveType = PrimitiveType::new("string".to_string());
    */
    
impl RockInstance<PrimitiveType> for PrimitiveInstance {
    fn _type(&self) -> &PrimitiveType {
        self._type 
    }
}
    
pub struct ComplexInstance {
    pub _type: &'static ComplexType,
    pub values: Vec<&'static dyn RockInstance<dyn Any>>
}
    
impl RockInstance<ComplexType> for ComplexInstance {
    fn _type(&self) -> &ComplexType {
        self._type
    }
}