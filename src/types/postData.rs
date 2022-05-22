pub struct PostData<'a> {
    pub(crate) context: serde_json::Value,
    pub(crate) continuation: &'a str,
}


impl PostData<'_> {

    fn name(self){
        return; 
    }
}