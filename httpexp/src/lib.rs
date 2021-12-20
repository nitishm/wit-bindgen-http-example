wit_bindgen_rust::export!("../httpexp.wit");

struct Httpexp {}
impl httpexp::Httpexp for Httpexp {
    fn request(_req:httpexp::Request) ->Result<httpexp::Response,httpexp::HttpError> {
        println!("I have been invoked");
      
        Ok(httpexp::Response{
            status: 200,
            headers: None,
            body: None
        })
    }
}