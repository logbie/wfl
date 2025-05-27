
#[cfg(feature = "lsp-bridge")]
use wfl_lsp::WflLanguageServer;

#[cfg(feature = "lsp-bridge")]
pub struct Server {
    server: WflLanguageServer,
}

#[cfg(feature = "lsp-bridge")]
impl Server {
    pub fn new() -> Self {
        Self {
            server: WflLanguageServer::new(),
        }
    }
    
    pub fn start(&self) {
    }
}

#[cfg(not(feature = "lsp-bridge"))]
pub struct Server;

#[cfg(not(feature = "lsp-bridge"))]
impl Server {
    pub fn new() -> Self {
        Self
    }
    
    pub fn start(&self) {
    }
}
