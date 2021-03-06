// The MIT License (MIT)

// Copyright (c) 2014 Y. T. CHUNG <zonyitoo@gmail.com>

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::rc::Rc;

use relay::loadbalancing::server::LoadBalancer;
use config::{Config, ServerConfig};

#[derive(Clone)]
pub struct RoundRobin {
    servers: Vec<Rc<ServerConfig>>,
    index: usize,
}

impl RoundRobin {
    pub fn new(config: &Config) -> RoundRobin {
        RoundRobin {
            servers: config.server.iter().map(|s| Rc::new(s.clone())).collect(),
            index: 0usize,
        }
    }
}

impl LoadBalancer for RoundRobin {
    fn pick_server(&mut self) -> Rc<ServerConfig> {
        let server = &self.servers;

        if server.is_empty() {
            panic!("No server");
        }

        let ref s = server[self.index];
        self.index = (self.index + 1) % server.len();
        s.clone()
    }

    fn total(&self) -> usize {
        self.servers.len()
    }
}
