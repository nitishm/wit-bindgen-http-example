mod httpexp {
  /// This is a temporary workaround very similar to https://github.com/deislabs/wasi-experimental-http.
  /// Once asynchronous functions, streams, and the upstream HTTP API are available, this should be removed.
  /// The HTTP status code.
  /// This is currently an unsigned 16-bit integer,
  /// but it could be represented as an enum containing
  /// all possible HTTP status codes.
  pub type HttpStatus = u16;
  /// The HTTP body.
  /// Currently, this is a synchonous byte array, but it should be
  /// possible to have a stream for both request and response bodies.
  pub type Body = Vec<u8>;
  /// The HTTP headers represented as a list of (name, value) pairs.
  pub type Headers = Vec<(String,String,)>;
  /// The HTTP parameter queries, represented as a list of (name, value) pairs.
  pub type Params = Vec<(String,String,)>;
  /// The HTTP URI of the current request.
  pub type Uri = String;
  /// The HTTP method.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum Method{
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
  }
  impl std::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Method::Get => {
          f.debug_tuple("Method::Get").finish()
        }
        Method::Post => {
          f.debug_tuple("Method::Post").finish()
        }
        Method::Put => {
          f.debug_tuple("Method::Put").finish()
        }
        Method::Delete => {
          f.debug_tuple("Method::Delete").finish()
        }
        Method::Patch => {
          f.debug_tuple("Method::Patch").finish()
        }
        Method::Head => {
          f.debug_tuple("Method::Head").finish()
        }
        Method::Options => {
          f.debug_tuple("Method::Options").finish()
        }
      }
    }
  }
  /// An HTTP request.
  #[derive(Clone)]
  pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: Headers,
    pub params: Params,
    pub body: Option<Body>,
  }
  impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Request").field("method", &self.method).field("uri", &self.uri).field("headers", &self.headers).field("params", &self.params).field("body", &self.body).finish()}
  }
  /// An HTTP response.
  #[derive(Clone)]
  pub struct Response {
    pub status: HttpStatus,
    pub headers: Option<Headers>,
    pub body: Option<Body>,
  }
  impl std::fmt::Debug for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Response").field("status", &self.status).field("headers", &self.headers).field("body", &self.body).finish()}
  }
  /// HTTP errors returned by the runtime.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum HttpError{
    Success,
    DestinationNotAllowed,
    InvalidUrl,
    RequestError,
    RuntimeError,
  }
  impl std::fmt::Debug for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        HttpError::Success => {
          f.debug_tuple("HttpError::Success").finish()
        }
        HttpError::DestinationNotAllowed => {
          f.debug_tuple("HttpError::DestinationNotAllowed").finish()
        }
        HttpError::InvalidUrl => {
          f.debug_tuple("HttpError::InvalidUrl").finish()
        }
        HttpError::RequestError => {
          f.debug_tuple("HttpError::RequestError").finish()
        }
        HttpError::RuntimeError => {
          f.debug_tuple("HttpError::RuntimeError").finish()
        }
      }
    }
  }
  #[export_name = "request"]
  unsafe extern "C" fn __wit_bindgen_request(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: i32, ) -> i32{
    let len0 = arg2 as usize;
    let base3 = arg3;
    let len3 = arg4;
    let mut result3 = Vec::with_capacity(len3 as usize);
    for i in 0..len3 {
      let base = base3 + i *16;
      result3.push({
        let len1 = *((base + 4) as *const i32) as usize;
        let len2 = *((base + 12) as *const i32) as usize;
        
        (String::from_utf8(Vec::from_raw_parts(*((base + 0) as *const i32) as *mut _, len1, len1)).unwrap(), String::from_utf8(Vec::from_raw_parts(*((base + 8) as *const i32) as *mut _, len2, len2)).unwrap())
      });
    }
    std::alloc::dealloc(
    base3 as *mut _,
    std::alloc::Layout::from_size_align_unchecked(
    (len3 as usize) * 16,
    4,
    ),
    );
    let base6 = arg5;
    let len6 = arg6;
    let mut result6 = Vec::with_capacity(len6 as usize);
    for i in 0..len6 {
      let base = base6 + i *16;
      result6.push({
        let len4 = *((base + 4) as *const i32) as usize;
        let len5 = *((base + 12) as *const i32) as usize;
        
        (String::from_utf8(Vec::from_raw_parts(*((base + 0) as *const i32) as *mut _, len4, len4)).unwrap(), String::from_utf8(Vec::from_raw_parts(*((base + 8) as *const i32) as *mut _, len5, len5)).unwrap())
      });
    }
    std::alloc::dealloc(
    base6 as *mut _,
    std::alloc::Layout::from_size_align_unchecked(
    (len6 as usize) * 16,
    4,
    ),
    );
    let result8 = <super::Httpexp as Httpexp>::request(Request{method:match arg0 {
      0 => Method::Get,
      1 => Method::Post,
      2 => Method::Put,
      3 => Method::Delete,
      4 => Method::Patch,
      5 => Method::Head,
      6 => Method::Options,
      _ => panic!("invalid enum discriminant"),
    }, uri:String::from_utf8(Vec::from_raw_parts(arg1 as *mut _, len0, len0)).unwrap(), headers:result3, params:result6, body:match arg7 {
      0 => None,
      1 => Some({
        let len7 = arg9 as usize;
        
        Vec::from_raw_parts(arg8 as *mut _, len7, len7)
      }),
      _ => panic!("invalid enum discriminant"),
    }, });
    let (result17_0,result17_1,result17_2,result17_3,result17_4,result17_5,result17_6,result17_7,) = match result8{
      Ok(e) => { {
        let Response{ status:status9, headers:headers9, body:body9, } = e;
        let (result14_0,result14_1,result14_2,) = match headers9{
          None => { (0i32, 0i32, 0i32)}
          Some(e) => { {
            let vec13 = e;
            let len13 = vec13.len() as i32;
            let layout13 = core::alloc::Layout::from_size_align_unchecked(vec13.len() * 16, 4);
            let result13 = std::alloc::alloc(layout13);
            if result13.is_null() { std::alloc::handle_alloc_error(layout13); }
            for (i, e) in vec13.into_iter().enumerate() {
              let base = result13 as i32 + (i as i32) * 16;
              {
                let (t10_0, t10_1, ) = e;
                let vec11 = (t10_0.into_bytes()).into_boxed_slice();
                let ptr11 = vec11.as_ptr() as i32;
                let len11 = vec11.len() as i32;
                core::mem::forget(vec11);
                *((base + 4) as *mut i32) = len11;
                *((base + 0) as *mut i32) = ptr11;
                let vec12 = (t10_1.into_bytes()).into_boxed_slice();
                let ptr12 = vec12.as_ptr() as i32;
                let len12 = vec12.len() as i32;
                core::mem::forget(vec12);
                *((base + 12) as *mut i32) = len12;
                *((base + 8) as *mut i32) = ptr12;
                
              }}
              
              (1i32, result13 as i32, len13)
            }}
          };
          let (result16_0,result16_1,result16_2,) = match body9{
            None => { (0i32, 0i32, 0i32)}
            Some(e) => { {
              let vec15 = (e).into_boxed_slice();
              let ptr15 = vec15.as_ptr() as i32;
              let len15 = vec15.len() as i32;
              core::mem::forget(vec15);
              
              (1i32, ptr15, len15)
            }}
          };
          
          (0i32, wit_bindgen_rust::rt::as_i32(status9), result14_0, result14_1, result14_2, result16_0, result16_1, result16_2)
        }}
        Err(e) => { (1i32, e as i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32)}
      };
      let ptr18 = RET_AREA.as_mut_ptr() as i32;
      *((ptr18 + 56) as *mut i32) = result17_7;
      *((ptr18 + 48) as *mut i32) = result17_6;
      *((ptr18 + 40) as *mut i32) = result17_5;
      *((ptr18 + 32) as *mut i32) = result17_4;
      *((ptr18 + 24) as *mut i32) = result17_3;
      *((ptr18 + 16) as *mut i32) = result17_2;
      *((ptr18 + 8) as *mut i32) = result17_1;
      *((ptr18 + 0) as *mut i32) = result17_0;
      ptr18
    }
    pub trait Httpexp {
      /// Send an HTTP request and return a response or a potential error.
      fn request(req: Request,) -> Result<Response,HttpError>;
    }
    static mut RET_AREA: [i64; 8] = [0; 8];
  }
  