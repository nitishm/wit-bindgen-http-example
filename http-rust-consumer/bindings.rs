pub mod httpexp {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
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
  pub type BodyParam<'a,> = &'a [u8];
  /// The HTTP body.
  /// Currently, this is a synchonous byte array, but it should be
  /// possible to have a stream for both request and response bodies.
  pub type BodyResult = Vec<u8>;
  /// The HTTP headers represented as a list of (name, value) pairs.
  pub type HeadersParam<'a,> = &'a [(&'a  str,&'a  str,)];
  /// The HTTP headers represented as a list of (name, value) pairs.
  pub type HeadersResult = Vec<(String,String,)>;
  /// The HTTP parameter queries, represented as a list of (name, value) pairs.
  pub type Params<'a,> = &'a [(&'a  str,&'a  str,)];
  /// The HTTP URI of the current request.
  pub type Uri<'a,> = &'a  str;
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
  pub struct Request<'a,> {
    pub method: Method,
    pub uri: Uri<'a,>,
    pub headers: HeadersParam<'a,>,
    pub params: Params<'a,>,
    pub body: Option<BodyParam<'a,>>,
  }
  impl<'a,> std::fmt::Debug for Request<'a,> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Request").field("method", &self.method).field("uri", &self.uri).field("headers", &self.headers).field("params", &self.params).field("body", &self.body).finish()}
  }
  /// An HTTP response.
  #[derive(Clone)]
  pub struct Response {
    pub status: HttpStatus,
    pub headers: Option<HeadersResult>,
    pub body: Option<BodyResult>,
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
  
  /// Auxiliary data associated with the wasm exports.
  ///
  /// This is required to be stored within the data of a
  /// `Store<T>` itself so lifting/lowering state can be managed
  /// when translating between the host and wasm.
  #[derive(Default)]
  pub struct HttpexpData {
  }
  pub struct Httpexp<T> {
    get_state: Box<dyn Fn(&mut T) -> &mut HttpexpData + Send + Sync>,
    canonical_abi_free: wasmtime::TypedFunc<(i32, i32, i32), ()>,
    canonical_abi_realloc: wasmtime::TypedFunc<(i32, i32, i32, i32), i32>,
    memory: wasmtime::Memory,
    request: wasmtime::TypedFunc<(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,), (i32,)>,
  }
  impl<T> Httpexp<T> {
    #[allow(unused_variables)]
    
    /// Adds any intrinsics, if necessary for this exported wasm
    /// functionality to the `linker` provided.
    ///
    /// The `get_state` closure is required to access the
    /// auxiliary data necessary for these wasm exports from
    /// the general store's state.
    pub fn add_to_linker(
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut HttpexpData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()> {
      Ok(())
    }
    
    /// Instantaites the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    ///
    /// The `linker` provided will have intrinsics added to it
    /// automatically, so it's not necessary to call
    /// `add_to_linker` beforehand. This function will
    /// instantiate the `module` otherwise using `linker`, and
    /// both an instance of this structure and the underlying
    /// `wasmtime::Instance` will be returned.
    ///
    /// The `get_state` parameter is used to access the
    /// auxiliary state necessary for these wasm exports from
    /// the general store state `T`.
    pub fn instantiate(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    module: &wasmtime::Module,
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut HttpexpData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<(Self, wasmtime::Instance)> {
      Self::add_to_linker(linker, get_state)?;
      let instance = linker.instantiate(&mut store, module)?;
      Ok((Self::new(store, &instance,get_state)?, instance))
    }
    
    /// Low-level creation wrapper for wrapping up the exports
    /// of the `instance` provided in this structure of wasm
    /// exports.
    ///
    /// This function will extract exports from the `instance`
    /// defined within `store` and wrap them all up in the
    /// returned structure which can be used to interact with
    /// the wasm module.
    pub fn new(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    instance: &wasmtime::Instance,
    get_state: impl Fn(&mut T) -> &mut HttpexpData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<Self> {
      let mut store = store.as_context_mut();
      let canonical_abi_free= instance.get_typed_func::<(i32, i32, i32), (), _>(&mut store, "canonical_abi_free")?;
      let canonical_abi_realloc= instance.get_typed_func::<(i32, i32, i32, i32), i32, _>(&mut store, "canonical_abi_realloc")?;
      let memory= instance
      .get_memory(&mut store, "memory")
      .ok_or_else(|| {
        anyhow::anyhow!("`memory` export not a memory")
      })?
      ;
      let request= instance.get_typed_func::<(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,), (i32,), _>(&mut store, "request")?;
      Ok(Httpexp{
        canonical_abi_free,
        canonical_abi_realloc,
        memory,
        request,
        get_state: Box::new(get_state),
        
      })
    }
    /// Send an HTTP request and return a response or a potential error.
    pub fn request(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,req: Request<'_,>,)-> Result<Result<Response,HttpError>, wasmtime::Trap> {
      let func_canonical_abi_realloc = &self.canonical_abi_realloc;
      let func_canonical_abi_free = &self.canonical_abi_free;
      let memory = &self.memory;
      let Request{ method:method0, uri:uri0, headers:headers0, params:params0, body:body0, } = req;
      let vec1 = uri0;
      let ptr1 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec1.len() as i32) * 1))?;
      memory.data_mut(&mut caller).store_many(ptr1, vec1.as_ref())?;
      let vec5 = headers0;
      let len5 = vec5.len() as i32;
      let result5 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 4, len5 * 16))?;
      for (i, e) in vec5.into_iter().enumerate() {
        let base = result5 + (i as i32) * 16;
        {
          let (t2_0, t2_1, ) = e;
          let vec3 = t2_0;
          let ptr3 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec3.len() as i32) * 1))?;
          memory.data_mut(&mut caller).store_many(ptr3, vec3.as_ref())?;
          memory.data_mut(&mut caller).store(base + 4, wit_bindgen_wasmtime::rt::as_i32(vec3.len() as i32))?;
          memory.data_mut(&mut caller).store(base + 0, wit_bindgen_wasmtime::rt::as_i32(ptr3))?;
          let vec4 = t2_1;
          let ptr4 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec4.len() as i32) * 1))?;
          memory.data_mut(&mut caller).store_many(ptr4, vec4.as_ref())?;
          memory.data_mut(&mut caller).store(base + 12, wit_bindgen_wasmtime::rt::as_i32(vec4.len() as i32))?;
          memory.data_mut(&mut caller).store(base + 8, wit_bindgen_wasmtime::rt::as_i32(ptr4))?;
        }}let vec9 = params0;
        let len9 = vec9.len() as i32;
        let result9 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 4, len9 * 16))?;
        for (i, e) in vec9.into_iter().enumerate() {
          let base = result9 + (i as i32) * 16;
          {
            let (t6_0, t6_1, ) = e;
            let vec7 = t6_0;
            let ptr7 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec7.len() as i32) * 1))?;
            memory.data_mut(&mut caller).store_many(ptr7, vec7.as_ref())?;
            memory.data_mut(&mut caller).store(base + 4, wit_bindgen_wasmtime::rt::as_i32(vec7.len() as i32))?;
            memory.data_mut(&mut caller).store(base + 0, wit_bindgen_wasmtime::rt::as_i32(ptr7))?;
            let vec8 = t6_1;
            let ptr8 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec8.len() as i32) * 1))?;
            memory.data_mut(&mut caller).store_many(ptr8, vec8.as_ref())?;
            memory.data_mut(&mut caller).store(base + 12, wit_bindgen_wasmtime::rt::as_i32(vec8.len() as i32))?;
            memory.data_mut(&mut caller).store(base + 8, wit_bindgen_wasmtime::rt::as_i32(ptr8))?;
          }}let (result11_0,result11_1,result11_2,) = match body0{
            None => { (0i32, 0i32, 0i32)}
            Some(e) => { {
              let vec10 = e;
              let ptr10 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec10.len() as i32) * 1))?;
              memory.data_mut(&mut caller).store_many(ptr10, vec10.as_ref())?;
              (1i32, ptr10, vec10.len() as i32)
            }}
          };
          let (result12_0,) = self.request.call(&mut caller, (method0 as i32, ptr1, vec1.len() as i32, result5, len5, result9, len9, result11_0, result11_1, result11_2, ))?;
          let load13 = memory.data_mut(&mut caller).load::<i32>(result12_0 + 0)?;
          let load14 = memory.data_mut(&mut caller).load::<i32>(result12_0 + 8)?;
          let load15 = memory.data_mut(&mut caller).load::<i32>(result12_0 + 16)?;
          let load16 = memory.data_mut(&mut caller).load::<i32>(result12_0 + 24)?;
          let load17 = memory.data_mut(&mut caller).load::<i32>(result12_0 + 32)?;
          let load18 = memory.data_mut(&mut caller).load::<i32>(result12_0 + 40)?;
          let load19 = memory.data_mut(&mut caller).load::<i32>(result12_0 + 48)?;
          let load20 = memory.data_mut(&mut caller).load::<i32>(result12_0 + 56)?;
          Ok(match load13 {
            0 => Ok(Response{status:u16::try_from(load14).map_err(bad_int)?, headers:match load15 {
              0 => None,
              1 => Some({
                let len27 = load17;
                let base27 = load16;
                let mut result27 = Vec::with_capacity(len27 as usize);
                for i in 0..len27 {
                  let base = base27 + i *16;
                  result27.push({
                    let load21 = memory.data_mut(&mut caller).load::<i32>(base + 0)?;
                    let load22 = memory.data_mut(&mut caller).load::<i32>(base + 4)?;
                    let ptr23 = load21;
                    let len23 = load22;
                    let load24 = memory.data_mut(&mut caller).load::<i32>(base + 8)?;
                    let load25 = memory.data_mut(&mut caller).load::<i32>(base + 12)?;
                    let ptr26 = load24;
                    let len26 = load25;
                    (String::from_utf8(
                    copy_slice(
                    &mut caller,
                    memory,
                    func_canonical_abi_free,
                    ptr23, len23, 1
                    )?
                    )
                    .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?, String::from_utf8(
                    copy_slice(
                    &mut caller,
                    memory,
                    func_canonical_abi_free,
                    ptr26, len26, 1
                    )?
                    )
                    .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?)
                  });
                }
                func_canonical_abi_free.call(&mut caller, (base27, len27 * 16, 4))?;
                result27
              }),
              _ => return Err(invalid_variant("Option")),
            }, body:match load18 {
              0 => None,
              1 => Some({
                let ptr28 = load19;
                let len28 = load20;
                
                copy_slice(
                &mut caller,
                memory,
                func_canonical_abi_free,
                ptr28, len28, 1
                )?
                
              }),
              _ => return Err(invalid_variant("Option")),
            }, }),
            1 => Err(match load14 {
              0 => HttpError::Success,
              1 => HttpError::DestinationNotAllowed,
              2 => HttpError::InvalidUrl,
              3 => HttpError::RequestError,
              4 => HttpError::RuntimeError,
              _ => return Err(invalid_variant("HttpError")),
            }),
            _ => return Err(invalid_variant("Result")),
          })
        }
      }
      use wit_bindgen_wasmtime::rt::RawMem;
      use wit_bindgen_wasmtime::rt::invalid_variant;
      use core::convert::TryFrom;
      use wit_bindgen_wasmtime::rt::bad_int;
      use wit_bindgen_wasmtime::rt::copy_slice;
    }
    