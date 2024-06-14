use crate::runtime_module::RuntimeCallable;

#[derive(Clone)]
pub struct Frame {
    function: RuntimeCallable,
    ip: Option<usize>,
}

impl Frame {
    pub(crate) fn new(f: &RuntimeCallable) -> Self {
        Self {
            function: f.clone(),
            ip: None,
        }
    }

    pub(crate) fn set_ip(&mut self, ip: usize) {
        self.ip = Some(ip)
    }

    pub fn get_function(&self) -> RuntimeCallable {
        self.function.clone()
    }

    pub fn get_ip(&self) -> Option<usize> {
        self.ip
    }
}

impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ip) = self.ip {
            write!(f, "{}:{}", self.function.fullname(), ip)
        } else {
            write!(f, "{}", self.function.fullname())
        }
    }
}
