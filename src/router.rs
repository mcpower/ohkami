use crate::{
    components::method::Method, utils::range::RangeList, result::Result, handler::HandleFunc, setting::{Middleware, MiddlewareFunc},
};

// === mock for test ===
// pub(self) type Handler = usize;
// =====================

mod pattern;
mod node; use node::Node;

use self::pattern::Pattern;

mod test_resister;
mod test_search;


// #[derive(PartialEq, Debug)]
#[allow(non_snake_case)]
pub(crate) struct Router {
    root_middleware_proc: Vec<MiddlewareFunc>,
    GET:    Node,
    POST:   Node,
    PATCH:  Node,
    DELETE: Node,
}
impl Router {
    pub(crate) fn new() -> Self {
        Self {
            root_middleware_proc: Vec::new(),
            GET:    Node::new(Pattern::Nil),
            POST:   Node::new(Pattern::Nil),
            PATCH:  Node::new(Pattern::Nil),
            DELETE: Node::new(Pattern::Nil),
        }
    }

    pub(crate) fn register(&mut self,
        method:       Method,
        path_pattern: &'static str /* already validated */,
        handler:      HandleFunc,
    ) -> std::result::Result<(), String> {
        let err_msg = format!(
            "path pattern `{}` is resistred duplicatedly",
            if path_pattern == "" {"/"} else {path_pattern}
        );

        let mut path = path_pattern.split('/');
        { path.next(); }

        let tree = match method {
            Method::GET    => &mut self.GET,
            Method::POST   => &mut self.POST,
            Method::PATCH  => &mut self.PATCH,
            Method::DELETE => &mut self.DELETE,
        };
        
        tree.register_handler(path, handler, err_msg)
    }
    pub(crate) fn search<'req>(&self,
        method:       Method,
        request_path: &'req str,
    ) -> Result<(
        &HandleFunc,
        RangeList,
        Vec<&MiddlewareFunc>,
        Option<&MiddlewareFunc>,
    )> {
        let mut path = request_path.split('/');
        { path.next(); }

        let offset = method.len();

        let tree = match method {
            Method::GET    => &self.GET,
            Method::POST   => &self.POST,
            Method::PATCH  => &self.PATCH,
            Method::DELETE => &self.DELETE,
        };

        let mut middleware_proccess = Vec::new();
        for proc in &self.root_middleware_proc {
            middleware_proccess.push(proc)
        }

        tree.search(path, RangeList::new(), offset, middleware_proccess)
    }

    pub(crate) fn apply(mut self, middlware: Middleware) -> std::result::Result<Self, String> {
        if ! middlware.setup_errors.is_empty() {
            return Err(
                middlware.setup_errors
                    .into_iter()
                    .fold(String::new(), |it, next| it + &next + "\n")
            )
        }

        for (method, route, func) in middlware.proccess {
            let error_msg = format!("middleware func just for `{method} {route}` is registered duplicatedly");

            if route == "*" {
                self.root_middleware_proc.push(func)
            } else {
                match method {
                    Method::GET    => self.GET = self.GET.register_middleware_func(route, func, error_msg)?,
                    Method::POST   => self.POST = self.POST.register_middleware_func(route, func, error_msg)?,
                    Method::PATCH  => self.PATCH = self.PATCH.register_middleware_func(route, func, error_msg)?,
                    Method::DELETE => self.DELETE = self.DELETE.register_middleware_func(route, func, error_msg)?,
                }
            }
        }

        Ok(self)
    }
}
